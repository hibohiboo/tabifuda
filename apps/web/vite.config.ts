import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const here = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      // crates/tabifuda-wasm/pkg(wasm-pack出力)と shared/scenarios/(シナリオJSON)を
      // 開発サーバから読めるようにする(いずれもapps/webの外、リポジトリルート配下)
      allow: [resolve(here, "../..")],
    },
  },
});
