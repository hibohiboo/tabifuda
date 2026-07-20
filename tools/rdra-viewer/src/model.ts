import { load } from "js-yaml";

/** docs/rdra/*.yaml の要素共通部。source は docs/ からの相対パス+アンカー */
export interface RdraElement {
  id: string;
  name: string;
  description?: string;
  source: string;
}

export interface Actor extends RdraElement {}

export interface UseCase extends RdraElement {
  actors?: string[];
  information?: string[];
  states?: string[];
}

export interface RdraModel {
  actors: Actor[];
  usecases: UseCase[];
}

const GITHUB_DOCS_BASE = "https://github.com/hibohiboo/tabifuda/blob/master/docs/";

/** source(docs/相対パス#アンカー)を GitHub 上の閲覧URLへ変換する */
export function sourceUrl(source: string): string {
  return GITHUB_DOCS_BASE + source;
}

function parseSection<T>(raw: string, key: string): T[] {
  const doc = load(raw);
  if (typeof doc !== "object" || doc === null) {
    throw new Error(`RDRAデータのYAMLがオブジェクトではない: ${key}`);
  }
  const items = (doc as Record<string, unknown>)[key];
  if (!Array.isArray(items)) {
    throw new Error(`RDRAデータに配列 '${key}' がない`);
  }
  return items as T[];
}

export function parseModel(actorsYaml: string, usecasesYaml: string): RdraModel {
  return {
    actors: parseSection<Actor>(actorsYaml, "actors"),
    usecases: parseSection<UseCase>(usecasesYaml, "usecases"),
  };
}

/** 選択要素と直接関係する id の集合(選択なしなら null) */
export function relatedIds(model: RdraModel, selectedId: string | null): Set<string> | null {
  if (selectedId === null) return null;
  const related = new Set<string>([selectedId]);
  for (const uc of model.usecases) {
    const refs = [...(uc.actors ?? []), ...(uc.information ?? []), ...(uc.states ?? [])];
    if (uc.id === selectedId) {
      refs.forEach((id) => related.add(id));
    } else if (refs.includes(selectedId)) {
      related.add(uc.id);
    }
  }
  return related;
}
