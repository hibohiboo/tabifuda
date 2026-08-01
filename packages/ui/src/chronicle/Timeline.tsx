import type { Event, Scenario } from "../core/bindings";
import { dispatchTagged } from "../core/taggedUnion";
import { eventRenderers, type ChronicleContext } from "./eventRenderers";

export function Timeline({ events, scenario }: { events: Event[]; scenario: Scenario }) {
  const ctx: ChronicleContext = { scenario };
  return (
    <ol className="timeline">
      {events.map((event, index) => {
        const node = dispatchTagged(event, eventRenderers, ctx);
        if (node === null) return null;
        return <li key={index}>{node}</li>;
      })}
    </ol>
  );
}
