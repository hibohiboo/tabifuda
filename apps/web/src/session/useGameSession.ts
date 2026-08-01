import { useCallback, useMemo, useReducer, useState } from "react";
import type { Command, Event, Session, UserId, WasmError } from "../core/bindings";
import { applyAll, decide } from "../core/wasmClient";

function appendEvents(state: Event[], added: Event[]): Event[] {
  return [...state, ...added];
}

export interface GameSession {
  events: Event[];
  session: Session | null;
  error: WasmError | null;
  dispatch: (command: Command) => void;
}

/**
 * イベント列だけをReact stateとし、Sessionはそこからの純粋な導出にする
 * (docs/tasks/projects/phase3/plans/c2-design-plan.md「状態管理」)。
 * `session`を独立stateとして持たないことで、2つのstateの不整合を
 * 構造的に排除する。
 */
export function useGameSession(actor: UserId): GameSession {
  const [events, dispatchEvents] = useReducer(appendEvents, [] as Event[]);
  const [error, setError] = useState<WasmError | null>(null);

  const session = useMemo(() => applyAll(events), [events]);

  const dispatch = useCallback(
    (command: Command) => {
      const result = decide(session, actor, command);
      if (result.ok) {
        setError(null);
        dispatchEvents(result.events);
      } else {
        setError(result.error);
      }
    },
    [session, actor],
  );

  return { events, session, error, dispatch };
}
