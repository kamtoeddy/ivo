import { test, expect } from "@playwright/test";

test.describe("interactive playgrounds", () => {
  test("Rust playground runs the constants demo", async ({ page }) => {
    await page.goto("/docs/rs/definitions/constants");

    const runButton = page.locator("button", { hasText: "Run" }).first();
    await runButton.click({ force: true });

    const output = page.locator('[data-testid="rust-playground-output"]');
    await expect(output).toContainText('"username": "john-doe"', {
      timeout: 10_000,
    });
  });

  test("Rust playground runs the required demo", async ({ page }) => {
    await page.goto("/docs/rs/definitions/required");

    const runButton = page.locator("button", { hasText: "Run" }).first();
    await runButton.click({ force: true });

    const output = page.locator('[data-testid="rust-playground-output"]');
    await expect(output).toContainText("is required", { timeout: 10_000 });
  });

  test("TypeScript Sandpack loads and runs on a v2 page", async ({ page }) => {
    await page.goto("/docs/ts/definitions/constants");

    // Wait for the Sandpack wrapper and its console to appear.
    await page.locator(".sp-wrapper").first().waitFor({ timeout: 15_000 });
    const consolePanel = page
      .locator('.sp-console, [class*="console"]')
      .first();
    await consolePanel.waitFor({ timeout: 15_000 });

    // The constants example logs the created data object; we should see some runtime output.
    await expect(consolePanel).toContainText("id", { timeout: 15_000 });
  });
});
