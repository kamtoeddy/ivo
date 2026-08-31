use chrono::{Days, Utc};
use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::LazyLock;

mod domain;
mod slugify;

use domain::{PartialUserInput, User, UserCtxOptions, UserModel};

static TOKIO_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

fn bench_main_demo(c: &mut Criterion) {
    let rt: &tokio::runtime::Runtime = &TOKIO_RT;

    c.bench_function(
        "main_demo create [fail: required errors (email or phone_number)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let input = PartialUserInput::new().with_username("user-10".into());

                let _ = UserModel.create(input, UserCtxOptions::new()).await;
            });
        },
    );

    c.bench_function(
        "main_demo create [fail: required errors (email or phone_number, username)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let _ = UserModel
                    .create(PartialUserInput::new(), UserCtxOptions::new())
                    .await;
            });
        },
    );

    c.bench_function(
        "main_demo create [fail: validation error (email, slug_id, username)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let input = PartialUserInput::new()
                    .with_email(Some("1.com".into()))
                    .with_phone_number(Some("123 4567 8910".into()))
                    .with_slug_id("s".into())
                    .with_username("u".into());

                let _ = UserModel.create(input, UserCtxOptions::new()).await;
            });
        },
    );

    c.bench_function(
        "main_demo create [fail: re_validation error (username taken)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let input = PartialUserInput::new()
                    .with_email(Some("1@1.com".into()))
                    .with_username("user-1".into());

                let _ = UserModel.create(input, UserCtxOptions::new()).await;
            });
        },
    );

    c.bench_function(
        "main_demo create [fail: post-validation error (slug taken)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let input = PartialUserInput::new()
                    .with_email(Some("1@1.com".into()))
                    .with_username("user-10".into())
                    .with_slug_id("user-1".into());

                let _ = UserModel.create(input, UserCtxOptions::new()).await;
            });
        },
    );

    c.bench_function("main_demo create [success: 2/4 inputs (a)]", |b| {
        b.to_async(rt).iter(|| async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_username("user-10".into());

            let _ = UserModel.create(input, UserCtxOptions::new()).await;
        });
    });

    c.bench_function("main_demo create [success: 2/4 inputs (b)]", |b| {
        b.to_async(rt).iter(|| async {
            let input = PartialUserInput::new()
                .with_phone_number(Some("123 4567 8910".into()))
                .with_username("user-10".into());

            let _ = UserModel.create(input, UserCtxOptions::new()).await;
        });
    });

    c.bench_function("main_demo create [success: 3/4 inputs]", |b| {
        b.to_async(rt).iter(|| async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_phone_number(Some("123 4567 8910".into()))
                .with_username("user-10".into());

            let _ = UserModel.create(input, UserCtxOptions::new()).await;
        });
    });

    c.bench_function("main_demo create [success: 4/4 inputs]", |b| {
        b.to_async(rt).iter(|| async {
            let input = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_phone_number(Some("123 4567 8910".into()))
                .with_username("user-10".into())
                .with_slug_id("sloppy-slug-id".into());

            let _ = UserModel.create(input, UserCtxOptions::new()).await;
        });
    });

    let (username, slug_id) = {
        let username = "John Doe";

        (username.into(), username.into())
    };

    let two_days_ago = Utc::now().checked_sub_days(Days::new(2)).unwrap();

    let user = User {
        id: 1,
        created_at: two_days_ago,
        updated_at: two_days_ago,
        email: Some("1@1.com".into()),
        phone_number: Some("123 4567 8910".into()),
        username,
        username_last_updated_at: None,
        slug_id,
    };

    c.bench_function(
        "main_demo update [fail: required error (email or phone_number)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let updates = PartialUserInput::new()
                    .with_email(None)
                    .with_phone_number(None);

                let _ = UserModel
                    .update(user.clone(), updates, UserCtxOptions::new())
                    .await;
            });
        },
    );

    c.bench_function(
        "main_demo update [fail: validation error (email, slug_id, username)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let updates = PartialUserInput::new()
                    .with_email(Some("1.com".into()))
                    .with_phone_number(Some("123 4567 8910".into()))
                    .with_slug_id("s".into())
                    .with_username("u".into());

                let _ = UserModel
                    .update(user.clone(), updates, UserCtxOptions::new())
                    .await;
            });
        },
    );

    c.bench_function(
        "main_demo update [fail: re_validation error (username taken)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let updates = PartialUserInput::new().with_username("user-1".into());

                let _ = UserModel
                    .update(user.clone(), updates, UserCtxOptions::new())
                    .await;
            });
        },
    );

    c.bench_function(
        "main_demo update [fail: post-validation error (slug taken)]",
        |b| {
            b.to_async(rt).iter(|| async {
                let updates = PartialUserInput::new().with_slug_id("user-1".into());

                let _ = UserModel
                    .update(user.clone(), updates, UserCtxOptions::new())
                    .await;
            });
        },
    );

    c.bench_function("main_demo update [success: 1/4 inputs (d)]", |b| {
        b.to_async(rt).iter(|| async {
            let updates = PartialUserInput::new().with_email(Some("1@2.com".into()));

            let _ = UserModel
                .update(user.clone(), updates, UserCtxOptions::new())
                .await;
        });
    });

    c.bench_function("main_demo update [success: 1/4 inputs (b)]", |b| {
        b.to_async(rt).iter(|| async {
            let updates = PartialUserInput::new().with_phone_number(Some("123 4567 8911".into()));

            let _ = UserModel
                .update(user.clone(), updates, UserCtxOptions::new())
                .await;
        });
    });

    c.bench_function("main_demo update [success: 1/4 inputs (c)]", |b| {
        b.to_async(rt).iter(|| async {
            let updates = PartialUserInput::new().with_slug_id("newly-updated-slug-id: Lol".into());

            let _ = UserModel
                .update(user.clone(), updates, UserCtxOptions::new())
                .await;
        });
    });

    c.bench_function("main_demo update [success: 1/4 inputs (d)]", |b| {
        b.to_async(rt).iter(|| async {
            let updates = PartialUserInput::new().with_username("new-username".into());

            let _ = UserModel
                .update(user.clone(), updates, UserCtxOptions::new())
                .await;
        });
    });

    c.bench_function("main_demo update [success: 3/4 inputs]", |b| {
        b.to_async(rt).iter(|| async {
            let updates = PartialUserInput::new()
                .with_email(Some("1@1.com".into()))
                .with_phone_number(Some("123 4567 8910".into()))
                .with_username("new_username".into());

            let _ = UserModel
                .update(user.clone(), updates, UserCtxOptions::new())
                .await;
        });
    });

    c.bench_function("main_demo update [success: 4/4 inputs]", |b| {
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
