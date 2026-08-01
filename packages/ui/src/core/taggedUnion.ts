// ts-rsが生成する「単一キーオブジェクト」形式の外部タグ付き判別共用体
// (`{ "CardPlayed": {...} }`)から、タグの合併型とペイロード型を取り出す
// ユーティリティ。wasm-boundary.md方針3「TS側の網羅性はswitchではなく
// ハンドラマップで取る」の基盤(docs/design/client-conventions.md参照)。

/** Uの各メンバーが持つ唯一のキーの合併型。 */
export type TagOf<U extends object> = U extends unknown ? keyof U : never;

/** タグKに対応するペイロード型。 */
export type PayloadOf<U extends object, K extends TagOf<U>> = Extract<U, Record<K, unknown>>[K];

/**
 * Uの全タグを網羅するハンドラの型。Uに新タグが増えて再生成されると、
 * この型を満たすオブジェクトリテラルはキー不足でコンパイルエラーになる
 * (switch文のdefault:や部分的なif連鎖では発生しない静的検査)。
 * Cは描画・解決に必要な文脈(例: カード名解決用のScenario)。
 */
export type HandlerMap<U extends object, R, C = void> = {
  [K in TagOf<U>]: (payload: PayloadOf<U, K>, ctx: C) => R;
};

/** Uの値から実行時にタグを取り出す(ワイヤ形式=単一キーオブジェクト前提)。 */
export function tagOf<U extends object>(value: U): TagOf<U> {
  const keys = Object.keys(value);
  if (keys.length !== 1) {
    throw new Error(`tagged union value must have exactly one key: ${JSON.stringify(value)}`);
  }
  return keys[0] as TagOf<U>;
}

/** ハンドラマップに従ってUの値を1件処理する。 */
export function dispatchTagged<U extends object, R, C = void>(
  value: U,
  handlers: HandlerMap<U, R, C>,
  ctx: C,
): R {
  const tag = tagOf(value);
  const payload = (value as Record<string, unknown>)[tag as string] as PayloadOf<U, TagOf<U>>;
  return handlers[tag](payload, ctx);
}
