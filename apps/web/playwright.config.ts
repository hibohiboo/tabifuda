import { defineConfig } from "@playwright/test";

// P3 C4スモーク(test-strategy.md「E2E/スモーク」: Webでテンプレシナリオを1本通す)。
export default defineConfig({
  testDir: "./e2e",
  fullyParallel: true,
  reporter: "list",
  use: {
    baseURL: "http://localhost:4173",
  },
  webServer: {
    command: "pnpm run build && pnpm exec vite preview --port 4173 --strictPort",
    url: "http://localhost:4173",
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
