// カード展開用の最小モーダル(P6 C2、ライブラリ未導入方針のため自作)。
// Escキー・オーバーレイクリックでのクローズと、最小限のフォーカストラップ
// (Tabキーでの背後要素への移動をブロックする)のみ実装する。それ以外の
// 演出(装飾アニメーション等)は作り込まない(ui-visual-design.md「演出の
// 強さ」)。フォーカストラップが無いと、モーダルを開いたままTabで背後の
// フォーム(例: ProposalForm)へ移動して送信でき、その結果セッションが
// 遷移してモーダルごとアンマウントされ、入力中の自由入力が無警告で消える
// (edge-case-reviewerで指摘、2026-09-11)。
import { useEffect, useRef } from "react";
import type { ReactNode, RefObject } from "react";
import "./Modal.css";

function getFocusable(container: HTMLElement): HTMLElement[] {
  return Array.from(
    container.querySelectorAll<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
    ),
  ).filter((el) => !el.hasAttribute("disabled"));
}

export function Modal({
  children,
  onClose,
  restoreFocusFallbackRef,
}: {
  children: ReactNode;
  onClose: () => void;
  /** 開く前にフォーカスしていた要素が閉じる時点でDOMから消えている場合
   *  (例: Hand.tsxで「出す」を押しカードが手札から除去された場合)の
   *  フォーカス復帰先。省略時は何もしない(ブラウザ既定でbodyへ戻る)。
   *  edge-case-reviewerで指摘、2026-09-12: 「出す」経路では
   *  previouslyFocusedが常にdetachedになりフォーカス復帰が効かなかった。 */
  restoreFocusFallbackRef?: RefObject<HTMLElement | null>;
}) {
  // 呼び出し側(Hand.tsx)は毎レンダーで新しいアロー関数を渡すため、effectの
  // 依存にonClose自体を含めるとモーダル表示中の無関係な再レンダーのたびに
  // documentのリスナーを付け外ししてしまう。refで最新値だけ追従させ、
  // effect自体は初回のみ登録する。
  const onCloseRef = useRef(onClose);
  useEffect(() => {
    onCloseRef.current = onClose;
  });
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // 初期フォーカスをモーダル内へ移す。閉じたら元の要素へ戻す
    // (/code-review指摘、2026-09-12: 戻さないとキーボード操作で
    // 手札一覧からモーダルを開いて閉じるたびにフォーカスがdocument.body
    // へ飛び、次のTabがページ先頭からやり直しになっていた)。
    const previouslyFocused: Element | null = document.activeElement;
    // クリーンアップ実行時点でrestoreFocusFallbackRef.currentを直接読むと
    // 「その時点で変わっている可能性がある」とeslintに指摘されるため、
    // ここでローカル変数にコピーしておく(このrefは通常アンマウントまで
    // 同じ要素を指し続けるため実質的な違いは無い)。
    const fallback = restoreFocusFallbackRef?.current ?? null;
    const container = containerRef.current;
    if (container !== null) getFocusable(container)[0]?.focus();

    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        // IME変換確定前のEscapeは変換のキャンセルであり、モーダルを
        // 閉じる操作ではない(isComposingを見ないと日本語自由入力中に
        // 変換をキャンセルしただけで入力内容ごと消えてしまう)
        if (event.isComposing) return;
        onCloseRef.current();
        return;
      }
      if (event.key === "Tab" && container !== null) {
        const focusable = getFocusable(container);
        if (focusable.length === 0) return;
        const first = focusable[0];
        const last = focusable[focusable.length - 1];
        if (event.shiftKey && document.activeElement === first) {
          event.preventDefault();
          last.focus();
        } else if (!event.shiftKey && document.activeElement === last) {
          event.preventDefault();
          first.focus();
        }
      }
    };
    document.addEventListener("keydown", onKeyDown);
    return () => {
      document.removeEventListener("keydown", onKeyDown);
      // 「出す」でカード自体が手札から除去されると、previouslyFocusedは
      // このクリーンアップ実行時点で既にDOMから切り離されている
      // (isConnected: false)。その場合はfallback(呼び出し側が用意する
      // 手札一覧コンテナ等)へフォーカスする。
      if (previouslyFocused instanceof HTMLElement && previouslyFocused.isConnected) {
        previouslyFocused.focus();
      } else {
        fallback?.focus();
      }
    };
  }, [restoreFocusFallbackRef]);

  return (
    <div
      className="tf-modal-overlay"
      onClick={(event) => {
        // オーバーレイ自体のクリックのみ閉じる(中身へのクリックが伝播した
        // ケースは無視する)
        if (event.target === event.currentTarget) onClose();
      }}
    >
      <div className="tf-modal" role="dialog" aria-modal="true" ref={containerRef}>
        {children}
      </div>
    </div>
  );
}
