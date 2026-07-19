# シナリオlint(シナリオデータの静的検証)

**位置づけ**: 規範文書。[domain-model.md](domain-model.md) から分離した
(経緯: reviews/docs-structure-review.md H2)。テスト戦略上の位置づけは
[test-strategy.md](test-strategy.md) §2「シナリオデータ」。
実装は `tabifuda-core::lint` と `tabifuda-cli lint` サブコマンド。

## 置き場所と純粋性

将来の`scenario-validate`スキルと実装を共有する前提のため、CLI固有にせず
`tabifuda-core::lint` に置く(coreは状態機械の実行だけでなく「`Scenario`値を
検証する純粋関数」も純粋性の条件(IO・時刻・乱数・グローバル状態なし)を
満たすため、置いてよいと判断した)。ファイル読み込みは常にIO層
(tabifuda-cli等)が担い、lintは`Scenario`値を受け取る純粋関数として実装する。

```rust
pub fn lint(scenario: &Scenario) -> Vec<LintFinding>;

pub struct LintFinding {
    pub severity: Severity,  // Error | Warning
    pub issue: LintIssue,    // #[non_exhaustive]
}
```

## 検査項目と重大度

(test-strategy.md §2「参照解決/到達可能性/詰み検知」に対応)

| 区分 | 内容 | 重大度 |
|---|---|---|
| 参照解決 | `card_defs` 内 `CardId` の重複 | Error |
| 参照解決 | 全phase通して `SceneId` の重複 | Error |
| 参照解決 | `Deal.card` / `Effect::DealCard.card` / `Condition::HasCard` が指す `CardId` が `card_defs` に無い | Error |
| 参照解決 | `Transition.to` / `Effect::GotoScene` が指す `SceneId` がどのphaseにも無い | Error |
| 参照解決 | シナリオデータ内(`Deal.to` / `Effect::DealCard.to` / `Effect::ModifyStat.target`)に `Target::Character(_)` が使われている(domain-model.md「Targetの意味論」で上演中専用と規定済み。シナリオ作者データでは不正) | Error |
| 構造 | 先頭シーンが解決できない(`phases`が空、または先頭phaseに`scenes`が無い。`StartSession`が`RuleError::ScenarioHasNoScenes`で拒否する条件と同一) | Error |
| 到達可能性 | オープニング先頭シーンから到達できないシーンがある | Warning |
| 詰み検知 | そのシーンから`Effect::EndSession`を持つカードに到達する経路が無い | Warning |

Error系はlintとして「シナリオが壊れている」ことを意味し、`tabifuda-cli lint`は
Errorが1件でもあれば非ゼロ終了・テストは失敗として扱う。Warning系(到達可能性・
詰み検知)はtest-strategy.mdの「到達不能=警告」表現どおり検出のみ行い、
失敗扱いにはしない。

## 到達可能性・詰み検知の探索範囲(シーン単位の直接辺のみ)

グラフの辺は以下の2種のみとし、`Effect::DealCard`で後から配られたカードの
効果は追わない(不動点閉包は取らない)。テンプレシナリオ「単純討伐」相当の
構成(勝利/敗北カードをシーン入場時に配って選ばせる、domain-model.md
「勝敗分岐」参照)を検証するにはこのシーン単位の辺で十分であり、
閉包計算より実装がシンプルなため:
- シーン`S`の`exits[].to`(`Transition.condition`の充足可能性は見ない。
  常に辿れるとみなす楽観的判定)
- シーン`S`の`deals[].card`が指す`CardDef`の`effects`に含まれる
  `Effect::GotoScene(target)` → `S → target`

詰み検知は、到達可能な各シーンを起点に同じ辺で探索した閉包内に
`Effect::EndSession`を持つカードが配られるシーンが1つも無い場合に警告する
(到達不能シーンは既に別の警告で報告済みのため対象外)。
