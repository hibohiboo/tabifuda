// cargo test --workspace を実行し、結果を docs-site のテストビュー用JSONへ変換する。
// 生成物(src/generated/test-report.json)はコミットしない(ビルドの度に再生成)。
// 出典: docs/tasks/tools/docs-site/task.md D2、docs/design/test-strategy.md

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(here, "../../..");
const outPath = join(here, "../src/generated/test-report.json");

// スイートID(モジュール名またはtests/配下のファイル名)→ test-strategy.md 上の位置づけ。
// 未知のスイートが現れたら失敗させる(テスト追加時の分類漏れを検知するゲート)。
const SUITES = {
  engine_tests: {
    crate: "tabifuda-core",
    label: "decide/applyのテーブル駆動テスト",
    description: "各Commandの受理/拒否をテーブル駆動で対にして検証する。",
    strategyAnchor: "1-cratescoreテストの8割をここに",
  },
  patch_tests: {
    crate: "tabifuda-core",
    label: "patch::validateの単体テスト",
    description: "ScenarioPatchの検証(パッチ安全性/不変条件5)を受理/拒否の対で検証する。",
    strategyAnchor: "1-cratescoreテストの8割をここに",
  },
  lint_tests: {
    crate: "tabifuda-core",
    label: "シナリオlintの単体テスト",
    description: "scenario-lint.mdの検査項目ごとに、該当issueが検出される/されないことを検証する。",
    strategyAnchor: "2-シナリオデータlintをテストとして実行",
  },
  golden_tests: {
    crate: "tabifuda-core",
    label: "ワイヤ形式のゴールデンJSONテスト",
    description: "主要enum/型のJSON表現そのものを固定し、破壊的なシリアライズ変更を検出する。",
    strategyAnchor: "1-cratescoreテストの8割をここに",
  },
  invariant_tests: {
    crate: "tabifuda-core",
    label: "不変条件のプロパティテスト",
    description: "決定性・整合性・状態機械の合法性・保存則・パッチ安全性をランダム入力で検証する。",
    strategyAnchor: "1-cratescoreテストの8割をここに",
  },
  replay_tests: {
    crate: "tabifuda-core",
    label: "fixtureからのリプレイ・スナップショットテスト",
    description: "通しプレイのイベント列fixtureをリプレイし、想定の最終状態と一致することを検証する。",
    strategyAnchor: "1-cratescoreテストの8割をここに",
  },
  roundtrip_tests: {
    crate: "tabifuda-core",
    label: "全公開型のシリアライズ往復テスト",
    description: "各公開型がJSONへの変換・復元を経ても値を保つことをプロパティテストで検証する。",
    strategyAnchor: "1-cratescoreテストの8割をここに",
  },
  "chronicle::tests": {
    crate: "tabifuda-cli",
    label: "冒険記描画の単体テスト",
    description: "イベント列からCLI向け冒険記テキストへの翻訳ロジックを検証する。",
    strategyAnchor: "3-engine-cli--engine-wasm薄く",
  },
  "fork::tests": {
    crate: "tabifuda-cli",
    label: "フォーク構築の単体テスト",
    description: "セッション終了後のフォークシナリオ構築(パッチ由来dealsの統合等)を検証する。",
    strategyAnchor: "3-engine-cli--engine-wasm薄く",
  },
  "oplog::tests": {
    crate: "tabifuda-cli",
    label: "運用ログ生成の単体テスト",
    description: "運用ログが自由入力本文を記録しないことを検証する。",
    strategyAnchor: "3-engine-cli--engine-wasm薄く",
  },
  lint_cli: {
    crate: "tabifuda-cli",
    label: "lintコマンドの結合テスト",
    description: "`tabifuda-cli lint`の翻訳(ファイル読込→lint呼び出し→終了コード)を検証する。",
    strategyAnchor: "3-engine-cli--engine-wasm薄く",
  },
  play_cli: {
    crate: "tabifuda-cli",
    label: "通しプレイのCLIスモークテスト",
    description: "標準入出力を通した通しプレイで、勝利エンド到達とフォーク保存を検証する。",
    strategyAnchor: "3-engine-cli--engine-wasm薄く",
  },
  scenario_lint: {
    crate: "tabifuda-cli",
    label: "同梱シナリオ全件のlint実行",
    description: "shared/scenarios/配下の同梱テンプレシナリオ全件がlintを通過することを検証する。",
    strategyAnchor: "2-シナリオデータlintをテストとして実行",
  },
};

