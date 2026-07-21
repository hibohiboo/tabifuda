import type { Flow, RdraModel } from "./model";

/** mermaidのノードidに使える形へ変換する(ハイフン等を避ける) */
function nodeId(id: string): string {
  return `n_${id.replace(/[^a-zA-Z0-9_]/g, "_")}`;
}

/** ラベル中の mermaid 構文と衝突しうる文字を無害化する(表示専用の簡略化) */
function sanitizeLabel(label: string): string {
  return label.replace(/::/g, "→").replace(/"/g, "'");
}

export function stateDiagram(model: RdraModel): string {
  const lines = ["stateDiagram-v2"];
  for (const s of model.states) {
    lines.push(`    ${nodeId(s.id)}: ${sanitizeLabel(s.name)}`);
  }
  for (const t of model.stateTransitions) {
    const from = t.from === null ? "[*]" : nodeId(t.from);
    lines.push(`    ${from} --> ${nodeId(t.to)}: ${sanitizeLabel(t.label)}`);
  }
  return lines.join("\n");
}

export function flowDiagram(flow: Flow): string {
  const lines = ["flowchart TD"];
  for (const step of flow.steps) {
    const shape = step.branch === true ? `{{"${sanitizeLabel(step.name)}"}}` : `["${sanitizeLabel(step.name)}"]`;
    lines.push(`    ${nodeId(step.id)}${shape}`);
  }
  const mainPath = flow.steps.filter((s) => s.branch !== true);
  // 主経路(branch:trueでないステップ)を一本道でつなぐ
  for (let i = 0; i < mainPath.length - 1; i += 1) {
    lines.push(`    ${nodeId(mainPath[i].id)} --> ${nodeId(mainPath[i + 1].id)}`);
  }
  // 分岐ステップは主経路の各ステップから点線で接続する(いつでも起こりうる)
  for (const branch of flow.steps.filter((s) => s.branch === true)) {
    for (const step of mainPath) {
      lines.push(`    ${nodeId(step.id)} -.-> ${nodeId(branch.id)}`);
    }
  }
  return lines.join("\n");
}
