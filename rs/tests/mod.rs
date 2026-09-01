#[macro_export]
macro_rules! async_test_matrix {
    ($name:ident) => {
        paste::paste! {
            #[tokio::test]
            async fn [<$name _tokio>]() {
                $name().await;
            }

            #[async_std::test]
            async fn [<$name _async_std>]() {
                $name().await;
            }

            #[test]
            fn [<$name _smol>]() {
                smol::block_on(async { $name().await; });
            }
        }
    };

    ($expected:literal, $name:ident) => {
        paste::paste! {
            #[should_panic(expected = $expected)]
            #[tokio::test]
            async fn [<$name _tokio>]() {
                $name().await;
            }

            #[should_panic(expected = $expected)]
            #[async_std::test]
            async fn [<$name _async_std>]() {
                $name().await;
            }

            #[should_panic(expected = $expected)]
            #[test]
            fn [<$name _smol>]() {
                smol::block_on(async { $name().await; });
            }
        }
    };
}

mod extras;
mod field_configs;
mod fields;
mod ivo_struct;
mod options;
mod smoke;

use std::sync::atomic::AtomicUsize;

pub static ON_SUCCESS_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub static ON_FAILURE_COUNTER: AtomicUsize = AtomicUsize::new(0);
