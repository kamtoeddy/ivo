use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::LazyLock;

mod domain;
mod slugify;

use domain::{PartialUserInput, UserCtxOptions, USER_MODEL};

static TOKIO_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

fn bench_main_demo(c: &mut Criterion) {
    let rt: &tokio::runtime::Runtime = &TOKIO_RT;

    c.bench_function("main_demo create", |b| {
        b.to_async(rt).iter(|| async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_username("user-10".into());
            let _ = USER_MODEL.create(&input, UserCtxOptions::new()).await;
        });
    });

    c.bench_function("main_demo update", |b| {
        let user = rt.block_on(async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_username("user-10".into());
            let (data, _, _) = match USER_MODEL.create(&input, UserCtxOptions::new()).await {
                Ok(result) => result,
                Err(_) => panic!("main_demo create failed"),
            };
            data
        });

        b.to_async(rt).iter(|| async {
            let updates = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_phone_number(Some("123 4567 8910".into()))
                .with_slug_id("updated-slug-id: Lol".into())
                .with_username("new_username".into());

            let _ = USER_MODEL
                .update(&user, &updates, UserCtxOptions::new())
                .await;
        });
    });

    c.bench_function("main_demo delete", |b| {
        let user = rt.block_on(async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_username("user-10".into());
            let (data, _, _) = match USER_MODEL.create(&input, UserCtxOptions::new()).await {
                Ok(result) => result,
                Err(_) => panic!("main_demo create failed"),
            };
            data
        });

        b.to_async(rt).iter(|| async {
            USER_MODEL.delete(&user, UserCtxOptions::new()).await;
        });
    });
}

criterion_group!(benches, bench_main_demo);
criterion_main!(benches);
