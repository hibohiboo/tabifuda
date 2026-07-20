import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";
import { loadTasks } from "./src/progress";

const here = dirname(fileURLToPath(import.meta.url));

// frontmatter と本文見出しの乖離をビルド時に落とす(進捗の正を腐らせない)。
// ブラウザ側の progressData.ts と同じ loadTasks を使う
function progressFrontmatterCheck(): Plugin {
  return {
    name: "progress-frontmatter-check",
    buildStart() {
      const rawByPath: Record<string, string> = {};
      for (const group of ["projects", "tools"]) {
        const groupDir = join(here, "../../docs/tasks", group);
        for (const entry of readdirSync(groupDir, { withFileTypes: true })) {
          if (!entry.isDirectory()) continue;
          const taskPath = join(groupDir, entry.name, "task.md");
          rawByPath[`docs/tasks/${group}/${entry.name}/task.md`] = readFileSync(taskPath, "utf8");
        }
      }
      loadTasks(rawByPath); // 不整合なら throw してビルドが落ちる
    },
  };
}

// GitHub Pages(https://hibohiboo.github.io/tabifuda/)配下で配信するため base を固定
export default defineConfig({
  base: "/tabifuda/",
  plugins: [react(), progressFrontmatterCheck()],
});
