use demos::places::run_places_demo;

#[tokio::main]
// #[async_std::main]
async fn main() {
    // run_users_demo().await;
    run_places_demo().await;
}

// use smol::io;

// fn main() -> io::Result<()> {
//     smol::block_on(async {
//         run_users_demo().await;

//         Ok(())
//     })
// }
