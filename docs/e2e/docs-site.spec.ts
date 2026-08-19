import { test, expect } from "@playwright/test";

test.describe("ivo docs site", () => {
  test("home page loads", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle(/ivo/);
  });

  test.describe("TypeScript docs", () => {
    test("default landing version is v2.0.0", async ({ page }) => {
      await page.goto("/docs/ts");
      await expect(page.locator("text=Getting Started").first()).toBeVisible();
      await expect(page.locator("text=2.0.0").first()).toBeVisible();
    });

    test("v1.9.0 archive is accessible", async ({ page }) => {
      await page.goto("/docs/ts/1.9.0");
      await expect(
        page.locator("text=Defining a schema").first(),
      ).toBeVisible();
      await expect(page.locator("text=1.9.0").first()).toBeVisible();
    });

    test("French translation of v2.0.0 works", async ({ page }) => {
      await page.goto("/fr/docs/ts");
      await expect(page.locator("text=Premiers pas").first()).toBeVisible();
      await expect(page.locator("text=2.0.0").first()).toBeVisible();
    });

    test("French translation of v1.9.0 archive does not 404", async ({
      page,
    }) => {
      await page.goto("/fr/docs/ts/1.9.0");
      await expect(
        page.locator("text=Définir un schéma").first(),
      ).toBeVisible();
      await expect(
        page.locator("text=Page Not Found").first(),
      ).not.toBeVisible();
    });

    test("language switcher navigates between locales on v2.0.0", async ({
      page,
    }) => {
      await page.goto("/docs/ts");
      await page.getByRole("button", { name: /english/i }).click();
      await page.getByRole("link", { name: /français/i }).click();
      await expect(page).toHaveURL(/\/fr\/docs\/ts\/?$/);
      await expect(page.locator("text=Premiers pas").first()).toBeVisible();
    });

    test("language switcher navigates between locales on v1.9.0", async ({
      page,
    }) => {
      await page.goto("/docs/ts/1.9.0");
      await page.getByRole("button", { name: /english/i }).click();
      await page.getByRole("link", { name: /français/i }).click();
      await expect(page).toHaveURL(/\/fr\/docs\/ts\/1\.9\.0\/?$/);
      await expect(
        page.locator("text=Définir un schéma").first(),
      ).toBeVisible();
    });

    test("interactive playground renders on a v2.0.0 page", async ({
      page,
    }) => {
      await page.goto("/docs/ts/definitions/constants");
      await page.locator(".sp-wrapper").first().waitFor({ timeout: 10_000 });
    });
  });

  test.describe("Rust docs", () => {
    test("Rust docs load", async ({ page }) => {
      await page.goto("/docs/rs");
      await expect(page.locator("text=Rust").first()).toBeVisible();
    });

    test("TS version dropdown is not shown on Rust docs", async ({ page }) => {
      await page.goto("/docs/rs");
      await expect(
        page.locator('.navbar__item:has-text("2.0.0")').first(),
      ).not.toBeVisible();
      await expect(
        page.locator('.navbar__item:has-text("1.9.0")').first(),
      ).not.toBeVisible();
    });
  });
});
