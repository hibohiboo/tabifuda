// gen-test-report.mjs が生成する src/generated/test-report.json の型。
// 生成物自体はコミットしない(ビルドの度にcargo testを実行して作り直す)。
export interface TestCase {
  name: string;
  ok: boolean;
}

export interface TestSuite {
  id: string;
  crate: string;
  label: string;
  description: string;
  strategyAnchor: string;
  passed: number;
  failed: number;
  tests: TestCase[];
}

export interface TestReport {
  generatedAt: string;
  totalPassed: number;
  totalFailed: number;
  suites: TestSuite[];
}
