use serde::{de::DeserializeOwned, Serialize};

pub trait HasPartial {
    type Partial: Serialize + DeserializeOwned;
}

// 2. The TypeScript-style Utility Alias
pub type Partial<T> = <T as HasPartial>::Partial;
