import type { WasmError } from "../core/bindings";

export function ErrorBanner({ error }: { error: WasmError | null }) {
  if (error === null) return null;
  return (
    <p role="alert">
      エラー({error.kind}): {JSON.stringify(error.error)}
    </p>
  );
}
