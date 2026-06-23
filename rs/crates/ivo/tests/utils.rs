/// Generates runtime-specific test bindings for a single async block
#[macro_export]
macro_rules! async_test_matrix {
    ($async_test_fn: ident) => {
        // The paste macro allows identifier concatenation
        paste::paste! {
            // 1. TOKIO RUNNER
            #[tokio::test]
            async fn [< $async_test_fn _tokio >]() {
                $async_test_fn().await;
            }

            // 2. ASYNC-STD RUNNER
            #[async_std::test]
            async fn [< $async_test_fn _async_std >]() {
                $async_test_fn().await;
            }

            // 3. SMOL RUNNER
            #[test]
            fn [< $async_test_fn _smol >]() {
                smol::block_on($async_test_fn());
            }
        }
    };
    ($panic_msg: literal, $async_test_fn: ident) => {
        // The paste macro allows identifier concatenation
        paste::paste! {
            // 1. TOKIO RUNNER
            #[tokio::test]
            #[should_panic(expected = $panic_msg)]
            async fn [< $async_test_fn _tokio >]() {
                $async_test_fn().await;
            }

            // 2. ASYNC-STD RUNNER
            #[async_std::test]
            #[should_panic(expected = $panic_msg)]
            async fn [< $async_test_fn _async_std >]() {
                $async_test_fn().await;
            }

            // 3. SMOL RUNNER
            #[test]
            #[should_panic(expected = $panic_msg)]
            fn [< $async_test_fn _smol >]() {
                smol::block_on($async_test_fn());
            }
        }
    };
}
