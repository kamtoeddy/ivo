#![allow(dead_code)]

pub use ivo_derive::ivo_schema;

use std::collections::HashMap;

// Error handling

pub type DefaultFieldErrorMetadata = ();
pub type DefaultErrorPayload = IvoErrorPayload<DefaultFieldErrorMetadata>;

#[derive(Debug, Clone)]
pub struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<Metadata>,
}

pub type IvoErrorPayload<Metadata> = HashMap<String, FieldError<Metadata>>;

pub async fn run_resolver<T, Ctx, Opts, F, Fut>(ctx: Ctx, opts: &Opts, resolver: F) -> T
where
    F: FnOnce(Ctx, &Opts) -> Fut,
    Fut: std::future::Future<Output = T>,
{
    resolver(ctx, opts).await
}

pub async fn run_sanitizer<T, Ctx, Opts, F>(value: T, ctx: &Ctx, opts: &Opts, sanitizer: F) -> T
where
    F: for<'a> FnOnce(
        T,
        &'a Ctx,
        &'a Opts,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = T> + 'a>>,
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
    F: for<'a> FnOnce(
        T,
        &'a Ctx,
        &'a Opts,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<T>, FieldError<Metadata>>> + 'a>,
    >,
{
    validator(value, ctx, opts).await
}

pub async fn run_boolean_resolver<Ctx, Opts, F>(ctx: &Ctx, opts: &Opts, resolver: F) -> bool
where
    F: for<'a> FnOnce(
        &'a Ctx,
        &'a Opts,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + 'a>>,
{
    resolver(ctx, opts).await
}

pub async fn run_required_resolver<Ctx, Opts, F>(
    ctx: &Ctx,
    opts: &Opts,
    resolver: F,
) -> Option<String>
where
    F: for<'a> FnOnce(
        &'a Ctx,
        &'a Opts,
    )
        -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + 'a>>,
{
    resolver(ctx, opts).await
}

pub async fn run_hook<Ctx, Opts, F, Fut>(ctx: Ctx, opts: &Opts, handler: F)
where
    F: FnOnce(Ctx, &Opts) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    handler(ctx, opts).await
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
    type Partial: PartialStructMethods + Default + Clone + Send + Sync + 'static;
}

pub trait WithPartialErrors<Metadata: Send + Sync> {
    type PartialErrors: PartialErrorsMethods<Metadata> + Send + Sync;
}

pub trait IvoStructMethods: WithPartialStruct + Clone + Send + Sync + 'static {
    fn ivo_internal_update_with(&mut self, updates: &Self::Partial);
}

pub trait IvoStruct:
    IvoStructMethods + WithPartialStruct + Into<Self::Partial> + Send + Sync + 'static
{
    fn append_updates(&mut self, updates: &Self::Partial) {
        self.ivo_internal_update_with(updates);
    }
}

pub trait IvoInputStruct<CtxOptions, ErrorSanitizer: IvoErrorSanitizer<CtxOptions>>:
    IvoStruct + WithPartialErrors<ErrorSanitizer::Metadata>
{
}

pub trait PartialStructMethods: Clone + Default + Send + Sync + 'static {
    fn ivo_internal_fields_available(&self) -> Vec<String>;
}

pub trait PartialErrorsMethods<Metadata: Send + Sync> {
    fn entries(self) -> Vec<(String, (String, Option<Metadata>))>;
}

impl<Metadata: Send + Sync + Clone> PartialErrorsMethods<Metadata> for IvoErrorPayload<Metadata> {
    fn entries(self) -> Vec<(String, (String, Option<Metadata>))> {
        self.into_iter()
            .map(|(k, v)| (k, (v.reason, v.metadata)))
            .collect()
    }
}

// Context types

#[derive(Clone)]
pub struct IvoContext<I, O: WithPartialStruct> {
    input: I,
    values: O,
    changes: O::Partial,
    is_update: bool,
}

impl<I, O: WithPartialStruct> IvoContext<I, O> {
    pub fn new(input: I, values: O, changes: O::Partial, is_update: bool) -> Self {
        Self {
            input,
            values,
            changes,
            is_update,
        }
    }

    pub fn input(&self) -> &I {
        &self.input
    }

    pub fn raw_input(&self) -> &I {
        &self.input
    }

    pub fn values(&self) -> &O {
        &self.values
    }

    pub fn changes(&self) -> &O::Partial {
        &self.changes
    }

    pub fn full_values(&self) -> &O {
        &self.values
    }

    pub fn previous_values(&self) -> &O {
        &self.values
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }
}

pub struct IvoDefaultCtx<I> {
    _input: std::marker::PhantomData<I>,
}

impl<I> IvoDefaultCtx<I> {
    pub fn input(&self) -> &I {
        unimplemented!()
    }

    pub fn raw_input(&self) -> &I {
        unimplemented!()
    }
}

pub struct IvoConstantCtx<I, O> {
    _input: std::marker::PhantomData<I>,
    _output: std::marker::PhantomData<O>,
}

impl<I, O> IvoConstantCtx<I, O> {
    pub fn input(&self) -> &I {
        unimplemented!()
    }

    pub fn raw_input(&self) -> &I {
        unimplemented!()
    }

    pub fn values(&self) -> &O {
        unimplemented!()
    }
}

// Options wrappers

pub struct IvoCtxOptions<CtxOptions>(pub std::sync::Arc<CtxOptions>);

impl<CtxOptions> Clone for IvoCtxOptions<CtxOptions> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct IvoRwCtxOptions<CtxOptions>(pub std::sync::Arc<std::sync::RwLock<CtxOptions>>);

impl<CtxOptions> Clone for IvoRwCtxOptions<CtxOptions> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
