import { useState } from "react";
import { sourceUrl } from "../model";
import testReportJson from "../generated/test-report.json";
import type { TestReport, TestSuite } from "../testReport";

const testReport = testReportJson as TestReport;

function SuiteCard({ suite }: { suite: TestSuite }) {
  const [open, setOpen] = useState(false);
  return (
    <article className="task">
      <header className="task__header">
        <span className={`badge badge--${suite.failed > 0 ? "planned" : "done"}`}>
          {suite.failed > 0 ? `${suite.failed}件失敗` : `${suite.passed}件成功`}
        </span>
        <button type="button" className="task__title suite__toggle" onClick={() => setOpen((o) => !o)}>
          {suite.label}
        </button>
        <span className="suite__crate">{suite.crate}</span>
      </header>
      <p className="card__desc">{suite.description}</p>
      <a
        className="card__source"
        href={sourceUrl(`design/test-strategy.md#${suite.strategyAnchor}`)}
        target="_blank"
        rel="noreferrer"
      >
        出典: docs/design/test-strategy.md
      </a>
      {open && (
        <ul className="suite__tests">
          {suite.tests.map((t) => (
            <li key={t.name} className={t.ok ? "suite__test--ok" : "suite__test--failed"}>
              {t.name}
            </li>
          ))}
        </ul>
      )}
    </article>
  );
}

export default function TestsView() {
  return (
    <>
      <p className="view-note">
        テスト戦略の正は{" "}
        <a href={sourceUrl("design/test-strategy.md")} target="_blank" rel="noreferrer">
          docs/design/test-strategy.md
        </a>
        。以下はビルド時に <code>cargo test --workspace</code> を実行した実結果
        (生成: {new Date(testReport.generatedAt).toLocaleString("ja-JP")})。
        全 {testReport.totalPassed + testReport.totalFailed} 件中{" "}
        <strong>{testReport.totalPassed} 件成功</strong>
        {testReport.totalFailed > 0 && `、${testReport.totalFailed} 件失敗`}。
        スイート名をクリックすると個々のテスト名(日本語)が開く。
      </p>
      <section className="layer">
        <h2 className="layer__title">スイート一覧</h2>
        <div className="task-list">
          {testReport.suites.map((s) => (
            <SuiteCard key={s.id} suite={s} />
          ))}
        </div>
      </section>
    </>
  );
}
