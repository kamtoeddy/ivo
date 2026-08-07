use std::{future::ready, sync::LazyLock};

use ivo::{required_field, IvoContext, IvoInputStruct, IvoModel, IvoShared, IvoStruct};

const MIN_USERNAME_LEN: usize = 4;

#[async_std::main]
async fn main() {
    let (payload, handle_failure, _) = DATA_MODEL
        .create(&PartialDataInput { username: None }, None)
        .await
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", payload);

    assert_eq!(
        payload.get("username").unwrap().reason,
        "\"username\" is required!"
    );

    handle_failure().await;

    let data = Data {
        username: "john-doe".to_string(),
    };

    let updated_username = Some("jane-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                username: updated_username.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            username: updated_username.as_deref().map(sanitize_username)
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub username: String,
}

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(

                required_field("username")
                    .validate(|_, _, _| ready(Ok(None::<String>)))
                    .re_validate(|v: String, _, _| {
                        if v.len() < MIN_USERNAME_LEN {
                            return ready(Err((
                                format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long"),
                                None,
                            )));
                        }

                        ready(Ok(Some(sanitize_username(&v))))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_success]: username = {}",
                            ctx.values().username.unwrap()
                        );

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: username = {}", data.username);

                        ready(())
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_failure]: raw username = {}",
                            ctx.raw_input().username.unwrap()
                        );
                        println!(
                            "\n[on_failure]: validated username = {}",
                            ctx.input().username.unwrap()
                        );

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});

fn sanitize_username(name: &str) -> String {
    format!("re-validated-{name}")
}
