import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const here = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      // crates/tabifuda-wasm/pkg(wasm-pack出力)と shared/scenarios/(シナリオJSON)を
      // 開発サーバから読めるようにする(いずれもapps/webの外、リポジトリルート配下)
      allow: [resolve(here, "../..")],
    },
  },
  // vitest(P7 C2、2026-09-12導入)。単体テストの対象は
  // loadScenarios.tsのようなロジックを持つ純粋関数のみで、Playwright
  // (test:e2e)が担うE2E/コンポーネント描画とは領分を分ける
  // (.claude/rules/testing.md参照)。DOM APIを使わない対象のみのため
  // environmentは既定のnodeのまま(jsdom等は導入しない)。
  test: {
    include: ["src/**/*.test.ts"],
  },
});
