import { sourceUrl } from "../model";

export default function TestsView() {
  return (
    <>
      <p className="view-note">
        テストビューは D2 で実装予定(docs/tasks/tools/docs-site/task.md 参照)。
        戦略の正は{" "}
        <a href={sourceUrl("design/test-strategy.md")} target="_blank" rel="noreferrer">
          docs/design/test-strategy.md
        </a>
        。テストの成否は master の{" "}
        <a
          href="https://github.com/hibohiboo/tabifuda/actions/workflows/ci.yml"
          target="_blank"
          rel="noreferrer"
        >
          CI
        </a>{" "}
        で検証されている。
      </p>
    </>
  );
}
