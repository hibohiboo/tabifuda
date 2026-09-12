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
  id: z.string(),
  title: z.string(),
  summary: z.string(),
});

export interface LoadedScenario {
  id: string;
  title: string;
  summary: string;
  scenario: Scenario;
}

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
export const scenarios: LoadedScenario[] = Object.entries(modules)
  .map(([path, mod]) => loadScenario(path, mod))
  .filter((s): s is LoadedScenario => s !== null)
  .sort((a, b) => a.id.localeCompare(b.id));
