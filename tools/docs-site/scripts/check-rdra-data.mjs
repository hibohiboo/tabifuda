// docs/rdra/*.yaml の構造検証(zodスキーマ)+ source(docs/相対パス+アンカー)の
// リンク先ファイル・アンカー存在チェック + 参照idの存在チェックを行う。
// GitHub Pagesへのデプロイ前にゲートし、規範文書の節名変更への追従漏れ
// (docs/rdra/README.md「更新の規律」)や壊れた参照を公開しない。
// アンカーはGitHubの見出しスラグ生成アルゴリズム(小文字化 + 文字/数字/空白/
// ハイフン/アンダースコア以外を除去 + 空白をハイフンへ + 同一スラグの重複は
// -1,-2...を付与)を再現して照合する。完全な再現ではないため、GitHub側の
// 挙動が変わった場合はここも追従する必要がある。

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { load } from "js-yaml";
import { z } from "zod";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(here, "../../..");
const docsRoot = join(repoRoot, "docs");

const RdraElementSchema = z.object({
  id: z.string().min(1),
  name: z.string().min(1),
  description: z.string().optional(),
  source: z.string().min(1),
});

const UseCaseSchema = RdraElementSchema.extend({
  actors: z.array(z.string()).optional(),
  information: z.array(z.string()).optional(),
  states: z.array(z.string()).optional(),
});

const InformationSchema = RdraElementSchema.extend({
  states: z.array(z.string()).optional(),
});

const StateTransitionSchema = z.object({
  from: z.string().nullable(),
  to: z.string(),
  via: z.string(),
  label: z.string(),
});

const RequirementSchema = RdraElementSchema.extend({
  status: z.enum(["realized", "future"]),
  actors: z.array(z.string()).optional(),
});

const FlowStepSchema = z.object({
  id: z.string().min(1),
  name: z.string().min(1),
  description: z.string().optional(),
  actors: z.array(z.string()).optional(),
  usecases: z.array(z.string()).optional(),
  branch: z.boolean().optional(),
});

const FlowSchema = z.object({
  id: z.string().min(1),
  name: z.string().min(1),
  source: z.string().min(1),
  steps: z.array(FlowStepSchema),
});

// ファイル名 -> (トップレベルキー -> 配列要素のzodスキーマ)。
// 1ファイルに複数キー(states.yamlはstates+transitions)を持つものもある。
const FILES = {
  "actors.yaml": { actors: RdraElementSchema },
  "usecases.yaml": { usecases: UseCaseSchema },
  "information.yaml": { information: InformationSchema },
  "states.yaml": { states: RdraElementSchema, transitions: StateTransitionSchema },
  "requirements.yaml": { requirements: RequirementSchema },
  "business-flow.yaml": { flows: FlowSchema },
};

function loadYamlFile(fileName) {
  const raw = readFileSync(join(docsRoot, "rdra", fileName), "utf-8");
  const doc = load(raw);
  if (typeof doc !== "object" || doc === null) {
    throw new Error(`RDRAデータのYAMLがオブジェクトではない: ${fileName}`);
  }
  return doc;
}

/** zodスキーマ検証。エラーがあれば `"ファイル名: 内容"` の配列を返す(例外は投げない) */
function validateSchema(fileName, doc, keySchemas) {
  const errors = [];
  for (const [key, itemSchema] of Object.entries(keySchemas)) {
    const items = doc[key];
    if (!Array.isArray(items)) {
      errors.push(`${fileName}: 配列 '${key}' が無い`);
      continue;
    }
    const result = z.array(itemSchema).safeParse(items);
    if (!result.success) {
      for (const issue of result.error.issues) {
        errors.push(`${fileName}.${key}[${issue.path.join(".")}]: ${issue.message}`);
      }
    }
  }
  return errors;
}

// GitHub見出しスラグの簡易再現(references/comrak準拠の主要ルールのみ)
function slugify(headingText) {
  return headingText
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s_-]/gu, "")
    .trim()
    .replace(/\s+/g, "-");
}

const headingSlugCache = new Map();

