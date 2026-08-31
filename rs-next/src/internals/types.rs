use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Error handling

pub type DefaultFieldErrorMetadata = ();
pub type DefaultErrorPayload = IvoErrorPayload<DefaultFieldErrorMetadata>;

#[derive(Debug, Clone)]
pub struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<Metadata>,
}

pub type IvoErrorPayload<Metadata> = HashMap<String, FieldError<Metadata>>;

pub async fn run_resolver<T, Ctx, Opts, F>(ctx: Ctx, opts: &Opts, resolver: F) -> T
where
    F: AsyncFn(Ctx, &Opts) -> T,
{
    resolver(ctx, opts).await
}

pub async fn run_sanitizer<T, Ctx, Opts, F>(value: T, ctx: &Ctx, opts: &Opts, sanitizer: F) -> T
where
    F: AsyncFn(T, &Ctx, &Opts) -> T,
{
    sanitizer(value, ctx, opts).await
}

pub async fn run_validator<T, Ctx, Opts, F, Metadata>(
    value: T,
    ctx: &Ctx,
    opts: &Opts,
    validator: F,
) -> Result<Option<T>, FieldError<Metadata>>
where
    Metadata: Clone,
    F: AsyncFn(T, &Ctx, &Opts) -> Result<Option<T>, (String, Option<Metadata>)>,
{
    match validator(value, ctx, opts).await {
        ::core::result::Result::Ok(v) => ::core::result::Result::Ok(v),
        ::core::result::Result::Err((reason, metadata)) => {
            ::core::result::Result::Err(FieldError { reason, metadata })
        }
    }
}

pub async fn run_boolean_resolver<Ctx, Opts, F>(ctx: &Ctx, opts: &Opts, resolver: F) -> bool
where
    F: AsyncFn(&Ctx, &Opts) -> bool,
{
    resolver(ctx, opts).await
}

pub async fn run_required_resolver<Ctx, Opts, F>(
    ctx: &Ctx,
    opts: &Opts,
    resolver: F,
) -> Option<String>
where
    F: AsyncFn(&Ctx, &Opts) -> Option<String>,
{
    resolver(ctx, opts).await
}

pub async fn run_grouped_required_resolver<Ctx, Opts, E, F>(
    ctx: &Ctx,
    opts: &Opts,
    resolver: F,
) -> Option<E>
where
    F: AsyncFn(&Ctx, &Opts) -> Option<E>,
{
    resolver(ctx, opts).await
}

pub async fn run_hook<Ctx, Opts, F>(ctx: Ctx, opts: &Opts, handler: F)
where
    F: AsyncFn(Ctx, &Opts),
{
    handler(ctx, opts).await
}

// Synchronous counterparts used when a generated method has no async handlers.

pub fn run_resolver_sync<T, Ctx, Opts, F>(ctx: Ctx, opts: &Opts, resolver: F) -> T
where
    F: Fn(Ctx, &Opts) -> T,
{
    resolver(ctx, opts)
}

pub fn run_sanitizer_sync<T, Ctx, Opts, F>(value: T, ctx: &Ctx, opts: &Opts, sanitizer: F) -> T
where
    F: Fn(T, &Ctx, &Opts) -> T,
{
    sanitizer(value, ctx, opts)
}

pub fn run_validator_sync<T, Ctx, Opts, F, Metadata>(
    value: T,
    ctx: &Ctx,
    opts: &Opts,
    validator: F,
) -> Result<Option<T>, FieldError<Metadata>>
where
    Metadata: Clone,
    F: Fn(T, &Ctx, &Opts) -> Result<Option<T>, (String, Option<Metadata>)>,
{
    match validator(value, ctx, opts) {
        ::core::result::Result::Ok(v) => ::core::result::Result::Ok(v),
        ::core::result::Result::Err((reason, metadata)) => {
            ::core::result::Result::Err(FieldError { reason, metadata })
        }
    }
}

pub fn run_boolean_resolver_sync<Ctx, Opts, F>(ctx: &Ctx, opts: &Opts, resolver: F) -> bool
where
    F: Fn(&Ctx, &Opts) -> bool,
{
    resolver(ctx, opts)
}

