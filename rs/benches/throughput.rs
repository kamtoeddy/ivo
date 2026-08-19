use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::LazyLock;

mod schemas;

static TOKIO_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

fn bench_throughput(c: &mut Criterion) {
    let rt: &tokio::runtime::Runtime = &TOKIO_RT;

    c.bench_function("minimal create", |b| {
        b.to_async(rt).iter(|| async {
            let input = schemas::PartialMinimalInput { value: Some(42) };
            let _ = schemas::MINIMAL_MODEL
                .create(&input, None)
                .await
                .ok()
                .unwrap();
        });
    });

    c.bench_function("user create", |b| {
        b.to_async(rt).iter(|| async {
            let input = schemas::PartialUserInput {
                id: Some(String::from("1")),
                name: Some(String::from("Alice")),
                email: Some(String::from("alice@example.com")),
                age: Some(30),
            };
            let (_, _, _) = schemas::USER_MODEL.create(&input, None).await.ok().unwrap();
        });
    });

    c.bench_function("create 20 required fields (sync validators)", |b| {
        b.to_async(rt).iter(|| async {
            let input = schemas::PartialManyFieldInput20 {
                field_0: Some(0),
                field_1: Some(1),
                field_2: Some(2),
                field_3: Some(3),
                field_4: Some(4),
                field_5: Some(5),
                field_6: Some(6),
                field_7: Some(7),
                field_8: Some(8),
                field_9: Some(9),
                field_10: Some(10),
                field_11: Some(11),
                field_12: Some(12),
                field_13: Some(13),
                field_14: Some(14),
                field_15: Some(15),
                field_16: Some(16),
                field_17: Some(17),
                field_18: Some(18),
                field_19: Some(19),
            };
            let (_, _, _) = schemas::MANY_FIELD_MODEL_20
                .create(&input, None)
                .await
                .ok()
                .unwrap();
        });
    });

    c.bench_function("dependent chain length 10", |b| {
        b.to_async(rt).iter(|| async {
            let input = schemas::PartialChainInput { field_0: Some(1) };
            let _ = schemas::CHAIN_MODEL
                .create(&input, None)
                .await
                .ok()
                .unwrap();
        });
    });

    c.bench_function("create 10 readonly lax fields", |b| {
        b.to_async(rt).iter(|| async {
            let input = schemas::PartialReadonlyInput {
                readonly_0: Some(String::from("a")),
                readonly_1: Some(String::from("b")),
                readonly_2: Some(String::from("c")),
                readonly_3: Some(String::from("d")),
                readonly_4: Some(String::from("e")),
                readonly_5: Some(String::from("f")),
                readonly_6: Some(String::from("g")),
                readonly_7: Some(String::from("h")),
                readonly_8: Some(String::from("i")),
                readonly_9: Some(String::from("j")),
            };
            let _ = schemas::READONLY_MODEL
                .create(&input, None)
                .await
                .ok()
                .unwrap();
        });
    });

    c.bench_function("no-op update", |b| {
        let data = rt.block_on(async {
            let input = schemas::PartialUserInput {
                id: Some(String::from("1")),
                name: Some(String::from("Alice")),
                email: Some(String::from("alice@example.com")),
                age: Some(30),
            };
            let (data, _, _) = schemas::USER_MODEL.create(&input, None).await.ok().unwrap();
            data
        });

        b.to_async(rt).iter(|| async {
            let updates = schemas::PartialUserInput::default();
            let _ = schemas::USER_MODEL.update(&data, &updates, None).await;
        });
    });

    c.bench_function("single field update", |b| {
        let data = rt.block_on(async {
            let input = schemas::PartialUserInput {
                id: Some(String::from("1")),
                name: Some(String::from("Alice")),
                email: Some(String::from("alice@example.com")),
                age: Some(30),
            };
            schemas::USER_MODEL
                .create(&input, None)
                .await
                .ok()
                .unwrap()
                .0
        });

        b.to_async(rt).iter(|| async {
            let mut updates = schemas::PartialUserInput::default();
            updates.age = Some(31);
            let (_, _, _) = schemas::USER_MODEL
                .update(&data, &updates, None)
                .await
                .ok()
                .unwrap();
        });
    });
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
