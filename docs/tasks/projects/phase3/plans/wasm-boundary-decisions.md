# wasm境界API設計 決定ログ

対象: P3 C1(crates/tabifuda-wasm)。Opus 4.8による設計レビュー
(2026-08-01)で挙がった論点のうち、C1スコープ外だが人間の判断が要るもの。

## 状態

| # | 論点 | 状態 |
|---|---|---|
| 1 | Session内HashMap → BTreeMap化 | 未着手 |

## 論点1: Session内HashMap → BTreeMap化

**背景**: wasm境界はSessionをJSON文字列として毎回シリアライズして
やり取りする設計(docs/design/wasm-boundary.md)。`Session.roles`/
`hands`等の`HashMap`はキー順が非決定的なため、同じ状態でも実行ごとに
JSON文字列が変わりうる。

**選択肢**:
- A: `HashMap`のまま据え置く。ソロMVP(要素数1〜2)では実害が出にくい
- B: ID型(`UserId`/`CharacterId`)に`Ord`をderiveし、該当フィールドを
  `HashMap`→`BTreeMap`へ機械的に置換する

**影響範囲**: P3.5(永続化)でセーブファイルの差分が無意味に発生する、
将来state文字列をmemoキー・差分比較に使うと誤爆する可能性。

**決定**: 未定。P3.5着手前に判断する。

**反映先**: 決定後、docs/design/domain-model.md「セッション状態」の
該当フィールド定義、およびcrates/tabifuda-core/src/session.rsを更新。
