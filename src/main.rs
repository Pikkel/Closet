#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    warp::serve(warp::fs::dir("Projects"))
        .run(([0, 0, 0, 0], 1337))
        .await;
}
