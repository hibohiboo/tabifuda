// docs/ 配下のmarkdown間相対リンクが指すファイルの存在を検証する。
// GitHub Pagesへのデプロイ前にpages.ymlのCIステップとして実行し、
// リンク切れのある状態を公開しない(出典: ユーザー依頼 2026-07-22)。
// アンカー(#見出し)の存在までは検証しない(GitHubのスラグ生成は完全な
// 再現が難しいため。docs/tasks/tools/docs-site/task.md C3で別途扱う)。

import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(here, "../../..");
const docsRoot = join(repoRoot, "docs");

// これらから始まるリンクはリポジトリルート相対、それ以外はリンク元ファイルからの相対
const ROOT_RELATIVE_PREFIXES = ["docs", ".claude", "crates", "tools"];

const LINK_RE = /\]\(([^)#\s]+?\.md)(#[^)]*)?\)/g;

function listMarkdownFiles(dir) {
  const out = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) out.push(...listMarkdownFiles(full));
    else if (entry.isFile() && entry.name.endsWith(".md")) out.push(full);
  }
  return out;
}

function resolveTarget(link, fileDir) {
  const firstSegment = link.split(/[\\/]/)[0];
  const base = ROOT_RELATIVE_PREFIXES.includes(firstSegment) ? repoRoot : fileDir;
  return normalize(join(base, link));
}

function exists(path) {
  try {
    statSync(path);
    return true;
  } catch {
    return false;
  }
}

export function checkDocLinks() {
  const broken = [];
  for (const file of listMarkdownFiles(docsRoot)) {
    const text = readFileSync(file, "utf-8");
    const fileDir = dirname(file);
    for (const m of text.matchAll(LINK_RE)) {
      const link = m[1];
      if (/^https?:\/\//.test(link)) continue;
      const target = resolveTarget(link, fileDir);
      if (!exists(target)) {
        broken.push(`${relative(repoRoot, file)} -> ${link}`);
      }
    }
  }
  return broken;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const broken = checkDocLinks();
  if (broken.length > 0) {
    console.error(`docs/ 内でリンク切れが${broken.length}件見つかりました:`);
    for (const line of broken) console.error(`  ${line}`);
    process.exitCode = 1;
  } else {
    console.log("docs/ 内の相対リンクは全て解決できました。");
  }
}
