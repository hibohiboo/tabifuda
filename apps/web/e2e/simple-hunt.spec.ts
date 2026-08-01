import { expect, test } from "@playwright/test";

// test-strategy.md「E2E/スモーク」: テンプレシナリオ「単純討伐」を1本通す。
// ルール分岐の検証はcore側で済んでいるため、ここではUI操作で最後まで
// 遷移できること(勝利エンドまで到達すること)だけを見る。
test("単純討伐を勝利まで1本通す", async ({ page }) => {
  await page.goto("/");

  await page.getByRole("button", { name: "はじめる" }).click();

  await page.getByRole("button", { name: "依頼を受ける" }).click();
  await page.getByRole("button", { name: "出す" }).click();

  await page.getByRole("button", { name: "獣の巣に到着する" }).click();
  await page.getByRole("button", { name: "打ち倒す" }).click();
  await page.getByRole("button", { name: "村に帰還を告げる" }).click();
  await page.getByRole("button", { name: "出す" }).click();

  await expect(page.getByText("勝利", { exact: true })).toBeVisible();
});
