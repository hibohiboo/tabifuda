import { useLayoutEffect, useRef, useState } from "react";

/**
 * カードタイトルが1行に収まるよう、はみ出す場合だけフォントサイズを
 * 段階的に縮小する(2026-09-12、ユーザー指定: 改行が必要な長さの時だけ
 * 縮小し、収まる場合は既定サイズのまま)。
 *
 * `.tf-card__title`はCSSでwhite-space: nowrap; overflow: hidden;
 * text-overflow: ellipsis;(1行固定)にしてあるため、ここでは実測の
 * scrollWidth(全文表示に必要な幅)とclientWidth(実際に使える幅)を比較し、
 * minSizePxまで縮小しても収まらない場合はCSSのellipsis表示に委ねる。
 */
export function useAutoFitTitle(text: string, baseSizePx: number, minSizePx: number) {
  const ref = useRef<HTMLParagraphElement | null>(null);
  const [fontSize, setFontSize] = useState(baseSizePx);

  useLayoutEffect(() => {
    const el = ref.current;
    if (el === null) return;

    let size = baseSizePx;
    el.style.fontSize = `${size}px`;
    while (el.scrollWidth > el.clientWidth && size > minSizePx) {
      size -= 1;
      el.style.fontSize = `${size}px`;
    }
    setFontSize(size);
    // text/カード幅が変わるたびに1行に収まるかを再計算する
  }, [text, baseSizePx, minSizePx]);

  return { ref, fontSize };
}
