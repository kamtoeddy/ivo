import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

// Sidebar for the "rs" docs plugin instance (unversioned - latest API only).
const sidebars: SidebarsConfig = {
  rsSidebar: [
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
        { type: "doc", id: "definitions/timestamps", label: "Timestamps" },
        { type: "doc", id: "definitions/virtuals", label: "Virtuals" },
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
  ],
};

export default sidebars;