pub fn run_required_resolver_sync<Ctx, Opts, F>(
    ctx: &Ctx,
    opts: &Opts,
    resolver: F,
) -> Option<String>
where
    F: Fn(&Ctx, &Opts) -> Option<String>,
{
    resolver(ctx, opts)
}

pub fn run_grouped_required_resolver_sync<Ctx, Opts, E, F>(
    ctx: &Ctx,
    opts: &Opts,
    resolver: F,
) -> Option<E>
where
    F: Fn(&Ctx, &Opts) -> Option<E>,
{
    resolver(ctx, opts)
}

pub fn run_hook_sync<Ctx, Opts, F>(ctx: Ctx, opts: &Opts, handler: F)
where
    F: Fn(Ctx, &Opts),
{
    handler(ctx, opts)
}

pub fn run_callback_sync<F>(handler: F)
where
    F: Fn(),
{
    handler()
}

pub fn run_value_resolver_sync<T, F>(resolver: F) -> T
where
    F: Fn() -> T,
{
    resolver()
}

pub async fn run_post_validator<Ctx, Opts, F, Partial, Errors>(
    ctx: Ctx,
    opts: Opts,
    validator: F,
) -> Result<Option<Partial>, Errors>
where
    F: AsyncFn(Ctx, Opts) -> Result<Option<Partial>, Errors>,
{
    validator(ctx, opts).await
}

pub fn run_post_validator_sync<Ctx, Opts, F, Partial, Errors>(
    ctx: Ctx,
    opts: Opts,
    validator: F,
) -> Result<Option<Partial>, Errors>
where
    F: Fn(Ctx, Opts) -> Result<Option<Partial>, Errors>,
{
    validator(ctx, opts)
}

pub trait IvoErrorSanitizer<CtxOptions> {
    type Metadata: Clone + Send + Sync;
    type Payload;

    fn sanitize(
        payload: IvoErrorPayload<Self::Metadata>,
        ctx_options: &CtxOptions,
    ) -> Self::Payload;
}

#[derive(Debug)]
pub struct DefaultErrorSanitizer<Metadata: Clone = DefaultFieldErrorMetadata> {
    _marker: std::marker::PhantomData<Metadata>,
}

impl<CtxOptions, Metadata: Clone + Send + Sync> IvoErrorSanitizer<CtxOptions>
    for DefaultErrorSanitizer<Metadata>
{
    type Metadata = Metadata;
    type Payload = IvoErrorPayload<Self::Metadata>;

    fn sanitize(payload: IvoErrorPayload<Self::Metadata>, _: &CtxOptions) -> Self::Payload {
        payload
    }
}

// Struct traits (filled in by derive macros)

pub trait WithPartialStruct {
    type Partial: Default + Clone + Send + Sync + 'static;
}

pub trait WithPartialErrors<Metadata: Send + Sync> {
    type PartialErrors: Send + Sync;
}

pub trait IvoStruct: WithPartialStruct + Into<Self::Partial> + Send + Sync + 'static {
    fn append_updates(&mut self, updates: &Self::Partial);
}

pub trait IvoInputStruct<CtxOptions, ErrorSanitizer: IvoErrorSanitizer<CtxOptions>>:
    IvoStruct + WithPartialErrors<ErrorSanitizer::Metadata>
{
}

// Context types

#[derive(Clone)]
pub struct IvoContext<I, O: WithPartialStruct> {
    input: I,
    raw_input: I,
    values: O,
    changes: Option<O::Partial>,
    previous_values: Option<O>,
}

impl<I, O: WithPartialStruct> IvoContext<I, O> {
    /// `raw_input` is the schema's original partial input exactly as the
    /// caller passed it to `create`/`update`, captured once up front and
    /// never mutated afterward -- distinct from `input`, which evolves as
    /// the pipeline runs (ignored fields cleared, validated/re-validated/
    /// sanitized values substituted in, `post_validate` updates merged in).
    ///
    /// `changes` and `previous_values` are only meaningful during an update
    /// (there's no prior record and no diff-from-prior-record at creation),
    /// so both are `None` on `create` and `Some` on `update` -- `is_update()`
    /// is derived from `previous_values`'s presence rather than tracked
    /// separately, so the two can't drift out of sync.
    pub fn new(
        input: I,
        raw_input: I,
        values: O,
        changes: Option<O::Partial>,
        previous_values: Option<O>,
    ) -> Self {
        Self {
            input,
            raw_input,
            values,
            changes,
            previous_values,
        }
    }

