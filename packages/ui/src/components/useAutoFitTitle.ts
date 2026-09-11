import { useLayoutEffect, useRef, useState } from "react";

const STEP_PX = 0.5;
// Canvas計測とDOM描画のサブピクセル丸め差を吸収する小さな余裕。
const SAFETY_MARGIN_PX = 1;

// Canvas 2Dのテキスト計測は同一フォント設定なら使い回せるため、
// コンポーネント外に1つだけ用意する(呼び出しごとに生成しない)。
let measureCanvasCtx: CanvasRenderingContext2D | null | undefined;

function measureTextWidth(text: string, font: string): number {
  if (measureCanvasCtx === undefined) {
    measureCanvasCtx = document.createElement("canvas").getContext("2d");
  }
  if (measureCanvasCtx === null) return 0;
  measureCanvasCtx.font = font;
  return measureCanvasCtx.measureText(text).width;
}

/**
 * カードタイトルが1行に収まるよう、はみ出す場合だけフォントサイズを
 * 段階的に縮小する(2026-09-12、ユーザー指定: 改行が必要な長さの時だけ
 * 縮小し、収まる場合は既定サイズのまま)。
 *
 * `.tf-card__title`はCSSでwhite-space: nowrap; overflow: hidden;
 * text-overflow: ellipsis;(1行固定)にしてある。判定には
 * `scrollWidth`/`clientWidth`の比較は使わない ——
 * `.tf-card__title`は`flex: 1 1 0`で残り幅いっぱいに広がる実装のため、
 * コンテンツ(テキスト)が要素の幅より小さい場合、ブラウザの実装上
 * `scrollWidth`は要素自身の幅(clientWidthと同値)を返し、「実際に
 * どれだけ余裕があるか」を判定できない(2026-09-12、この方式で実装した
 * ところ短いタイトルまで誤って最小サイズまで縮小される不具合が発覚)。
 * 代わりにCanvas 2Dの`measureText`でテキストの実際の描画幅を直接計算し、
 * `clientWidth`(実際に使える幅。flex: 1により固定値になる)と比較する。
 */
export function useAutoFitTitle(text: string, baseSizePx: number, minSizePx: number) {
  const ref = useRef<HTMLParagraphElement | null>(null);
  const [fontSize, setFontSize] = useState(baseSizePx);

  useLayoutEffect(() => {
    const el = ref.current;
    if (el === null) return;

    const availableWidth = el.clientWidth;
    const fontFamily = getComputedStyle(el).fontFamily;

    let size = baseSizePx;
    while (
      measureTextWidth(text, `${size}px ${fontFamily}`) > availableWidth - SAFETY_MARGIN_PX &&
      size > minSizePx
    ) {
      size -= STEP_PX;
    }
    setFontSize(size);
    // text/カード幅が変わるたびに1行に収まるかを再計算する
  }, [text, baseSizePx, minSizePx]);

  return { ref, fontSize };
}
