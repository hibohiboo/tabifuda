# wasm境界API設計 決定ログ

対象: P3 C1(crates/tabifuda-wasm)。Opus 4.8による設計レビュー
(2026-08-01)で挙がった論点のうち、C1スコープ外だが人間の判断が要るもの。

## 状態

| # | 論点 | 状態 |
|---|---|---|
| 1 | Session内HashMap → BTreeMap化 | 済(P3.5 C1、2026-08-01) |

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

**決定**: B(BTreeMap化する)。P3.5がまさにセーブ機能を作るサイクルであり、
今のうちに直すのが自然なため。

**反映先**: docs/design/domain-model.md「コレクションとidの規則」表・
「セッション状態」節、crates/tabifuda-core/src/{ids,session,character,event,engine}.rs
を更新済み(P3.5 C1)。
