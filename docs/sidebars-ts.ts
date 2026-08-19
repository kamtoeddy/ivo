import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

// Sidebar for the "ts" docs plugin instance (versioned - see docusaurus.config.ts).
const sidebars: SidebarsConfig = {
  tsSidebar: [
    {
      type: "doc",
      id: "index",
      label: "Getting Started",
    },
    {
      type: "category",
      label: "Fields",
      items: [
        { type: "doc", id: "definitions/constants", label: "Constants" },
        { type: "doc", id: "definitions/dependents", label: "Dependents" },
        { type: "doc", id: "definitions/lax", label: "Lax" },
        { type: "doc", id: "definitions/required", label: "Required" },
        { type: "doc", id: "definitions/virtuals", label: "Virtuals" },
        { type: "doc", id: "definitions/timestamps", label: "Timestamps" },
      ],
    },
    {
      type: "doc",
      id: "options",
      label: "Options",
    },
    {
      type: "doc",
      id: "life-cycles",
      label: "Life Cycles",
    },
    {
      type: "doc",
      id: "validators",
      label: "Validators",
    },
    {
      type: "doc",
      id: "definitions/extend-schemas",
      label: "Extended Schemas",
    },
  ],
};

export default sidebars;
