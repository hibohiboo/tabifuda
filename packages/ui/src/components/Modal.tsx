// カード展開用の最小モーダル(P6 C2、ライブラリ未導入方針のため自作)。
// フォーカストラップ等は作り込まず、Escキー・オーバーレイクリックでの
// クローズのみ対応する(ui-visual-design.md「演出の強さ」: 視認性・操作性
// を最優先し、装飾目的の作り込みはしない)。
import { useEffect, useRef } from "react";
import type { ReactNode } from "react";
import "./Modal.css";

export function Modal({ children, onClose }: { children: ReactNode; onClose: () => void }) {
  // 呼び出し側(Hand.tsx)は毎レンダーで新しいアロー関数を渡すため、effectの
  // 依存にonClose自体を含めるとモーダル表示中の無関係な再レンダーのたびに
  // documentのリスナーを付け外ししてしまう。refで最新値だけ追従させ、
  // effect自体は初回のみ登録する。
  const onCloseRef = useRef(onClose);
  onCloseRef.current = onClose;

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onCloseRef.current();
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, []);

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
