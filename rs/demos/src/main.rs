use demos::users::run_users_demo;

// #[tokio::main]
#[async_std::main]
async fn main() {
    run_users_demo().await
}

// use smol::io;

// fn main() -> io::Result<()> {
//     smol::block_on(async {
//         run_users_demo().await;

//         Ok(())
//     })
// }
