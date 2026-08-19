import React, { type ReactNode } from "react";
import { useActivePlugin } from "@docusaurus/plugin-content-docs/client";
import DocsVersionDropdownNavbarItem from "@theme-original/NavbarItem/DocsVersionDropdownNavbarItem";
import type DocsVersionDropdownNavbarItemType from "@theme/NavbarItem/DocsVersionDropdownNavbarItem";
import type { WrapperProps } from "@docusaurus/types";

type Props = WrapperProps<typeof DocsVersionDropdownNavbarItemType>;

export default function DocsVersionDropdownNavbarItemWrapper(
  props: Props,
): ReactNode {
  const activePlugin = useActivePlugin();

  // Only show the TS version dropdown when viewing the TS docs plugin.
  if (activePlugin?.pluginId !== "ts") {
    return null;
  }

  return <DocsVersionDropdownNavbarItem {...props} />;
}
