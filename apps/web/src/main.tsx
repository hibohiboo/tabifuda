import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./App.css";
import { ensureWasmReady } from "./core/wasmClient";

const root = document.getElementById("root");
if (root === null) {
  throw new Error("#root が index.html にない");
}

await ensureWasmReady();
createRoot(root).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
