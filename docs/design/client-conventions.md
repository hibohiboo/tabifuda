# クライアント層の表示・操作規約

対象: CLI/WASM/Web各層。decide/applyの規範(何が起きるか)は
domain-model.mdが正であり、本文書はそれを**どう見せる/どう操作させるか**の
決定を集約する(非規範。core の decide/apply には影響しない)。

置き場の判断(P3 C0決定): wasm境界そのものの型設計・API仕様は
[wasm-boundary.md](wasm-boundary.md) に分離する。本文書は複数のクライアント
実装(CLI/Web)にまたがりうる「表示・操作ロジックの決定」を集める。

## 手札表示からの Marker 除外

`CardKind::Marker` は世界の状態・選択の成立を示す印であり
(domain-guide.md「世界はすべてカード」)、プレイヤーが選ぶ対象ではない。
そのため **クライアントの手札表示からは `CardKind::Marker` を除外する**。

- `session.hands` のデータ自体は変更しない(除外は表示層のみの決定。
  `Condition::HasCard` の判定にも影響しない)
- CLI(tabifuda-cli)は本規約に従って実装済み
- Web版UIもC2以降でこの規約に従う
