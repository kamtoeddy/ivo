import { useState, type ReactNode } from "react";
import BrowserOnly from "@docusaurus/BrowserOnly";
import CodeBlock from "@theme/CodeBlock";
import styles from "./styles.module.css";

import constantsSource from "./sources/constants.rs";
import laxDefaultsSource from "./sources/lax_defaults.rs";
import requiredSource from "./sources/required.rs";
import virtualsSource from "./sources/virtuals.rs";
import dependentsSource from "./sources/dependents.rs";
import timestampsSource from "./sources/timestamps.rs";

// One entry per wasm-exported demo (see docs/wasm/ivo-playground/src/lib.rs).
// Adding a new curated rs/examples/*.rs demo means adding both the wasm export
// and an entry here - see docs/TODO.md Phase 4.
const DEMOS = {
  constants: {
    label: "Constant fields",
    wasmFn: "constantsCreate",
    defaultInput: '{\n  "username": "john-doe"\n}',
    source: constantsSource,
  },
  lax_defaults: {
    label: "Lax fields (defaults)",
    wasmFn: "laxDefaultsCreate",
    defaultInput: "{}",
    source: laxDefaultsSource,
  },
  required: {
    label: "Required fields",
    wasmFn: "requiredCreate",
    defaultInput: "{}",
    source: requiredSource,
  },
  virtuals: {
    label: "Virtual fields",
    wasmFn: "virtualsCreate",
    defaultInput: '{\n  "virtual_field": "hello"\n}',
    source: virtualsSource,
  },
  dependents: {
    label: "Dependent fields",
    wasmFn: "dependentsCreate",
    defaultInput: '{\n  "value": 10\n}',
    source: dependentsSource,
  },
  timestamps: {
    label: "Timestamps",
    wasmFn: "timestampsCreate",
    defaultInput: '{\n  "username": "john-doe"\n}',
    source: timestampsSource,
  },
} as const;

type DemoKey = keyof typeof DEMOS;

type WasmModule = Record<string, (inputJson: string) => Promise<string>>;

let wasmModulePromise: Promise<WasmModule> | null = null;

function loadWasmModule(): Promise<WasmModule> {
  if (!wasmModulePromise) {
    wasmModulePromise = (async () => {
      // Built by scripts/build-rust-wasm.sh - not present until that script has run.
      const mod =
        await import("@site/static/wasm/ivo-playground/ivo_playground.js");
      await mod.default();
      return mod as unknown as WasmModule;
    })();
  }
  return wasmModulePromise;
}

type RustPlaygroundProps = {
  demo: DemoKey;
};

function RustPlaygroundImpl({ demo }: RustPlaygroundProps): ReactNode {
  const config = DEMOS[demo];
  const [input, setInput] = useState<string>(config.defaultInput);
  const [output, setOutput] = useState<string>("");
  const [running, setRunning] = useState(false);

  async function run() {
    setRunning(true);
    try {
      const mod = await loadWasmModule();
      const result = await mod[config.wasmFn](input);
      setOutput(JSON.stringify(JSON.parse(result), null, 2));
    } catch (e) {
      setOutput(`error: ${e instanceof Error ? e.message : String(e)}`);
    } finally {
      setRunning(false);
    }
  }

  return (
    <div>
      <CodeBlock language="rust" title="Schema / model code">
        {config.source}
      </CodeBlock>
      <div className={styles.playground}>
        <div className={styles.pane}>
          <label
            className={styles.label}
            htmlFor={`rust-playground-input-${demo}`}
          >
            Input (JSON)
          </label>
          <textarea
            id={`rust-playground-input-${demo}`}
            className={styles.textarea}
            value={input}
            onChange={(e) => setInput(e.target.value)}
            spellCheck={false}
          />
        </div>
        <div className={styles.pane}>
          <div className={styles.outputHeader}>
            <span className={styles.label}>Output</span>
            <button
              className="button button--primary button--sm"
              onClick={run}
              disabled={running}
            >
              {running ? "Running…" : "Run"}
            </button>
          </div>
          <pre className={styles.output} data-testid="rust-playground-output">
            {output || "// click Run"}
          </pre>
        </div>
      </div>
    </div>
  );
}

export default function RustPlayground(props: RustPlaygroundProps): ReactNode {
  return (
    <BrowserOnly fallback={<div>Loading playground…</div>}>
      {() => <RustPlaygroundImpl {...props} />}
    </BrowserOnly>
  );
}
