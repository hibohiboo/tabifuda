import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";
import { loadTasks } from "./src/progress";
import { generateTestReport } from "./scripts/gen-test-report.mjs";
import { checkDocLinks } from "./scripts/check-doc-links.mjs";

const here = dirname(fileURLToPath(import.meta.url));

// cargo testを実行し、テストビュー用データ(src/generated/test-report.json)を
// ビルドの都度作り直す(コミットしない生成物。docs/tasks/tools/docs-site/task.md D2)
function testReportPlugin(): Plugin {
  return {
    name: "generate-test-report",
    buildStart() {
      generateTestReport();
    },
  };
}

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

// docs/ 配下のmarkdown間相対リンクが指すファイルの存在をビルド時に検証する
// (リンク切れのある状態をPagesへ公開しない。scripts/check-doc-links.mjs参照)
function docLinkCheckPlugin(): Plugin {
  return {
    name: "check-doc-links",
    buildStart() {
      const broken = checkDocLinks();
      if (broken.length > 0) {
        throw new Error(`docs/ 内でリンク切れが${broken.length}件見つかりました:\n` + broken.join("\n"));
      }
    },
  };
}

// GitHub Pages(https://hibohiboo.github.io/tabifuda/)配下で配信するため base を固定
export default defineConfig({
  base: "/tabifuda/",
  plugins: [react(), progressFrontmatterCheck(), testReportPlugin(), docLinkCheckPlugin()],
});
