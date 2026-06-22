#![allow(type_alias_bounds)]

use futures::future::BoxFuture;
use std::{fmt::Debug, sync::Arc};

use crate::{IvoContext, IvoSchemaStruct};

pub trait IvoFieldValue: Clone + Debug + Send + Sync + 'static {}

impl<T> IvoFieldValue for T where T: Clone + Debug + Send + Sync + 'static {}

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;
pub trait IsProvided {}
pub trait IsProvidedButNotComputed: IsProvided {}

impl IsProvided for Yes {}
impl IsProvided for YesComputed {}
impl IsProvidedButNotComputed for Yes {}

pub type DeleteHandler<O: IvoSchemaStruct, CtxOptions> =
    Box<dyn Fn(Arc<O>, Arc<CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> = Box<
    dyn Fn(Arc<IvoContext<I, O>>, Arc<CtxOptions>) -> BoxFuture<'static, ()>
        + Send
        + Sync
        + 'static,
>;

pub type SuccessHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> = Box<
    dyn Fn(Arc<IvoContext<I, O>>, Arc<CtxOptions>) -> BoxFuture<'static, ()>
        + Send
        + Sync
        + 'static,
>;
