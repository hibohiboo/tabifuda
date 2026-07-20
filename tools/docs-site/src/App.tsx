import { useEffect, useState } from "react";
import RdraView from "./views/RdraView";
import ProgressView from "./views/ProgressView";
import TestsView from "./views/TestsView";

const VIEWS = [
  { hash: "#/rdra", label: "RDRA", component: RdraView },
  { hash: "#/progress", label: "進捗", component: ProgressView },
  { hash: "#/tests", label: "テスト", component: TestsView },
] as const;

function useHash(): string {
  const [hash, setHash] = useState(window.location.hash);
  useEffect(() => {
    const onChange = () => setHash(window.location.hash);
    window.addEventListener("hashchange", onChange);
    return () => window.removeEventListener("hashchange", onChange);
  }, []);
  return hash;
}

export default function App() {
  const hash = useHash();
  const view = VIEWS.find((v) => v.hash === hash) ?? VIEWS[0];
  const View = view.component;

  return (
    <main className="board">
      <header className="board__header">
        <h1>Tabifuda docs-site</h1>
        <nav className="nav">
          {VIEWS.map((v) => (
            <a
              key={v.hash}
              href={v.hash}
              className={`nav__tab ${v === view ? "nav__tab--active" : ""}`}
            >
              {v.label}
            </a>
          ))}
        </nav>
      </header>
      <View />
    </main>
  );
}
