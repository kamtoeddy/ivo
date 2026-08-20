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

  // Only show a docs version dropdown when viewing the docs plugin it belongs to.
  if (activePlugin?.pluginId !== props.docsPluginId) {
    return null;
  }

  return <DocsVersionDropdownNavbarItem {...props} />;
}
