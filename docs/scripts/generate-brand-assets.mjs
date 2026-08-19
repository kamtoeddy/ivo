#!/usr/bin/env node
/**
 * Generate the ivo brand assets from the logo text "i√o".
 *
 * Outputs:
 *   - static/img/logo.svg
 *   - static/img/logo.png (hi-res source for derived icons)
 *   - static/img/social-card.svg
 *   - static/img/social-card.png
 *   - static/img/icons/*.png (favicon/touch/PWA sizes)
 */
import { chromium } from "@playwright/test";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { execSync } from "node:child_process";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, "..");
const STATIC = resolve(ROOT, "static");
const IMG = resolve(STATIC, "img");
const ICONS = resolve(IMG, "icons");

const DARK = "#0a0a0a";
const LIGHT = "#ffffff";
const FONT =
  'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace';

function logoSvg({ width, height, fontSize }) {
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">
  <rect width="100%" height="100%" fill="${DARK}" />
  <text
    x="50%"
    y="50%"
    dominant-baseline="central"
    text-anchor="middle"
    fill="${LIGHT}"
    font-family='${FONT}'
    font-size="${fontSize}"
    font-weight="500"
    letter-spacing="-0.05em"
  >i√o</text>
</svg>`;
}

function socialCardSvg({ width, height }) {
  const logoSize = Math.round(height * 0.22);
  const taglineSize = Math.round(height * 0.045);
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">
  <rect width="100%" height="100%" fill="${DARK}" />
  <text
    x="50%"
    y="${Math.round(height * 0.44)}"
    dominant-baseline="central"
    text-anchor="middle"
    fill="${LIGHT}"
    font-family='${FONT}'
    font-size="${logoSize}"
    font-weight="500"
    letter-spacing="-0.05em"
  >i√o</text>
  <text
    x="50%"
    y="${Math.round(height * 0.62)}"
    dominant-baseline="central"
    text-anchor="middle"
    fill="${LIGHT}"
    font-family="system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, sans-serif"
    font-size="${taglineSize}"
    font-weight="400"
    opacity="0.9"
  >The schema validator that brings user stories to life</text>
</svg>`;
}

function logoHtml({ width, height, fontSize }) {
  return `<!DOCTYPE html>
<html>
  <head>
    <style>
      * { margin: 0; padding: 0; box-sizing: border-box; }
      body {
        width: ${width}px;
        height: ${height}px;
        background: ${DARK};
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
      }
      .logo {
        color: ${LIGHT};
        font-family: ${FONT};
        font-size: ${fontSize}px;
        font-weight: 500;
        letter-spacing: -0.05em;
        white-space: nowrap;
        line-height: 1;
      }
    </style>
  </head>
  <body><div class="logo">i√o</div></body>
</html>`;
}

function socialCardHtml({ width, height }) {
  const logoSize = Math.round(height * 0.22);
  const taglineSize = Math.round(height * 0.045);
  return `<!DOCTYPE html>
<html>
  <head>
    <style>
      * { margin: 0; padding: 0; box-sizing: border-box; }
      body {
        width: ${width}px;
        height: ${height}px;
        background: ${DARK};
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        overflow: hidden;
        color: ${LIGHT};
        text-align: center;
      }
      .logo {
        font-family: ${FONT};
        font-size: ${logoSize}px;
        font-weight: 500;
        letter-spacing: -0.05em;
        line-height: 1;
        margin-bottom: ${Math.round(height * 0.04)}px;
      }
      .tagline {
        font-family: system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, sans-serif;
        font-size: ${taglineSize}px;
        font-weight: 400;
        opacity: 0.9;
      }
    </style>
  </head>
  <body>
    <div class="logo">i√o</div>
    <div class="tagline">The schema validator that brings user stories to life</div>
  </body>
</html>`;
}

async function render(page, { width, height, html, outPath }) {
  await page.setViewportSize({ width, height });
  await page.setContent(html, { waitUntil: "networkidle" });
  await page.screenshot({ path: outPath, type: "png" });
}

async function main() {
  // SVG assets
  writeFileSync(
    resolve(IMG, "logo.svg"),
    logoSvg({ width: 376, height: 312, fontSize: 160 }),
  );
  writeFileSync(
    resolve(IMG, "social-card.svg"),
    socialCardSvg({ width: 1200, height: 630 }),
  );

  const browser = await chromium.launch();
  const page = await browser.newPage();

  // PNG assets
  await render(page, {
    width: 376,
    height: 312,
    html: logoHtml({ width: 376, height: 312, fontSize: 160 }),
    outPath: resolve(IMG, "logo.png"),
  });

  await render(page, {
    width: 1200,
    height: 630,
    html: socialCardHtml({ width: 1200, height: 630 }),
    outPath: resolve(IMG, "social-card.png"),
  });

  // Icon sizes rendered square so they are not distorted.
  const iconSizes = [16, 32, 180, 192, 512];
  for (const size of iconSizes) {
    const fontSize = Math.round(size * 0.55);
    await render(page, {
      width: size,
      height: size,
      html: logoHtml({ width: size, height: size, fontSize }),
      outPath: resolve(ICONS, `icon-${size}.png`),
    });
  }

  await browser.close();

  // Keep the classic favicon filenames that Docusaurus / browsers expect.
  execSync("mv icon-16.png favicon-16x16.png", { cwd: ICONS });
  execSync("mv icon-32.png favicon-32x32.png", { cwd: ICONS });
  execSync("mv icon-180.png apple-touch-icon.png", { cwd: ICONS });
  execSync("mv icon-192.png android-chrome-192x192.png", { cwd: ICONS });
  execSync("mv icon-512.png android-chrome-512x512.png", { cwd: ICONS });

  console.log("Brand assets generated.");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
