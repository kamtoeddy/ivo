#![expect(type_alias_bounds)]
#![expect(clippy::borrowed_box)]

use futures::future::BoxFuture;
pub use futures_locks::RwLock as IvoRwLock;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt::Debug,
    future::Future,
};

use crate::{
    __private_types::DefaultFieldErrorMetadata, IvoContext, IvoErrorSanitizer, IvoRwCtxOptions,
};

pub trait IvoStruct:
    Send + Sync + Sized + 'static + WithPartialStruct + IvoStructMethods + Into<Self::Partial>
{
    #[inline(always)]
    fn append_updates(&mut self, updates: &Self::Partial) {
        self.ivo_internal_update_with(updates)
    }

    #[inline(always)]
    fn clone_with_updates(&self, updates: &Self::Partial) -> Self {
        self.ivo_internal_clone_with_ref(updates)
    }
}

pub trait IvoInputStruct<CtxOptions, ErrorSanitizer: IvoErrorSanitizer<CtxOptions>>:
    IvoStruct + WithPartialErrors<ErrorSanitizer::Metadata>
{
}

pub trait WithPartialStruct {
    type Partial: PartialEq + Debug + Default + Send + Sync + Clone + PartialStructMethods;
}

pub trait WithPartialErrors<FieldErrorMetadata: Send + Sync> {
    type PartialErrors: Send + Sync + PartialErrorsMethods<FieldErrorMetadata>;
}

pub trait IvoStructMethods: WithPartialStruct + Clone {
    fn ivo_internal_dangerously_get_values_from_partial(partial_values: Self::Partial) -> Self;

    fn ivo_internal_get_updates_from_partial(
        &self,
        updates: &Self::Partial,
    ) -> Option<Self::Partial>;

    #[inline(always)]
    fn ivo_internal_clone_with(&self, updates: Self::Partial) -> Self {
        self.ivo_internal_clone_with_ref(&updates)
    }

    fn ivo_internal_clone_with_ref(&self, updates: &Self::Partial) -> Self {
        let mut cloned = self.clone();

        cloned.ivo_internal_update_with(updates);

        cloned
    }

    fn ivo_internal_update_with(&mut self, updates: &Self::Partial);

    fn ivo_internal_field_names() -> HashSet<String>;

    fn ivo_internal_name() -> String;
}

pub trait PartialStructMethods: Clone {
    fn ivo_internal_enumerate_fields_available(&self) -> Vec<(String, ErasedValue)>;

    fn ivo_internal_fields_available(&self) -> Vec<String>;

    fn ivo_internal_get_erased_value(&self, field_name: &str) -> ErasedValue;

    fn ivo_internal_is_value_equal(&self, field_name: &str, value: &ErasedValue) -> bool;

    fn ivo_internal_set(&mut self, field_name: &str, value: &ErasedValue);

    fn ivo_internal_unset(&mut self, field_name: &str);
}

pub trait PartialErrorsMethods<FieldErrorMetadata: Send + Sync> {
    fn entries(self) -> Vec<(String, (String, Option<FieldErrorMetadata>))>;
}

pub type Partial<T> = <T as WithPartialStruct>::Partial;

pub trait FieldValue: Clone + Debug + Send + Sync + 'static {}

impl<T> FieldValue for T where T: Clone + Debug + Send + Sync + 'static {}

pub trait CloneableAny: Any + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableAny>;
    fn as_any(&self) -> &dyn Any;
    fn runtime_type_name(&self) -> &'static str;
}

impl<T> CloneableAny for T
where
    T: FieldValue,
{
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(T::clone(self)) // This triggers the concrete type's clone method!
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn runtime_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

impl Clone for Box<dyn CloneableAny> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type ErasedValue = Box<dyn CloneableAny>;

#[inline(always)]
pub fn erase_value<T: FieldValue>(value: T) -> Box<dyn CloneableAny> {
    Box::new(value)
}

#[inline(always)]
pub fn parse_value<T: FieldValue>(e: &Box<dyn CloneableAny>) -> Option<T> {
    e.as_any().downcast_ref::<T>().cloned()
}

pub fn parse_or_panic<T: FieldValue>(
    erased_value: &Box<dyn CloneableAny>,
    field_name: Option<&str>,
) -> T {
    let value = parse_value::<T>(erased_value);

    if let Some(actual_value) = value {
        return actual_value;
    }

    let expected_type_path = std::any::type_name::<T>();
    let actual_type_path = erased_value.runtime_type_name();
    let field_name = field_name
        .map(|n| format!("\"{n}\""))
        .unwrap_or_else(|| "value".into());

    panic!(
        "\nFailed to parse {field_name}. Expected: \"{expected_type_path}\", but got \"{actual_type_path}\"\n"
    )
}

pub type ValidatorResponse<T: FieldValue, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<Option<T>, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);

pub type PostValidatorError<FieldErrorMetadata = DefaultFieldErrorMetadata> =
    HashMap<String, ValidatorError<FieldErrorMetadata>>;

pub type PostValidatorResponse<
    I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
> = Result<Option<I::Partial>, I::PartialErrors>;

pub type Resolver<T, I: IvoStruct, O: IvoStruct, CtxOptions> = Box<
    dyn Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, T>
        + Send
        + Sync
        + 'static,
>;

pub type InitResolver<T, I: IvoStruct, CtxOptions> = Box<
    dyn Fn(I::Partial, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, T>
        + Send
        + Sync
        + 'static,
>;

pub type BooleanResolver<I, O, CtxOptions> = Resolver<bool, I, O, CtxOptions>;

pub type InitBooleanResolver<I, CtxOptions> = InitResolver<bool, I, CtxOptions>;

pub trait IntoIgnoreUpdateResolver<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_resolver(self) -> BooleanResolver<I, O, CtxOptions>;
}

pub type IgnoreUpdateOptionResolver<I: IvoStruct, O: IvoStruct, CtxOptions> = Box<
    dyn Fn(I::Partial, O, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, bool>
        + Send
        + Sync
        + 'static,
>;

impl<F, Fut, I: IvoStruct, O: IvoStruct, CtxOptions> IntoIgnoreUpdateResolver<I, O, CtxOptions>
    for F
where
    F: Fn(I::Partial, O, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + 'static,
{
    fn into_resolver(self) -> BooleanResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx.input(), ctx.full_values().unwrap(), o)))
    }
}
