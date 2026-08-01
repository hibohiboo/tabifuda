import js from "@eslint/js";
import tseslint from "typescript-eslint";
import reactHooks from "eslint-plugin-react-hooks";

export default tseslint.config(
  { ignores: ["node_modules"] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ["src/**/*.{ts,tsx}"],
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
