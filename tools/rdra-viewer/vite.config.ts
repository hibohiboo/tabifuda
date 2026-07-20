import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// GitHub Pages(https://hibohiboo.github.io/tabifuda/)配下で配信するため base を固定
export default defineConfig({
  base: "/tabifuda/",
  plugins: [react()],
});
