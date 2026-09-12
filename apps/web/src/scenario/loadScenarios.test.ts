// buildScenarioList(loadScenarios.tsのロジック本体)の単体テスト。
// import.meta.glob自体はビルド時静的解決のためテスト対象にできない
// (loadScenarios.ts参照)。P7 C2、edge-case-reviewer指摘(2026-09-12)。
import { describe, expect, it, vi } from "vitest";
import { buildScenarioList } from "./loadScenarios";

function mod(meta: unknown): { default: unknown } {
  return { default: { meta, card_defs: [], phases: [] } };
}

describe("buildScenarioList", () => {
  it("有効な複数シナリオをid順で返す", () => {
    const modules = {
      "./b.json": mod({ id: "b", title: "B", summary: "" }),
      "./a.json": mod({ id: "a", title: "A", summary: "概要" }),
    };

    const result = buildScenarioList(modules);

    expect(result.map((s) => s.id)).toEqual(["a", "b"]);
    expect(result[0]).toMatchObject({ id: "a", title: "A", summary: "概要" });
  });

  it("meta検証に失敗したファイルは1件だけ除外し、残りは読み込む", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const modules = {
      "./broken.json": mod({ id: "", title: "空id", summary: "" }),
      "./ok.json": mod({ id: "ok", title: "OK", summary: "" }),
    };

    const result = buildScenarioList(modules);

    expect(result.map((s) => s.id)).toEqual(["ok"]);
    expect(warn).toHaveBeenCalled();
    warn.mockRestore();
  });

  it("titleが空文字のシナリオを除外する", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const modules = {
      "./notitle.json": mod({ id: "no-title", title: "", summary: "" }),
    };

    const result = buildScenarioList(modules);

    expect(result).toEqual([]);
    warn.mockRestore();
  });

  it("同一idの複数ファイルは2件目以降を除外する(先勝ちで別ファイルの中身を握りつぶさない)", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const modules = {
      "./dup1.json": mod({ id: "dup", title: "1件目", summary: "" }),
      "./dup2.json": mod({ id: "dup", title: "2件目", summary: "" }),
    };

    const result = buildScenarioList(modules);

    expect(result).toHaveLength(1);
    expect(result[0]?.title).toBe("1件目");
    expect(warn).toHaveBeenCalledWith(expect.stringContaining("dup"));
    warn.mockRestore();
  });

  it("シナリオが1件も無ければ空配列を返す", () => {
    expect(buildScenarioList({})).toEqual([]);
  });
});
