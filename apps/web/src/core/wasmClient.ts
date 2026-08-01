// crates/tabifuda-wasm(wasm-pack --target web出力)への薄いJSONラッパー。
// 深い相対importはこのファイルとbindings.tsに閉じ込める。
import initWasm, {
  decide as wasmDecide,
  apply_all as wasmApplyAll,
  init as initPanicHook,
} from "../../../../crates/tabifuda-wasm/pkg/tabifuda_wasm";
import type { Command, Event, Session, WasmError } from "./bindings";

let ready: Promise<void> | null = null;

/** wasmモジュールの読み込み+panic hook設定。アプリ起動時に1回awaitする。 */
export function ensureWasmReady(): Promise<void> {
  if (ready === null) {
    ready = initWasm().then(() => {
      initPanicHook();
    });
  }
  return ready;
}

export type DecideResult =
  | { ok: true; events: Event[] }
  | { ok: false; error: WasmError };

export function decide(session: Session | null, actor: string, command: Command): DecideResult {
  try {
    const eventsJson = wasmDecide(
      session ? JSON.stringify(session) : null,
      actor,
      JSON.stringify(command),
    );
    return { ok: true, events: JSON.parse(eventsJson) as Event[] };
  } catch (raw) {
    return { ok: false, error: parseWasmError(raw) };
  }
}

/**
 * `apply_all(null, events)`は全イベントからのリプレイと等価
 * (docs/design/wasm-boundary.md「apply_all」)。UI状態はここから導出する。
 */
export function applyAll(events: Event[]): Session | null {
  const json = wasmApplyAll(null, JSON.stringify(events));
  return json === undefined ? null : (JSON.parse(json) as Session);
}

function parseWasmError(raw: unknown): WasmError {
  if (typeof raw === "string") {
    try {
      return JSON.parse(raw) as WasmError;
    } catch {
      // 想定外の非JSON文字列(通常はwasm境界がWasmError JSONのみを投げる)
    }
  }
  return { kind: "decode", error: String(raw) };
}
