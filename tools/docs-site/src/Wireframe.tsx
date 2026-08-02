import type { LayoutBlock } from "./model";

/** スマホ幅の枠に、sizeHintに応じた高さ比のラベル付きボックスを縦積みする簡易ワイヤーフレーム */
export default function Wireframe({ layout }: { layout: LayoutBlock[] }) {
  return (
    <div className="wireframe">
      {layout.map((block, i) => (
        <div key={i} className={`wireframe__block wireframe__block--${block.sizeHint}`}>
          {block.label}
        </div>
      ))}
    </div>
  );
}
