import { readFile, writeFile } from "node:fs/promises";
import { glob } from "node:fs/promises";

const patterns = process.argv.slice(2).length
  ? process.argv.slice(2)
  : [
      "docs-ts/**/*.md",
      "i18n/fr/docusaurus-plugin-content-docs-ts/current/**/*.md",
    ];

for (const pattern of patterns) {
  const files = await Array.fromAsync(
    glob(pattern, { cwd: new URL("..", import.meta.url) }),
  );

  for (const file of files) {
    const path = new URL(`../${file}`, import.meta.url);
    let content = await readFile(path, "utf-8");

    content = content.replace(/code=\{`([\s\S]*?)`\}/g, (match, code) => {
      if (code.includes("async function main() {")) {
        return match;
      }

      const lines = code.split("\n");
      const importLines = [];
      const bodyLines = [];

      for (const line of lines) {
        if (line.trimStart().startsWith("import ")) {
          importLines.push(line);
        } else {
          bodyLines.push(line);
        }
      }

      while (bodyLines.length && bodyLines[0].trim() === "") {
        bodyLines.shift();
      }

      const indentedBody = bodyLines
        .map((line) => (line.length ? `  ${line}` : line))
        .join("\n");

      const wrapped = [
        ...importLines,
        "",
        "async function main() {",
        indentedBody,
      ].join("\n");

      return `code={\`${wrapped}\n}\n\nmain();\`}`;
    });

    await writeFile(path, content);
  }

  console.log(`Updated ${files.length} files for ${pattern}.`);
}
