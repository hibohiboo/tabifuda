import { expect, test } from "@playwright/test";

// test-strategy.md「E2E/スモーク」: テンプレシナリオ「単純討伐」を1本通す。
// ルール分岐の検証はcore側で済んでいるため、ここではUI操作で最後まで
// 遷移できること(勝利エンドまで到達すること)だけを見る。
//
// P6 C2(カードUI改善)決定: 手札のカードは全種類統一で
// 「タップ→CardLargeをモーダル展開→出す」の2段階を踏む(誤タップ防止。
// docs/design/ui-visual-design.md「画面ごとのUI方向性」)。そのため各カードを
// 名前でタップした後、モーダル内の「出す」を押す手順を挟む。
async function playCard(page: import("@playwright/test").Page, name: string) {
  await page.getByRole("button", { name, exact: true }).click();
  await page.getByRole("button", { name: "出す" }).click();
}

test("単純討伐を勝利まで1本通す", async ({ page }) => {
  await page.goto("/");

  // 依頼選択(P7 C2): 依頼カードをタップ→ScenarioCardLargeを確認→
  // 「これで始める」で選択確定(Hand.tsxと同じタップ→展開→確定の2段階)。
  // shared/scenarios/にはCLIフォーク出力のデモ用サンプル
  // (simple-hunt-fork.json)も同名タイトル「単純討伐」で存在するため
  // `.first()`(id順ソートの先頭=simple-hunt)で一意に選ぶ。
  await page.getByRole("button", { name: "単純討伐", exact: true }).first().click();
  await page.getByRole("button", { name: "これで始める" }).click();

  await playCard(page, "依頼を受ける");

  await playCard(page, "獣の巣に到着する");
  await playCard(page, "打ち倒す");
  await playCard(page, "村に帰還を告げる");

  await expect(page.getByText("勝利", { exact: true })).toBeVisible();
});
