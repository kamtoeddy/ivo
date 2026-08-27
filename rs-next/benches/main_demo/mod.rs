use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::LazyLock;

mod domain;
mod slugify;

use domain::{PartialUserInput, UserCtxOptions, UserModel};

static TOKIO_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

fn bench_main_demo(c: &mut Criterion) {
    let rt: &tokio::runtime::Runtime = &TOKIO_RT;

    c.bench_function("main_demo create", |b| {
        b.to_async(rt).iter(|| async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_username("user-10".into());
            let _ = UserModel.create(input, UserCtxOptions::new()).await;
        });
    });

    c.bench_function("main_demo update", |b| {
        let user = rt.block_on(async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_username("user-10".into());
            match UserModel.create(input, UserCtxOptions::new()).await {
                Ok(handle) => handle.data,
                Err(_) => panic!("main_demo create failed"),
            }
        });

        b.to_async(rt).iter(|| async {
            let updates = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_phone_number(Some("123 4567 8910".into()))
                .with_slug_id("updated-slug-id: Lol".into())
                .with_username("new_username".into());

            let _ = UserModel
                .update(user.clone(), updates, UserCtxOptions::new())
                .await;
        });
    });

    c.bench_function("main_demo delete", |b| {
        let user = rt.block_on(async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_username("user-10".into());
            match UserModel.create(input, UserCtxOptions::new()).await {
                Ok(handle) => handle.data,
                Err(_) => panic!("main_demo create failed"),
            }
        });

        b.to_async(rt).iter(|| async {
            UserModel.delete(&user, UserCtxOptions::new());
        });
    });
}

criterion_group!(benches, bench_main_demo);
criterion_main!(benches);
