import { useEffect, useId, useRef, useState } from "react";
import type { Mermaid as MermaidApi } from "mermaid";

// mermaid本体(依存込みで大きい)はRDRAビューを開いたときだけ読み込む。
// 進捗/テストタブしか見ないユーザーの初期ロードを軽く保つ。
let mermaidPromise: Promise<MermaidApi> | null = null;
function loadMermaid(): Promise<MermaidApi> {
  if (mermaidPromise === null) {
    mermaidPromise = import("mermaid").then((mod) => {
      const api = mod.default;
      api.initialize({ startOnLoad: false, theme: "neutral", securityLevel: "strict" });
      return api;
    });
  }
  return mermaidPromise;
}

/**
 * docs/rdra/*.yaml(ビルド時に取り込む信頼済みデータ)由来のMermaid定義のみを描画する。
 * ユーザー入力・実行時UGCはここを通らない(cross-cutting.md「自由入力(UGC)の取り扱い」の
 * 対象外。生SVGの挿入はmermaid自身がsecurityLevel:"strict"でサニタイズする)。
 */
export default function Mermaid({ definition }: { definition: string }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [error, setError] = useState<string | null>(null);
  const id = `mermaid-${useId().replace(/:/g, "")}`;

  useEffect(() => {
    let cancelled = false;
    loadMermaid()
      .then((mermaid) => mermaid.render(id, definition))
      .then(({ svg }) => {
        if (cancelled || containerRef.current === null) return;
        containerRef.current.innerHTML = svg;
      })
      .catch((err: unknown) => {
        if (!cancelled) setError(err instanceof Error ? err.message : String(err));
      });
    return () => {
      cancelled = true;
    };
  }, [definition, id]);

  if (error !== null) {
    return <p className="mermaid__error">図の描画に失敗しました: {error}</p>;
  }
  return <div className="mermaid" ref={containerRef} />;
}
