# Project Rules

- When running package scripts, use `bun <command>` instead of `bun run <command>`.
- Use `bun build:dev` to build the TypeScript package (since `bun build` is a Bun built-in command).
- Avoid using `any` in TypeScript code and test files; always use precise types (e.g. `ReadonlyIvoSummary`, `IvoSummary`, `ReadonlyIvoContext`, `IvoContext`, or appropriate generics) from the library.

