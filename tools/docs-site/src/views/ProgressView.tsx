import { useState } from "react";
import { tasks } from "../progressData";
import { sourceUrl } from "../model";
import type { ProgressStatus, TaskProgress } from "../progress";

const STATUS_LABEL: Record<ProgressStatus, string> = {
  done: "完了",
  "in-progress": "進行中",
  planned: "未着手",
  frozen: "凍結",
};

type Filter = "active" | "done" | "all";

const FILTER_LABEL: Record<Filter, string> = {
  active: "これから",
  done: "完了",
  all: "全部",
};

function matchesFilter(status: ProgressStatus, filter: Filter): boolean {
  if (filter === "all") return true;
  if (filter === "done") return status === "done";
  return status !== "done";
}

function TaskCard({ task }: { task: TaskProgress }) {
  return (
    <article className={`task task--${task.status}`}>
      <header className="task__header">
        <span className={`badge badge--${task.status}`}>{STATUS_LABEL[task.status]}</span>
        <a href={sourceUrl(task.source)} target="_blank" rel="noreferrer" className="task__title">
          {task.title}
        </a>
      </header>
      {task.cycles.length > 0 && (
        <ul className="task__cycles">
          {task.cycles.map((c) => (
            <li key={c.id} className={`cycle cycle--${c.status}`} title={STATUS_LABEL[c.status]}>
              <span className="cycle__id">{c.id}</span>
              <span className="cycle__name">{c.name}</span>
            </li>
          ))}
        </ul>
      )}
    </article>
  );
}

export default function ProgressView() {
  const [filter, setFilter] = useState<Filter>("active");
  const allCycles = tasks.flatMap((t) => t.cycles);
  const doneCycles = allCycles.filter((c) => c.status === "done").length;
  const visibleTasks = tasks.filter((t) => matchesFilter(t.status, filter));
  const projects = visibleTasks.filter((t) => t.group === "projects");
  const tools = visibleTasks.filter((t) => t.group === "tools");

  return (
    <>
      <p className="view-note">
        進捗の正は各 task.md の frontmatter(サイクル完了と同PRで更新)。
        全サイクル {allCycles.length} 件中 <strong>{doneCycles} 件完了</strong>。
      </p>
      <div className="view-filter">
        {(Object.keys(FILTER_LABEL) as Filter[]).map((f) => (
          <button
            key={f}
            type="button"
            className={`view-filter__tab${f === filter ? " view-filter__tab--active" : ""}`}
            onClick={() => setFilter(f)}
          >
            {FILTER_LABEL[f]}
          </button>
        ))}
      </div>
      <section className="layer">
        <h2 className="layer__title">projects(開発フェーズ)</h2>
        <div className="task-list">
          {projects.map((t) => (
            <TaskCard key={t.id} task={t} />
          ))}
        </div>
      </section>
      <section className="layer">
        <h2 className="layer__title">tools(開発支援ツール)</h2>
        <div className="task-list">
          {tools.map((t) => (
            <TaskCard key={t.id} task={t} />
          ))}
        </div>
      </section>
    </>
  );
}
