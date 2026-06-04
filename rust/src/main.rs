use ivo::demo::run::run_example;

#[tokio::main]
async fn main() {
    println!("Program started\n",);
    run_example().await
}
