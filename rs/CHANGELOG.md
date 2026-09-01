# Changelog

All notable changes to this project will be documented in this file.

# v0.6.0 <small><sup>2026-09-01</sup></small>

- Breaking change: update `IvoSuccessHandle` and `IvoFailureHandle` signatures and constructors to store `CtxOptions` directly instead of wrapping in `IvoCtxOptions`.
- Update `ivo-derive` macro output to pass unwrapped `CtxOptions` into `IvoSuccessHandle::new` and `IvoFailureHandle::new`.
