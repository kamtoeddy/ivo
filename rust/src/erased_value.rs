use std::any::Any;
use std::fmt::Debug;

// use ivo::types::{erase_value, parse_or_panic};

// 1. Define a trait that requires Any + Send + Sync, and defines an explicit cloning helper
pub trait CloneableAny: Any + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableAny>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// 2. Implement this trait for ANY type that already implements Clone + Any + Send + Sync
impl<T> CloneableAny for T
where
    T: Clone + Debug + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(T::clone(&self)) // This triggers the concrete type's clone method!
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// 3. Implement standard Clone for our uniform Box type
impl Clone for Box<dyn CloneableAny> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type ErasedValue = Box<dyn CloneableAny>;

pub fn erase_value<T: Clone + Debug + Send + Sync + 'static>(value: T) -> Box<dyn CloneableAny> {
    Box::new(value)
}

pub fn parse_or_panic<T: Clone + Debug + Send + Sync + 'static>(e: &Box<dyn CloneableAny>) -> T {
    e.as_any()
        .downcast_ref::<T>()
        .cloned()
        .expect("Failed to parse value")
        .clone()
}
