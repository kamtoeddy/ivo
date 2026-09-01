---
title: Constant Fields
---

# Constant Fields

A constant is a purely output field whose value should never change after creation (e.g. `id`).

- It must have either a static value or a resolver.
- It may have [`on_delete` and `on_success`](../life-cycles.md) event handlers.

## Example

- [Static & dynamic values](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/constants.rs)

## Try it in the browser

`id` is a constant (always `1234`); `username` is lax with a default. Edit the input and run it.

<RustPlayground demo="constants" />
