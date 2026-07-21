import { load } from "js-yaml";

/** docs/rdra/*.yaml の要素共通部。source は docs/ からの相対パス+アンカー */
export interface RdraElement {
  id: string;
  name: string;
  description?: string;
  source: string;
}

export type Actor = RdraElement;

export interface UseCase extends RdraElement {
  actors?: string[];
  information?: string[];
  states?: string[];
}

export interface Information extends RdraElement {
  /** この情報が取りうる状態モデルへの参照(例: session -> running/paused/ended) */
  states?: string[];
}

export type StateItem = RdraElement;

export interface StateTransition {
  from: string | null; // null = その状態モデルが表す実体がまだ存在しない
  to: string;
  via: string; // usecase id
  label: string;
}

export interface Requirement extends RdraElement {
  status: "realized" | "future";
  actors?: string[];
}

export interface FlowStep {
  id: string;
  name: string;
  description?: string;
  actors?: string[];
  usecases?: string[];
  /** 主経路の一本道に含めず、任意発生の分岐として表示する */
  branch?: boolean;
}

export interface Flow {
  id: string;
  name: string;
  source: string;
  steps: FlowStep[];
}

export interface RdraModel {
  actors: Actor[];
  usecases: UseCase[];
  information: Information[];
  states: StateItem[];
  stateTransitions: StateTransition[];
  requirements: Requirement[];
  flows: Flow[];
}

const GITHUB_DOCS_BASE = "https://github.com/hibohiboo/tabifuda/blob/master/docs/";

/** source(docs/相対パス#アンカー)を GitHub 上の閲覧URLへ変換する */
export function sourceUrl(source: string): string {
  return GITHUB_DOCS_BASE + source;
}

function parseYaml(raw: string, label: string): Record<string, unknown> {
  const doc = load(raw);
  if (typeof doc !== "object" || doc === null) {
    throw new Error(`RDRAデータのYAMLがオブジェクトではない: ${label}`);
  }
  return doc as Record<string, unknown>;
}

function section<T>(doc: Record<string, unknown>, key: string, label: string): T[] {
  const items = doc[key];
  if (!Array.isArray(items)) {
    throw new Error(`RDRAデータに配列 '${key}' がない: ${label}`);
  }
  return items as T[];
}

export interface RdraYamlSources {
  actors: string;
  usecases: string;
  information: string;
  states: string;
  requirements: string;
  businessFlow: string;
}

export function parseModel(src: RdraYamlSources): RdraModel {
  const statesDoc = parseYaml(src.states, "states.yaml");
  const businessFlowDoc = parseYaml(src.businessFlow, "business-flow.yaml");
  return {
    actors: section<Actor>(parseYaml(src.actors, "actors.yaml"), "actors", "actors.yaml"),
    usecases: section<UseCase>(parseYaml(src.usecases, "usecases.yaml"), "usecases", "usecases.yaml"),
    information: section<Information>(
      parseYaml(src.information, "information.yaml"),
      "information",
      "information.yaml",
    ),
    states: section<StateItem>(statesDoc, "states", "states.yaml"),
    stateTransitions: section<StateTransition>(statesDoc, "transitions", "states.yaml"),
    requirements: section<Requirement>(
      parseYaml(src.requirements, "requirements.yaml"),
      "requirements",
      "requirements.yaml",
    ),
    flows: section<Flow>(businessFlowDoc, "flows", "business-flow.yaml"),
  };
}

interface RelationNode {
  id: string;
  refs: string[];
}

function relationNodes(model: RdraModel): RelationNode[] {
  const nodes: RelationNode[] = model.usecases.map((uc) => ({
    id: uc.id,
    refs: [...(uc.actors ?? []), ...(uc.information ?? []), ...(uc.states ?? [])],
  }));
  for (const r of model.requirements) {
    nodes.push({ id: r.id, refs: r.actors ?? [] });
  }
  for (const flow of model.flows) {
    for (const step of flow.steps) {
      nodes.push({ id: step.id, refs: [...(step.actors ?? []), ...(step.usecases ?? [])] });
    }
  }
  return nodes;
}

/**
 * 選択要素と直接関係する id の集合(選択なしなら null)。
 * usecase/requirement/業務フローの各ステップを「関係を運ぶノード」として扱い、
 * 選択idがノード自身ならそのrefs全部を、refsに含まれていればノード自身のidを加える
 * (アクター→関連UC→関連情報、のような1ホップ関係をどの層の要素からでも辿れる)。
 */
export function relatedIds(model: RdraModel, selectedId: string | null): Set<string> | null {
  if (selectedId === null) return null;
  const related = new Set<string>([selectedId]);
  for (const node of relationNodes(model)) {
    if (node.id === selectedId) {
      node.refs.forEach((id) => related.add(id));
    } else if (node.refs.includes(selectedId)) {
      related.add(node.id);
    }
  }
  // information -> states(例: session -> running/paused/ended)も辿る
  for (const info of model.information) {
    const refs = info.states ?? [];
    if (info.id === selectedId) refs.forEach((id) => related.add(id));
    else if (refs.includes(selectedId)) related.add(info.id);
  }
  return related;
}
