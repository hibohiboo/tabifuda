import tseslint from "typescript-eslint";
import reactHooks from "eslint-plugin-react-hooks";
import { baseConfig } from "./base.js";

// Reactを使うワークスペース(apps/web・packages/ui)向け。
export const frontendConfig = tseslint.config(
  ...baseConfig,
  {
    files: ["src/**/*.{ts,tsx}", "e2e/**/*.{ts,tsx}"],
    plugins: { "react-hooks": reactHooks },
    rules: {
      ...reactHooks.configs.recommended.rules,
      // cross-cutting.md「自由入力(UGC)の取り扱い」: 生HTML挿入の禁止をCIで機械的に強制する
      "no-restricted-syntax": [
        "error",
        {
          selector: "JSXAttribute[name.name='dangerouslySetInnerHTML']",
          message:
            "dangerouslySetInnerHTMLは禁止(cross-cutting.md「自由入力(UGC)の取り扱い」)。装飾が必要なら限定マークアップを検討する。",
        },
      ],
    },
  },
);
