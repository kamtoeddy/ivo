use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::LazyLock;

mod schemas;

static TOKIO_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

fn bench_memory(c: &mut Criterion) {
    let rt: &tokio::runtime::Runtime = &TOKIO_RT;

    c.bench_function("memory minimal create x1000", |b| {
        b.to_async(rt).iter(|| async {
            for _ in 0..1000 {
                let input = schemas::PartialMinimal { value: Some(42) };
                let _ = schemas::MinimalModel.create(input, ()).await.unwrap();
            }
        });
    });

    c.bench_function("memory user create x1000", |b| {
        b.to_async(rt).iter(|| async {
            for _ in 0..1000 {
                let input = schemas::PartialUser {
                    id: Some(String::from("1")),
                    name: Some(String::from("Alice")),
                    email: Some(String::from("alice@example.com")),
                    age: Some(30),
                };
                let _ = schemas::UserModel.create(input, ()).await.unwrap();
            }
        });
    });

    c.bench_function("memory 20 fields create x1000", |b| {
        b.to_async(rt).iter(|| async {
            for _ in 0..1000 {
                let input = schemas::PartialManyField20 {
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
                let _ = schemas::ManyField20Model.create(input, ()).await.unwrap();
            }
        });
    });

    c.bench_function("memory no-op update x1000", |b| {
        let data = rt.block_on(async {
            let input = schemas::PartialUser {
                id: Some(String::from("1")),
                name: Some(String::from("Alice")),
                email: Some(String::from("alice@example.com")),
                age: Some(30),
            };
            schemas::UserModel.create(input, ()).await.unwrap().0
        });

        b.to_async(rt).iter(|| async {
            for _ in 0..1000 {
                let updates = schemas::PartialUser::default();
                let _ = schemas::UserModel.update(data.clone(), updates, ()).await;
            }
        });
    });
}

criterion_group!(benches, bench_memory);
criterion_main!(benches);
