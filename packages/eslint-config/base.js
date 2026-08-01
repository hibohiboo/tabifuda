import js from "@eslint/js";
import tseslint from "typescript-eslint";
import sonarjs from "eslint-plugin-sonarjs";

// フロントエンド・バックエンド共通の土台。P4でapps/apiが加わった際は
// backend.js(この配列を再利用)を追加する想定(docs/adr/0002-package-manager.md参照)。
export const baseConfig = tseslint.config(
  { ignores: ["dist", "node_modules"] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  sonarjs.configs.recommended,
);
