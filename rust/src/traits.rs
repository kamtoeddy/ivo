use serde::{de::DeserializeOwned, Serialize};

pub trait HasPartial {
    type Partial: Serialize + DeserializeOwned;
}

pub type Partial<T> = <T as HasPartial>::Partial;
