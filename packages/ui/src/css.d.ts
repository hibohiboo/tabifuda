// CSSの副作用import(`import "./Card.css"`)に型を与える。packages/uiは
// wasmランタイムに依存しないビルドレスパッケージのため(client-conventions.md
// 「UIコンポーネントの置き場」)、vite/client型は取り込まず、必要最小限の
// 宣言のみここに置く。
declare module "*.css";