function headingSlugs(relPath) {
  if (headingSlugCache.has(relPath)) return headingSlugCache.get(relPath);
  const text = readFileSync(join(docsRoot, relPath), "utf-8");
  const slugs = new Set();
  const counts = new Map();
  let inFence = false;
  for (const line of text.split(/\r?\n/)) {
    if (/^\s*```/.test(line)) {
      inFence = !inFence;
      continue;
    }
    if (inFence) continue;
    const m = line.match(/^#{1,6}\s+(.+?)\s*$/);
    if (m === null) continue;
    let slug = slugify(m[1]);
    const count = counts.get(slug) ?? 0;
    counts.set(slug, count + 1);
    if (count > 0) slug = `${slug}-${count}`;
    slugs.add(slug);
  }
  headingSlugCache.set(relPath, slugs);
  return slugs;
}

/** source(docs/相対パス+#アンカー)がファイル・アンカーとも存在するか検証する */
function checkSource(fileName, id, source) {
  const [relPath, anchor] = source.split("#");
  const fullPath = join(docsRoot, relPath);
  let text;
  try {
    text = readFileSync(fullPath, "utf-8");
  } catch {
    return [`${fileName} (id=${id}): source先ファイルが無い: docs/${relPath}`];
  }
  if (anchor === undefined) return [];
  void text; // headingSlugsが再度読むため、存在確認のみここで済ませる
  const slugs = headingSlugs(relPath);
  if (!slugs.has(anchor)) {
    return [`${fileName} (id=${id}): source先アンカーが見出しに無い: docs/${relPath}#${anchor}`];
  }
  return [];
}

function collectElements(model) {
  // { id, refs: string[], fileName, sourceOwner? } の平坦なリスト。
  // refsはこの要素が参照する他要素のid(存在チェック対象)。
  const elements = [];
  for (const a of model.actors.actors) elements.push({ id: a.id, fileName: "actors.yaml", refs: [] });
  for (const uc of model.usecases.usecases) {
    elements.push({
      id: uc.id,
      fileName: "usecases.yaml",
      refs: [...(uc.actors ?? []), ...(uc.information ?? []), ...(uc.states ?? [])],
    });
  }
  for (const info of model.information.information) {
    elements.push({ id: info.id, fileName: "information.yaml", refs: [...(info.states ?? [])] });
  }
  for (const s of model.states.states) elements.push({ id: s.id, fileName: "states.yaml", refs: [] });
  for (const t of model.states.transitions ?? []) {
    const refs = [t.to, t.via];
    if (t.from !== null) refs.push(t.from);
    elements.push({ id: null, fileName: "states.yaml", refs });
  }
  for (const r of model.requirements.requirements) {
    elements.push({ id: r.id, fileName: "requirements.yaml", refs: [...(r.actors ?? [])] });
  }
  for (const flow of model.businessFlow.flows) {
    elements.push({ id: flow.id, fileName: "business-flow.yaml", refs: [] });
    for (const step of flow.steps) {
      elements.push({
        id: step.id,
        fileName: "business-flow.yaml",
        refs: [...(step.actors ?? []), ...(step.usecases ?? [])],
      });
    }
  }
  return elements;
}

export function checkRdraData() {
  const errors = [];
  const docs = {};
  for (const [fileName, keySchemas] of Object.entries(FILES)) {
    const doc = loadYamlFile(fileName);
    errors.push(...validateSchema(fileName, doc, keySchemas));
    docs[fileName] = doc;
  }
  if (errors.length > 0) return errors; // スキーマが壊れていたら以降の検証は意味を持たない

  const model = {
    actors: docs["actors.yaml"],
    usecases: docs["usecases.yaml"],
    information: docs["information.yaml"],
    states: docs["states.yaml"],
    requirements: docs["requirements.yaml"],
    businessFlow: docs["business-flow.yaml"],
  };
  const elements = collectElements(model);

  // id一意性(ファイル横断。docs/rdra/README.md「id(kebab-case、ファイル横断で一意)」)
  const seenIds = new Map();
  for (const el of elements) {
    if (el.id === null) continue;
    const owner = seenIds.get(el.id);
    if (owner !== undefined && owner !== el.fileName) {
      errors.push(`id重複: '${el.id}' が ${owner} と ${el.fileName} の両方にある`);
    }
    seenIds.set(el.id, el.fileName);
  }

  // 参照idの存在チェック
  const allIds = new Set(seenIds.keys());
  for (const el of elements) {
    for (const ref of el.refs) {
      if (!allIds.has(ref)) {
        const owner = el.id ?? "(transition)";
        errors.push(`${el.fileName} (id=${owner}): 存在しないidを参照している: ${ref}`);
      }
    }
  }

  // source(ファイル・アンカー)の存在チェック
  for (const [fileName, keySchemas] of Object.entries(FILES)) {
    for (const key of Object.keys(keySchemas)) {
      for (const item of docs[fileName][key] ?? []) {
        if (typeof item.source === "string") {
          errors.push(...checkSource(fileName, item.id, item.source));
        }
      }
    }
  }

  return errors;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const errors = checkRdraData();
  if (errors.length > 0) {
    console.error(`docs/rdra/ のデータ検証で${errors.length}件のエラーが見つかりました:`);
    for (const line of errors) console.error(`  ${line}`);
    process.exitCode = 1;
  } else {
    console.log("docs/rdra/ のデータ検証は全て通過しました。");
  }
}
