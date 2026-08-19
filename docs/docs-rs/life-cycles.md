---
title: Life Cycles
---

# Life Cycles

`ivo` lets you react to changes on a domain entity or its individual fields. The concepts below
are shared across both implementations - see the
[root README](https://github.com/kamtoeddy/ivo#lifecycle-events) for the full language-agnostic
definitions. This page covers how to wire them up in Rust.

## onDelete

Manually triggered by invoking the `delete` method of a schema's model. Subscribe for the entire
entity via schema options, or per output field. See the
`should_properly_trigger_on_delete_handlers` and `should_properly_trigger_all_on_delete_handlers`
test functions
[here](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/opions/mod.rs).

## onFailure

Manually triggered by invoking the `handle failure` function returned from an unsuccessful create
or update operation. Subscribe on individual input fields that have at least one validator.

## onSuccess

Manually triggered by invoking the `handle success` function returned from a successful create or
update operation. Subscribe on any individual field, or for
[a group of fields via schema options](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/option_on_success.rs)
(an empty fields array subscribes to changes on the entire entity).

## Custom context options

See [Getting Started - custom context options](./index.md#custom-context-options) for how to
thread extra data (dependency injection, caching, i18n, ...) through create/update/delete
operations and into these handlers.
