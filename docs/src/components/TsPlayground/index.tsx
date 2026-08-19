import { useEffect, useState, type ReactNode } from "react";
import BrowserOnly from "@docusaurus/BrowserOnly";
import useBaseUrl from "@docusaurus/useBaseUrl";
import {
  Sandpack,
  type SandpackFiles,
  type SandpackPredefinedTemplate,
} from "@codesandbox/sandpack-react";

type TsPlaygroundProps = {
  /**
   * The `ivo` version to resolve from the npm registry, e.g. "1.9.0". Use
   * `"local"` to run against the locally-built v2.0.0 bundle served from
   * `/ivo-2.0.0/index.js`.
   */
  ivoVersion: string;
  /** Contents of index.ts, run on load and on every edit. */
  code: string;
  template?: SandpackPredefinedTemplate;
};

export default function TsPlayground(props: TsPlaygroundProps): ReactNode {
  return (
    <BrowserOnly fallback={<div>Loading playground…</div>}>
      {() =>
        props.ivoVersion === "local" ? (
          <LocalTsPlayground {...props} />
        ) : (
          <SandpackTsPlayground {...props} />
        )
      }
    </BrowserOnly>
  );
}

function SandpackTsPlayground({
  ivoVersion,
  code,
  template = "vanilla-ts",
}: TsPlaygroundProps): ReactNode {
  const files: SandpackFiles = {
    "/index.ts": code,
  };

  return (
    <Sandpack
      template={template}
      theme="auto"
      files={files}
      customSetup={{ dependencies: { ivo: ivoVersion } }}
      options={{
        editorHeight: 420,
        showConsole: true,
        showConsoleButton: true,
      }}
    />
  );
}

function LocalTsPlayground({
  code,
  template = "vanilla-ts",
}: TsPlaygroundProps): ReactNode {
  const bundleUrl = useBaseUrl("/ivo-2.0.0/index.js");
  const [bundle, setBundle] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch(bundleUrl)
      .then((res) => {
        if (!res.ok) {
          throw new Error(`Failed to load local bundle: ${res.status}`);
        }
        return res.text();
      })
      .then(setBundle)
      .catch((err) =>
        setError(err instanceof Error ? err.message : String(err)),
      );
  }, [bundleUrl]);

  if (error) {
    return (
      <div className="ts-playground-error">
        Error loading playground: {error}
      </div>
    );
  }

  if (!bundle) {
    return <div>Loading playground…</div>;
  }

  const files: SandpackFiles = {
    "/index.ts": code,
    "/node_modules/ivo/package.json": {
      code: JSON.stringify({
        name: "ivo",
        version: "2.0.0-local",
        type: "module",
        main: "./index.js",
        module: "./index.js",
      }),
      hidden: true,
    },
    "/node_modules/ivo/index.js": {
      code: bundle,
      hidden: true,
    },
  };

  return (
    <Sandpack
      template={template}
      theme="auto"
      files={files}
      options={{
        editorHeight: 420,
        showConsole: true,
        showConsoleButton: true,
      }}
    />
  );
}
