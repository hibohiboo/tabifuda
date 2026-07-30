# 調査記録: gen-test-report.mjsの「Running見出しが無い」CI失敗(原因: CARGO_TERM_COLOR)

状態: **解決済み**(2026-07-31)。原因は当初診断(プロセス間の出力タイミング競合)
とは異なり、**CI環境の`CARGO_TERM_COLOR=always`によるANSI色付け**だった。
ファイル名の`race`は当初の誤診断の名残(コミット済みのため改名していない)。

## 症状

`.github/workflows/pages.yml` / `ci.yml` の `pnpm -r typecheck` が、
Ubuntu CI上でのみ以下のエラーで落ちる(ローカルWindowsでは再現しない)。

```
Error: test行に対応するRunning見出しが無い(cargoの出力形式が変わった可能性):
test chronicle::tests::冒険記の描画はSessionEndedのoutcomeを表示する ... ok
```

コミット ec9e74e「stdout/stderr分離パースをOS合流の単一ストリームへ」で
パース方式を根本から変えた後も、CIは同じ箇所で同じエラーを出し続けていた。

## 根本原因

**`dtolnay/rust-toolchain`アクションが、未設定の場合に`CARGO_TERM_COLOR=always`を
`$GITHUB_ENV`へ書き込む**(CIログのアクション実行部で確認)。これにより
CI上のcargoは非TTYのパイプ相手でも自身の進捗見出しをANSI色付きで出力する:

```
^[[1m^[[92m     Running^[[0m unittests src/main.rs (target/debug/deps/...)
```

行頭がエスケープシーケンス(`\x1b[1m\x1b[92m`)で始まるため、
[gen-test-report.mjs](../../../../../tools/docs-site/scripts/gen-test-report.mjs)の
`RUNNING_RE = /^ {2,}Running .../`が**一度もマッチせず**、最初のtest行で
「対応するRunning見出しが無い」で落ちる。一方、個々の`test ... ok`行は
テストバイナリ(libtest)が自前のTTY判定で色を付けないため素のまま出力され、
`TEST_LINE_RE`にはマッチする。この非対称が症状の形(見出しだけ消える)を説明する。

ローカルでは`CARGO_TERM_COLOR`が未設定でcargoが非TTYを検知し色を付けないため
再現しなかった。ローカルで`CARGO_TERM_COLOR=always`を設定すると、
**Windows上でもCIと同一のエラーが同一のテスト行で再現**することを確認した。

## 当初の誤診断とその棄却

当初は「cargo本体(stderr)とテストバイナリ(stdout)という別プロセスの
書き込みタイミング競合が`2>&1`合流後の出現順を崩す」と診断していたが、誤り:

1. cargoは`Running`見出しを書き込んで**から**テストバイナリを起動する。
   `2>&1`で両者が同一パイプを共有する場合、書き込みはwrite()の呼び出し順で
   並ぶため、順序の逆転は構造的に起きにくい
2. ec9e74eで実装方式を全く変えた後も**同一箇所で決定的に**失敗し続けた。
   非決定的な競合なら失敗箇所・頻度が揺れるはず
3. `gh run view <id> --log`でCIの実ログを確認したところ、失敗ステップのenvに
   `CARGO_TERM_COLOR: always`が表示されており、上記のローカル再現で確定した

教訓: 「ローカルで通りCIで落ちる」問題は、理論で説明を組み立てる前に
`gh run view --log`でCIの実ログ(特にステップのenvブロック)を見る。
同一箇所で決定的に再現する失敗は、タイミング競合ではなく環境差をまず疑う。

## 対処(実施済み)

`runCargoTest()`のspawnSyncに`env: { ...process.env, CARGO_TERM_COLOR: "never" }`を
明示し、スクリプトが自分の子プロセスの出力形式を環境に依存せず固定するようにした。
`CARGO_TERM_COLOR=always`環境下でローカル実行し、13スイート162件の
生成成功を確認済み。

検討したが採らなかった代替案:

- **スイートごとに`cargo test`を個別実行し、コマンド自体からスイートを特定する**
  (誤診断時の対応候補): 出力順への依存を消す点では最も堅牢だが、cargo起動回数が
  増え実装変更も大きい。真因が判明した今、環境変数の固定で十分
- **ワークフロー側で`CARGO_TERM_COLOR: never`を設定**: pages.ymlとci.ymlの
  両方に書く必要があり、CIログの他のcargo出力(lint-test等)の色も失われる。
  問題はスクリプトのパース都合なので、スクリプト側で閉じるのが正しい
- **パース前にANSIエスケープを除去**: 動くが対症療法。色を出させない方が
  出力全体の前提が単純になる
