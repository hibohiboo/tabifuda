# 調査メモ: gen-test-report.mjsの「Running見出し」対応がCIでのみ崩れる問題

状態: 未対応(今後の課題として保留)。発生日: 2026-07-31。

## 症状

`.github/workflows/pages.yml` の `pnpm -r typecheck` が、Ubuntu CI上でのみ
以下のエラーで落ちた(ローカルWindowsでは同じコミットで再現しない)。

```
Error: test行に対応するRunning見出しが無い(cargoの出力形式が変わった可能性):
test chronicle::tests::冒険記の描画はSessionEndedのoutcomeを表示する ... ok
```

このエラー自体は
[tools/docs-site/scripts/gen-test-report.mjs](../../../../../tools/docs-site/scripts/gen-test-report.mjs)
の `parse()` が投げるガード(未知の出力形式を検知して落とす安全弁)であり、
コミット ec9e74e「Fix gen-test-report.mjs: stdout/stderr分離パースを
OS合流の単一ストリームへ」で一度直したのと**同じ症状**が別の形で再発したもの。

## 調査結果(根本原因)

`cargo test --workspace` の出力は、実は**2つの別プロセス**が書き込んでいる:

- `   Running unittests src/main.rs (...)` … **cargo本体**がstderrに出す進捗表示
- `running N tests` / `test ... ok` … **cargoが起動した子プロセス(テストバイナリ)**が
  stdoutに出す結果

ec9e74eの修正は「stdout/stderrを別々にNode側で捕捉し、出現順をインデックスで
対応付ける」実装を「シェルの`2>&1`でOSレベルに合流させ、1本のストリームとして
出現順どおりに読む」実装に変えた。これは**インデックス対応のズレ**という
旧バグは直したが、**2つの独立したOSプロセスがそれぞれ同じパイプに書き込む際の
書き込みタイミング競合**までは解消していない。`2>&1`は両者を同じ書き込み先に
まとめるだけで、「cargoの`Running`メッセージが子プロセスの最初の出力より
物理的に先に書き込まれる」ことまでは保証しない。

親(cargo)の書き込みがわずかに遅延し、その間に子(テストバイナリ)が先に
`running N tests`/`test ... ok`を書き込んでしまえば、パーサーから見た出現順は
入れ替わる。これはプロセススケジューリングに依存する競合なので:

- Windows(手元)ではこれまでのところ毎回「Running出力が先に確定する」
  タイミングになっている(ローカル再現で確認。下記のログ抜粋を参照)
- Ubuntu CIランナー(共有vCPU・仮想化環境)はプロセス切り替えの揺らぎが
  ローカルより大きく、特に**ワークスペース中で最初に実行されるターゲット**
  (今回は`chronicle::tests`を含む tabifuda-cli の `unittests src/main.rs`。
  直前に既に確定済みの出力が無く「バッファの余裕」が無い最初のターゲットほど
  競合が表面化しやすい)で顕在化しやすい

と考えられる。非決定的な競合なので、**再実行すれば通る可能性がある**
(実際に競合が起きるかはランナーのタイミング次第)。

### ローカル再現ログ(参考。競合が起きなかった例)

```
     Running unittests src\main.rs (target\debug\deps\tabifuda_cli-....exe)

running 10 tests
test chronicle::tests::冒険記の描画はSessionEndedのoutcomeを表示する ... ok
...
```

手元では常にRunning見出しが先に確定しており、この前提の脆さが
可視化しにくかった。

## 今後の対応候補(未着手)

テキスト出力の出現順に依存する限り、この種の競合は原理的に完全には
潰せない。確実にするなら、パース方式そのものを変える必要がある:

**案: スイート(crate×target)ごとに`cargo test`を個別実行し、
実行コマンド自体からスイートを特定する。** 「Running見出しをテキストから
拾って対応付ける」処理自体を無くす。

- 長所: プロセス間の出力タイミングに一切依存しなくなり、確定的になる
- 短所: cargoの起動回数が増え`gen:test-report`の実行時間が伸びる。
  `tools/docs-site/scripts/gen-test-report.mjs` の`SUITES`定義
  (crate×target→スイートの対応)を今より詳細化する必要があり、
  実装変更の規模は小さくない
- 着手する場合は、実装前に本タスク(D2)のセクション記述
  (「stdout/stderrを出現順で対応付けてスイート単位に分類」という現在の記述)
  を新方式に合わせて更新すること(CLAUDE.md最重要ルール1)

## 現状の判断

2026-07-31時点ではユーザー判断により**修正は保留**。再発時にCI再実行で
様子を見るか、本メモの対応案に着手するかを判断する。
