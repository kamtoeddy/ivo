import type { ReactNode } from 'react';
import BrowserOnly from '@docusaurus/BrowserOnly';
import {
  Sandpack,
  type SandpackFiles,
  type SandpackPredefinedTemplate,
} from '@codesandbox/sandpack-react';

type TsPlaygroundProps = {
  /**
   * The `ivo` version to resolve from the npm registry, e.g. "1.9.0". Must be a
   * version that's actually published - see https://www.npmjs.com/package/ivo.
   * There is no local-bundle fallback for unpublished versions yet (v2.0.0
   * pre-release) - see Phase 3 in /docs/TODO.md.
   */
  ivoVersion: string;
  /** Contents of index.ts, run on load and on every edit. */
  code: string;
  template?: SandpackPredefinedTemplate;
};

export default function TsPlayground({
  ivoVersion,
  code,
  template = 'vanilla-ts',
}: TsPlaygroundProps): ReactNode {
  const files: SandpackFiles = {
    '/index.ts': code,
  };

  return (
    <BrowserOnly fallback={<div>Loading playground…</div>}>
      {() => (
        <Sandpack
          template={template}
          theme="auto"
          files={files}
          customSetup={{
            dependencies: {
              ivo: ivoVersion,
            },
          }}
          options={{
            editorHeight: 420,
            showConsole: true,
            showConsoleButton: true,
          }}
        />
      )}
    </BrowserOnly>
  );
}
