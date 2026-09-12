// shared/scenarios/(domain-model.md「シナリオファイルの配置」)配下の
// *.jsonをビルド時に動的検出する(1シナリオ1ファイルを維持したまま、
// 追加時の手動index更新を不要にする)。
//
// 依頼選択画面が表示する最小情報(id/title/summary)のみをzodで検証し、
// Scenario全体はts-rs生成の型へasキャストで委ねる(client-conventions.md
// 「シナリオの動的検出とランタイム検証」参照。壊れていれば実際にプレイを
// 始めた時点でcoreのdecideが拒否する既存の安全網がある)。
import { z } from "zod";
import type { Scenario } from "@tabifuda/ui";

const scenarioMetaSchema = z.object({
  // id/titleは空文字だと一覧上で選べない・区別できないカードになるため
  // 最低1文字を要求する(edge-case-reviewerで指摘、2026-09-12)。summaryは
  // domain-model.mdのBoundedString<400>が0文字を許容する仕様のためそのまま。
  id: z.string().min(1),
  title: z.string().min(1),
  summary: z.string(),
});

export interface LoadedScenario {
  id: string;
  title: string;
  summary: string;
  scenario: Scenario;
}

// 非再帰(サブディレクトリは対象外)。サブディレクトリに置いたシナリオは
// パターン自体が一致せず警告も無いまま一覧から漏れる点に注意
// (edge-case-reviewerで指摘、2026-09-12)。現状の運用(shared/scenarios/
// 直下へのフラット配置)ではこの制約は問題にならない。
const modules = import.meta.glob("../../../../shared/scenarios/*.json", {
  eager: true,
}) as Record<string, { default: unknown }>;

function loadScenario(path: string, mod: { default: unknown }): LoadedScenario | null {
  const raw = mod.default as { meta?: unknown } | undefined;
  const parsed = scenarioMetaSchema.safeParse(raw?.meta);
  if (!parsed.success) {
    // 1件の不備で一覧全体を壊さない(その1件だけ除外する)。
    console.warn(`シナリオの読み込みをスキップしました(${path}):`, parsed.error);
    return null;
  }
  return { ...parsed.data, scenario: raw as Scenario };
}

// ファイルシステムの列挙順は保証されないため、id順に揃えて表示順を決定的にする。
const loaded = Object.entries(modules)
  .map(([path, mod]) => loadScenario(path, mod))
  .filter((s): s is LoadedScenario => s !== null)
  .sort((a, b) => a.id.localeCompare(b.id));

// idが重複するファイルが存在する場合、`find(id)`が常に先頭側だけを返し
// 2件目以降のカードを選んでも1件目の中身が開始されてしまう(発番元が
// 作者データのためcoreの一意性検証を経由しない。edge-case-reviewerで
// 指摘、2026-09-12)。id順ソート後の先頭を残し、以降を除外・警告する。
const seenIds = new Set<string>();
export const scenarios: LoadedScenario[] = loaded.filter((s) => {
  if (seenIds.has(s.id)) {
    console.warn(`シナリオIDが重複しています。2件目以降を除外しました: ${s.id}`);
    return false;
  }
  seenIds.add(s.id);
  return true;
});
