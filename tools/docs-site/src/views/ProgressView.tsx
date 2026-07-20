import { tasks } from "../progressData";
import { sourceUrl } from "../model";
import type { ProgressStatus, TaskProgress } from "../progress";

const STATUS_LABEL: Record<ProgressStatus, string> = {
  done: "完了",
  "in-progress": "進行中",
  planned: "未着手",
};

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
  const allCycles = tasks.flatMap((t) => t.cycles);
  const doneCycles = allCycles.filter((c) => c.status === "done").length;
  const projects = tasks.filter((t) => t.group === "projects");
  const tools = tasks.filter((t) => t.group === "tools");

  return (
    <>
      <p className="view-note">
        進捗の正は各 task.md の frontmatter(サイクル完了と同PRで更新)。
        全サイクル {allCycles.length} 件中 <strong>{doneCycles} 件完了</strong>。
      </p>
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
