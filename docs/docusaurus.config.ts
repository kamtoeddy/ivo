import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";
import type { Options as DocsPluginOptions } from "@docusaurus/plugin-content-docs";

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const organizationName = "kamtoeddy";
const projectName = "ivo";
const repoUrl = `https://github.com/${organizationName}/${projectName}`;

// Allow importing Rust source files as raw strings in the RustPlayground
// component so readers can see the schema/model code behind each demo.
function rawRsSourcePlugin() {
  return {
    name: "raw-rs-source-plugin",
    configureWebpack() {
      return {
        module: {
          rules: [{ test: /\.rs$/, type: "asset/source" }],
        },
      };
    },
  };
}

const config: Config = {
  title: "ivo",
  tagline: "The schema validator that brings user stories to life",
  favicon: "img/favicon.png",

  future: {
    v4: true, // Improve compatibility with the upcoming Docusaurus v4
  },

  // TODO(phase 7): replace with the real Cloudflare Pages URL once the project is created
  url: "https://ivo.pages.dev",
  baseUrl: "/",

  organizationName,
  projectName,

  onBrokenLinks: "throw",
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: "warn",
    },
  },

  i18n: {
    defaultLocale: "en",
    locales: ["en", "fr"],
  },

  presets: [
    [
      "classic",
      {
        // Docs are served by two dedicated plugin instances below (ts + rs)
        // instead of the preset's default single docs instance.
        docs: false,
        blog: false,
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  plugins: [
    [
      "@docusaurus/plugin-content-docs",
      {
        id: "ts",
        path: "docs-ts",
        routeBasePath: "docs/ts",
        sidebarPath: "./sidebars-ts.ts",
        editUrl: `${repoUrl}/tree/main/docs/docs-ts/`,
        includeCurrentVersion: true,
        // v2.0.0 (docs-ts/, the "current" version) isn't released yet - keep the
        // latest *released* version as the default landing point, and expose the
        // in-progress v2.0.0 docs under /docs/ts/next/ until it ships (see
        // Phase 6 in TODO.md).
        lastVersion: "current",
        versions: {
          current: {
            label: "2.0.0",
            path: "",
          },
          "1.9.0": {
            label: "1.9.0",
            path: "1.9.0",
          },
        },
      } satisfies DocsPluginOptions,
    ],
    [
      "@docusaurus/plugin-content-docs",
      {
        id: "rs",
        path: "docs-rs",
        routeBasePath: "docs/rs",
        sidebarPath: "./sidebars-rs.ts",
        editUrl: `${repoUrl}/tree/main/docs/docs-rs/`,
        // Rust only ever documents the latest API - no version history to
        // preserve, so no ts_versioned_docs-style folder exists here and this
        // plugin instance is unversioned by default.
      } satisfies DocsPluginOptions,
    ],
    rawRsSourcePlugin,
  ],

  themeConfig: {
    image: "img/social-card.png",
    colorMode: {
      defaultMode: "light",
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: "ivo",
      logo: {
        alt: "ivo logo",
        src: "img/logo.svg",
        srcDark: "img/logo.svg",
      },
      items: [
        {
          type: "docSidebar",
          docsPluginId: "ts",
          sidebarId: "tsSidebar",
          position: "left",
          label: "TypeScript",
        },
        {
          type: "docSidebar",
          docsPluginId: "rs",
          sidebarId: "rsSidebar",
          position: "left",
          label: "Rust",
        },
        {
          type: "docsVersionDropdown",
          docsPluginId: "ts",
          position: "right",
        },
        {
          type: "localeDropdown",
          position: "right",
        },
        {
          href: repoUrl,
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            { label: "TypeScript", to: "/docs/ts" },
            { label: "Rust", to: "/docs/rs" },
          ],
        },
        {
          title: "More",
          items: [
            { label: "GitHub", href: repoUrl },
            { label: "Issues", href: `${repoUrl}/issues` },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} kamtoeddy. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ["rust", "bash", "json"],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
