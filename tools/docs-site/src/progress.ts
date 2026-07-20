import { load } from "js-yaml";

export type ProgressStatus = "done" | "in-progress" | "planned";

export interface CycleProgress {
  id: string;
  name: string;
  status: ProgressStatus;
}

export interface TaskProgress {
  id: string; // phase0, docs-site など(ディレクトリ名)
  group: "projects" | "tools";
  title: string;
  status: ProgressStatus;
  cycles: CycleProgress[];
  /** docs/ からの相対パス(GitHubリンク用) */
  source: string;
}

const STATUSES: ReadonlySet<string> = new Set(["done", "in-progress", "planned"]);

interface Frontmatter {
  status: ProgressStatus;
  cycles?: Record<string, ProgressStatus>;
}

function parseFrontmatter(raw: string, taskId: string): { fm: Frontmatter; body: string } {
  const m = raw.match(/^---\n([\s\S]*?)\n---\n/);
  if (m === null) {
    throw new Error(`task.md に frontmatter がない: ${taskId}(tasks/README.md「進捗 frontmatter」参照)`);
  }
  const doc = load(m[1]) as Record<string, unknown>;
  const status = doc.status;
  if (typeof status !== "string" || !STATUSES.has(status)) {
    throw new Error(`frontmatter の status が不正: ${taskId} (${String(status)})`);
  }
  const cycles = (doc.cycles ?? undefined) as Record<string, ProgressStatus> | undefined;
  for (const [cid, cst] of Object.entries(cycles ?? {})) {
    if (!STATUSES.has(cst)) {
      throw new Error(`frontmatter の cycles.${cid} が不正: ${taskId} (${String(cst)})`);
    }
  }
  return { fm: { status: status as ProgressStatus, cycles }, body: raw.slice(m[0].length) };
}

/** 本文のサイクル見出し(### C1: 名前 / ### D1: 名前)を抽出する */
function parseCycleHeadings(body: string): Map<string, string> {
  const headings = new Map<string, string>();
  for (const m of body.matchAll(/^### ([CD]\d+(?:\.\d+)?): (.+)$/gm)) {
    headings.set(m[1], m[2].trim());
  }
  return headings;
}

function parseTask(path: string, raw: string): TaskProgress {
  // path 例: ../../../docs/tasks/projects/phase0/task.md
  const rel = path.match(/docs\/(tasks\/(projects|tools)\/([^/]+)\/task\.md)$/);
  if (rel === null) throw new Error(`task.md のパスが想定外: ${path}`);
  const [, source, group, id] = rel;
  const { fm, body } = parseFrontmatter(raw, id);
  const headings = parseCycleHeadings(body);
  const fmIds = Object.keys(fm.cycles ?? {});

  // frontmatter と本文見出しの乖離はビルド時に落とす(進捗の正を腐らせない)
  const missingInBody = fmIds.filter((c) => !headings.has(c));
  const missingInFm = [...headings.keys()].filter((c) => !fmIds.includes(c));
  if (missingInBody.length > 0 || missingInFm.length > 0) {
    throw new Error(
      `frontmatter と見出しが不一致: ${id}` +
        (missingInBody.length > 0 ? ` / 見出しに無い: ${missingInBody.join(",")}` : "") +
        (missingInFm.length > 0 ? ` / frontmatterに無い: ${missingInFm.join(",")}` : ""),
    );
  }

  const title = body.match(/^# (.+)$/m)?.[1] ?? id;
  return {
    id,
    group: group as TaskProgress["group"],
    title,
    status: fm.status,
    cycles: fmIds.map((cid) => ({ id: cid, name: headings.get(cid) ?? "", status: fm.cycles![cid] })),
    source,
  };
}

function phaseOrder(id: string): number {
  const n = id.match(/^phase([\d.]+)$/);
  return n ? parseFloat(n[1]) : Number.MAX_SAFE_INTEGER;
}

export function loadTasks(rawByPath: Record<string, string>): TaskProgress[] {
  const tasks = Object.entries(rawByPath).map(([path, raw]) => parseTask(path, raw));
  return tasks.sort((a, b) =>
    a.group !== b.group
      ? a.group.localeCompare(b.group)
      : phaseOrder(a.id) - phaseOrder(b.id) || a.id.localeCompare(b.id),
  );
}