// cargoは「どのターゲットを実行するか(Running.../Doc-tests...)」をstderrに、
// 個々のtest結果をstdoutに出す。両者は同じ順序で1対1に並ぶため、出現順で対応付ける。
const RUNNING_RE = /^ {2,}Running (.+) \(.+\)$/;
const DOC_TESTS_RE = /^ {2,}Doc-tests /;
const RUNNING_TESTS_HEADER_RE = /^running \d+ tests?$/;
const TEST_LINE_RE = /^test (.+) \.\.\. (ok|FAILED)$/;

function suiteIdFor(runningPath, fullTestName) {
  const flat = runningPath.match(/tests[\\/](.+)\.rs$/);
  if (flat !== null) return flat[1];
  if (/unittests src[\\/](lib|main)\.rs$/.test(runningPath)) {
    const idx = fullTestName.lastIndexOf("::");
    if (idx === -1) {
      throw new Error(
        `unittestsブロックにモジュール階層の無いテストがある(想定外): ${fullTestName}`,
      );
    }
    return fullTestName.slice(0, idx);
  }
  throw new Error(`未知のcargo testターゲット(SUITES/gen-test-report.mjsの追従漏れ): ${runningPath}`);
}

function runCargoTest() {
  const result = spawnSync("cargo", ["test", "--workspace", "--no-fail-fast"], {
    cwd: repoRoot,
    encoding: "utf-8",
    maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  return { stdout: result.stdout, stderr: result.stderr };
}

/** stderrから、cargoがターゲットを実行する順序どおりに境界ラベルを抜き出す。
 * 通常ターゲットはrunningPath、Doc-testsはnull(個別テストが来たらエラーにする)。 */
function targetOrderFromStderr(stderr) {
  const targets = [];
  for (const line of stderr.split(/\r?\n/)) {
    const running = line.match(RUNNING_RE);
    if (running !== null) {
      targets.push(running[1]);
      continue;
    }
    if (DOC_TESTS_RE.test(line)) targets.push(null);
  }
  return targets;
}

function parse(stdout, stderr) {
  const targets = targetOrderFromStderr(stderr);
  const bySuite = new Map();
  let targetIndex = -1;

  for (const line of stdout.split(/\r?\n/)) {
    if (RUNNING_TESTS_HEADER_RE.test(line)) {
      targetIndex += 1;
      continue;
    }
    const testLine = line.match(TEST_LINE_RE);
    if (testLine === null) continue;
    if (targetIndex < 0 || targetIndex >= targets.length) {
      throw new Error(`test行に対応するRunning見出しが無い(cargoの出力形式が変わった可能性): ${line}`);
    }
    const runningPath = targets[targetIndex];
    if (runningPath === null) {
      throw new Error(`Doc-testsに個別テストが現れた(未対応の形): ${line}`);
    }
    const [, fullName, status] = testLine;
    const suiteId = suiteIdFor(runningPath, fullName);
    const meta = SUITES[suiteId];
    if (meta === undefined) {
      throw new Error(
        `未分類のテストスイート '${suiteId}'。tools/docs-site/scripts/gen-test-report.mjs の SUITES に追加してください。`,
      );
    }
    const name = fullName.includes("::") ? fullName.slice(fullName.lastIndexOf("::") + 2) : fullName;
    if (!bySuite.has(suiteId)) bySuite.set(suiteId, { ...meta, id: suiteId, tests: [] });
    bySuite.get(suiteId).tests.push({ name, ok: status === "ok" });
  }

  const suites = [...bySuite.values()]
    .map((s) => ({
      ...s,
      passed: s.tests.filter((t) => t.ok).length,
      failed: s.tests.filter((t) => !t.ok).length,
    }))
    .sort((a, b) => a.id.localeCompare(b.id));

  if (suites.length === 0) {
    throw new Error("cargo testの出力から1件もテストを抽出できなかった(出力形式が変わった可能性)");
  }

  return {
    generatedAt: new Date().toISOString(),
    totalPassed: suites.reduce((n, s) => n + s.passed, 0),
    totalFailed: suites.reduce((n, s) => n + s.failed, 0),
    suites,
  };
}

export function generateTestReport() {
  const { stdout, stderr } = runCargoTest();
  const report = parse(stdout, stderr);
  mkdirSync(dirname(outPath), { recursive: true });
  writeFileSync(outPath, JSON.stringify(report, null, 2) + "\n", "utf-8");
  return report;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const report = generateTestReport();
  console.log(
    `test-report.json を生成しました(${report.suites.length}スイート、` +
      `${report.totalPassed}件成功/${report.totalFailed}件失敗)`,
  );
  if (report.totalFailed > 0) process.exitCode = 1;
}
