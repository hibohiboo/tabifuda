// docs/tasks/ の全 task.md をビルド時に取り込む(進捗の正は各 task.md の frontmatter)
import { loadTasks } from "./progress";

const rawByPath = import.meta.glob("../../../docs/tasks/{projects,tools}/*/task.md", {
  query: "?raw",
  import: "default",
  eager: true,
}) as Record<string, string>;

export const tasks = loadTasks(rawByPath);