    pub fn input(&self) -> &I {
        &self.input
    }

    pub fn raw_input(&self) -> &I {
        &self.raw_input
    }

    /// The full, most-up-to-date set of output values, at creation or during
    /// an update (includes whatever this pipeline run has resolved so far).
    pub fn values(&self) -> &O {
        &self.values
    }

    /// The subset of output values changed by this update. `None` at
    /// creation (everything is new there, so "changed" is meaningless).
    pub fn changes(&self) -> Option<&O::Partial> {
        self.changes.as_ref()
    }

    /// The record as it was *before* this update was applied. `None` at
    /// creation (there is no prior record).
    pub fn previous_values(&self) -> Option<&O> {
        self.previous_values.as_ref()
    }

    pub fn is_update(&self) -> bool {
        self.previous_values.is_some()
    }
}

// Options wrappers

pub struct IvoCtxOptions<CtxOptions>(pub std::sync::Arc<async_lock::RwLock<CtxOptions>>);

impl<CtxOptions> IvoCtxOptions<CtxOptions> {
    pub fn new(rw: &IvoRwCtxOptions<CtxOptions>) -> Self {
        Self(rw.0.clone())
    }

    /// Read the options, `.await`ing if another guard is currently held.
    ///
    /// Use this from an **async** lifecycle hook (`on_success`/`on_failure`/
    /// `on_delete`). For a **sync** hook, use [`Self::read_sync`] instead --
    /// calling this and never polling the returned future does nothing
    /// useful, and blocking on it defeats the point of the async form.
    pub fn read(&self) -> impl Future<Output = async_lock::RwLockReadGuard<'_, CtxOptions>> + '_ {
        self.0.read()
    }

    /// Read the options, blocking the current thread if another guard is
    /// currently held.
    ///
    /// Use this from a **sync** lifecycle hook. Do not call it from inside
    /// an `async` block/future you're not immediately blocking on to
    /// completion -- parking the executor thread while other tasks on that
    /// same thread still need to run is the standard async-blocking
    /// footgun, and can stall or deadlock a single-threaded/limited-worker
    /// runtime. Safe to call here specifically because hooks are read-only
    /// (there's no writer this could ever contend with within `ivo`'s own
    /// generated pipeline -- see [`IvoRwCtxOptions::read_sync`] for the
    /// fuller version of this warning, which does apply to writers).
    pub fn read_sync(&self) -> async_lock::RwLockReadGuard<'_, CtxOptions> {
        self.0.read_blocking()
    }
}

impl<CtxOptions> Clone for IvoCtxOptions<CtxOptions> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct IvoRwCtxOptions<CtxOptions>(pub std::sync::Arc<async_lock::RwLock<CtxOptions>>);

impl<CtxOptions> IvoRwCtxOptions<CtxOptions> {
    pub fn new(opts: CtxOptions) -> Self {
        Self(std::sync::Arc::new(async_lock::RwLock::new(opts)))
    }

    /// Read the options, `.await`ing if a writer currently holds the lock.
    /// Any number of readers (sync or async) may hold it at once.
    ///
    /// Use this from an **async** validator/re-validator/sanitizer/resolver.
    /// For a **sync** handler, use [`Self::read_sync`] instead.
    pub fn read(&self) -> impl Future<Output = async_lock::RwLockReadGuard<'_, CtxOptions>> + '_ {
        self.0.read()
    }

    /// Write the options, `.await`ing until every other reader/writer has
    /// released. Excludes every other reader and writer while the returned
    /// guard is held.
    ///
    /// Use this from an **async** validator/re-validator/sanitizer/resolver.
    /// For a **sync** handler, use [`Self::write_sync`] instead.
    pub fn write(&self) -> impl Future<Output = async_lock::RwLockWriteGuard<'_, CtxOptions>> + '_ {
        self.0.write()
    }

