use std::any::Any;
use std::fmt::Debug;

pub trait CloneableAny: Any + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableAny>;
    fn as_any(&self) -> &dyn Any;
}

impl<T> CloneableAny for T
where
    T: Clone + Debug + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(T::clone(self)) // This triggers the concrete type's clone method!
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for Box<dyn CloneableAny> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type ErasedValue = Box<dyn CloneableAny>;

pub fn erase_value<T: Clone + Debug + Send + Sync + 'static>(value: T) -> Box<dyn CloneableAny> {
    Box::new(value)
}

pub fn parse_value<T: Clone + Debug + Send + Sync + 'static>(
    e: &Box<dyn CloneableAny>,
) -> Option<T> {
    e.as_any().downcast_ref::<T>().cloned()
}

pub fn parse_or_panic<T: Clone + Debug + Send + Sync + 'static>(e: &Box<dyn CloneableAny>) -> T {
    parse_value::<T>(e).expect("Failed to parse value").clone()
}
