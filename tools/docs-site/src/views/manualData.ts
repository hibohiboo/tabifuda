import type { CardKind } from "@tabifuda/ui";

// マニュアルタブの表示用データ。domain-model.mdの表をそのまま複製せず、
// マニュアル向けに書き起こした独自要約+出典リンクを持つ(docs/rdra/README.md
// 「description は1〜2行の要約に留め、規範の内容を複製しない」と同じ位置づけ)。
// 規範文書と食い違ったらこちら側を直す(正は出典リンク先のdesign/文書)。

export interface CardKindManualEntry {
  origin: string;
  description: string;
}

// Record<CardKind, ...>で網羅させる(client-conventions.md「CardKindのような
// 単純なリテラル合併型を網羅する場合はRecord<Kind, ...>を使う」)。CardKindに
// 新しい種別が増えるとここが型エラーになり追記漏れを防げる。
export const CARD_KIND_MANUAL: Record<CardKind, CardKindManualEntry> = {
  Action: { origin: "キャラメイク", description: "キャラ固有の技能・行動。" },
  Scenario: { origin: "シーン配布", description: "シナリオが状況に応じて配る選択肢。" },
  Dialogue: { origin: "常時/配布", description: "台詞カード。自由入力テキストを添えて出せる。" },
  Proposal: { origin: "常時", description: "新たな選択肢の提案。GMが採否を裁定する。" },
  Item: { origin: "配布/取得", description: "所持品。効果を持つことがある。" },
  Marker: {
    origin: "内部(配布/取得)",
    description:
      "世界の状態・選択の成立を示す印。#portableを持っていても常に持ち出し不可で、手札表示からも除外される。",
  },
};

// 現時点ではCardKindの1項目のみ。他の用語(Effect種別等)が必要になったら
// 都度追加する(汎用フレームワークの先回り実装はしない)。
export const CARD_KIND_MANUAL_INTRO =
  "世界のすべてはカードで表現される。「フラグ管理」のような見えない内部変数は持たず、" +
  "世界の状態・選択の成立もカード(Markerカード)で表現する。種別ごとに出所と役割が違う。";

export const CARD_KIND_MANUAL_SOURCE = "design/domain-model.md#カード";