    /// Read the options, blocking the current thread if a writer currently
    /// holds the lock.
    ///
    /// Use this from a **sync** validator/re-validator/sanitizer/resolver.
    ///
    /// **Do not** call this (or [`Self::write_sync`]) from inside a task you
    /// spawn yourself (e.g. `tokio::spawn`) that runs independently of the
    /// handler that spawned it. Within a single `create`/`update` call,
    /// `ivo`'s generated pipeline guarantees every sync handler in a phase
    /// runs to completion *before* that phase's async handlers are polled
    /// (see `emit_async_phase` in `crates/derive/src/lib.rs`), so a sync
    /// `read_sync`/`write_sync` call here is always racing against an
    /// uncontended lock. That guarantee only covers concurrency `ivo`
    /// itself orchestrates -- a task you spawn independently is no longer
    /// sequenced by it, and blocking the executor thread on a guard held by
    /// such a task (or that such a task is waiting to acquire) is the usual
    /// async-Rust footgun: it can stall or deadlock a single-threaded or
    /// limited-worker runtime. Use [`Self::read`] there instead, or
    /// `spawn_blocking` if you must combine both.
    pub fn read_sync(&self) -> async_lock::RwLockReadGuard<'_, CtxOptions> {
        self.0.read_blocking()
    }

    /// Write the options, blocking the current thread until every other
    /// reader/writer has released. Excludes every other reader and writer
    /// while the returned guard is held.
    ///
    /// Use this from a **sync** validator/re-validator/sanitizer/resolver.
    /// Same caveat as [`Self::read_sync`] about independently-spawned tasks
    /// -- it applies here too, and matters more for writers, since a stuck
    /// writer also blocks every reader.
    pub fn write_sync(&self) -> async_lock::RwLockWriteGuard<'_, CtxOptions> {
        self.0.write_blocking()
    }

    /// Downgrades this read/write handle to the read-only wrapper handed to
    /// lifecycle hooks. Only ever called by macro-generated code (to build
    /// the `IvoCtxOptions` passed into `on_success`/`on_failure`/`on_delete`
    /// and the returned `ctx_options` handle) -- not part of the public API,
    /// hence the leading `__` and `#[doc(hidden)]`, matching the crate's
    /// existing convention for internals that must stay technically
    /// reachable from macro-expanded code in the caller's own crate (Rust
    /// has no way to restrict visibility to "generated code only") but
    /// aren't meant to be called directly from user-written handlers.
    #[doc(hidden)]
    pub fn __read_only(&self) -> IvoCtxOptions<CtxOptions> {
        IvoCtxOptions(self.0.clone())
    }
}

impl<CtxOptions> Clone for IvoRwCtxOptions<CtxOptions> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<CtxOptions> fmt::Debug for IvoCtxOptions<CtxOptions> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IvoCtxOptions").finish()
    }
}

impl<CtxOptions> fmt::Debug for IvoRwCtxOptions<CtxOptions> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IvoRwCtxOptions").finish()
    }
}

/// A `Debug`-friendly wrapper around a trigger future so the returned
/// `(result, trigger, ctx_options)` tuple can be unwrapped in tests and user code.
pub struct IvoTriggerFuture<F>(F);

impl<F> IvoTriggerFuture<F> {
    pub fn new(future: F) -> Self {
        Self(future)
    }
}

impl<F> fmt::Debug for IvoTriggerFuture<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IvoTriggerFuture").finish()
    }
}

impl<F: Future> Future for IvoTriggerFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: projection to the inner field is structural for this wrapper.
        unsafe { Pin::map_unchecked_mut(self, |s| &mut s.0) }.poll(cx)
    }
}

/// Type-erased trigger future stored inside `IvoTriggerFn::Async`.
pub type IvoTrigger = IvoTriggerFuture<Pin<Box<dyn Future<Output = ()> + Send>>>;

/// A trigger that is either a synchronous closure or an asynchronous future.
pub enum IvoTriggerFn {
    Sync(Box<dyn FnOnce() + Send>),
    Async(IvoTrigger),
}

impl fmt::Debug for IvoTriggerFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IvoTriggerFn::Sync(_) => f.debug_struct("SyncTrigger").finish(),
            IvoTriggerFn::Async(_) => f.debug_struct("AsyncTrigger").finish(),
        }
    }
}

/// Wrap a synchronous trigger closure.
pub fn ivo_sync_trigger<F>(handler: F) -> IvoTriggerFn
where
    F: FnOnce() + Send + 'static,
{
    IvoTriggerFn::Sync(Box::new(handler))
}

