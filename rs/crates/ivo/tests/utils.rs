// use std::future::Future;

/// Generates runtime-specific test bindings for a single async block
#[macro_export]
macro_rules! test_matrix {
    ($name:ident, $async_body:expr) => {
        // The paste macro allows identifier concatenation
        paste::paste! {
            // 1. TOKIO RUNNER
            #[tokio::test]
            async fn [< $name _tokio >]() {
                let test_future = $async_body;
                test_future.await;
            }

            // 2. ASYNC-STD RUNNER
            #[async_std::test]
            async fn [< $name _async_std >]() {
                let test_future = $async_body;
                test_future.await;
            }

            // 3. SMOL RUNNER
            #[test]
            fn [< $name _smol >]() {
                let test_future = $async_body;
                smol::block_on(test_future);
            }
        }
    };
    ($name:ident, $panic_msg: literal, $async_body:expr) => {
        // The paste macro allows identifier concatenation
        paste::paste! {
            // 1. TOKIO RUNNER
            #[tokio::test]
            #[should_panic(expected = $panic_msg)]
            async fn [< $name _tokio >]() {
                let test_future = $async_body;
                test_future.await;
            }

            // 2. ASYNC-STD RUNNER
            #[async_std::test]
            #[should_panic(expected = $panic_msg)]
            async fn [< $name _async_std >]() {
                let test_future = $async_body;
                test_future.await;
            }

            // 3. SMOL RUNNER
            #[test]
            #[should_panic(expected = $panic_msg)]
            fn [< $name _smol >]() {
                let test_future = $async_body;
                smol::block_on(test_future);
            }
        }
    };
}
