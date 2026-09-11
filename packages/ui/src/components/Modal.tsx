// カード展開用の最小モーダル(P6 C2、ライブラリ未導入方針のため自作)。
// フォーカストラップ等は作り込まず、Escキー・オーバーレイクリックでの
// クローズのみ対応する(ui-visual-design.md「演出の強さ」: 視認性・操作性
// を最優先し、装飾目的の作り込みはしない)。
import { useEffect } from "react";
import type { ReactNode } from "react";
import "./Modal.css";

export function Modal({ children, onClose }: { children: ReactNode; onClose: () => void }) {
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [onClose]);

  return (
    <div
      className="tf-modal-overlay"
      onClick={(event) => {
        // オーバーレイ自体のクリックのみ閉じる(中身へのクリックが伝播した
        // ケースは無視する)
        if (event.target === event.currentTarget) onClose();
      }}
    >
      <div className="tf-modal" role="dialog" aria-modal="true">
        {children}
      </div>
    </div>
  );
}