/// Wrap an asynchronous trigger future.
pub fn ivo_trigger<F>(future: F) -> IvoTriggerFn
where
    F: Future<Output = ()> + Send + 'static,
{
    IvoTriggerFn::Async(IvoTriggerFuture::new(Box::pin(future)))
}

/// Handle returned on a successful `create`/`update`.
///
/// Call `handle_success` to run the `on_success` triggers that were captured for
/// this operation. If all captured handlers are synchronous (or there are none),
/// `handle_success` is synchronous. If any captured handler is asynchronous,
/// `handle_success` is asynchronous.
pub struct IvoSuccessHandle<O, CtxOptions, const ASYNC: bool, const HAS_SUCCESS: bool> {
    pub data: O,
    pub ctx_options: IvoCtxOptions<CtxOptions>,
    trigger: IvoTriggerFn,
}

impl<O, CtxOptions, const ASYNC: bool, const HAS_SUCCESS: bool>
    IvoSuccessHandle<O, CtxOptions, ASYNC, HAS_SUCCESS>
{
    pub fn new(data: O, ctx_options: IvoCtxOptions<CtxOptions>, trigger: IvoTriggerFn) -> Self {
        Self {
            data,
            ctx_options,
            trigger,
        }
    }
}

impl<O, CtxOptions> IvoSuccessHandle<O, CtxOptions, false, true> {
    pub fn handle_success(self) {
        match self.trigger {
            IvoTriggerFn::Sync(f) => f(),
            IvoTriggerFn::Async(_) => unreachable!(),
        }
    }
}

impl<O, CtxOptions> IvoSuccessHandle<O, CtxOptions, true, true> {
    pub async fn handle_success(self) {
        match self.trigger {
            IvoTriggerFn::Async(t) => t.await,
            IvoTriggerFn::Sync(_) => unreachable!(),
        }
    }
}

impl<O: fmt::Debug, CtxOptions, const ASYNC: bool, const HAS_SUCCESS: bool> fmt::Debug
    for IvoSuccessHandle<O, CtxOptions, ASYNC, HAS_SUCCESS>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IvoSuccessHandle")
            .field("data", &self.data)
            .finish()
    }
}

/// Handle returned on a failed `create`/`update`.
///
/// Call `handle_failure` to run the `on_failure` triggers that were captured for
/// this operation. If all captured handlers are synchronous (or there are none),
/// `handle_failure` is synchronous. If any captured handler is asynchronous,
/// `handle_failure` is asynchronous.
pub struct IvoFailureHandle<Payload, CtxOptions, const ASYNC: bool, const HAS_FAILURE: bool> {
    pub errors: Payload,
    pub ctx_options: IvoCtxOptions<CtxOptions>,
    trigger: IvoTriggerFn,
}

impl<Payload, CtxOptions, const ASYNC: bool, const HAS_FAILURE: bool>
    IvoFailureHandle<Payload, CtxOptions, ASYNC, HAS_FAILURE>
{
    pub fn new(
        errors: Payload,
        ctx_options: IvoCtxOptions<CtxOptions>,
        trigger: IvoTriggerFn,
    ) -> Self {
        Self {
            errors,
            ctx_options,
            trigger,
        }
    }
}

impl<Payload, CtxOptions> IvoFailureHandle<Payload, CtxOptions, false, true> {
    pub fn handle_failure(self) {
        match self.trigger {
            IvoTriggerFn::Sync(f) => f(),
            IvoTriggerFn::Async(_) => unreachable!(),
        }
    }
}

impl<Payload, CtxOptions> IvoFailureHandle<Payload, CtxOptions, true, true> {
    pub async fn handle_failure(self) {
        match self.trigger {
            IvoTriggerFn::Async(t) => t.await,
            IvoTriggerFn::Sync(_) => unreachable!(),
        }
    }
}

impl<Payload: fmt::Debug, CtxOptions, const ASYNC: bool, const HAS_FAILURE: bool> fmt::Debug
    for IvoFailureHandle<Payload, CtxOptions, ASYNC, HAS_FAILURE>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IvoFailureHandle")
            .field("errors", &self.errors)
            .finish()
    }
}
