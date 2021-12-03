#[tokio::main]
async fn main() {
    warp::serve(warp::fs::dir("Projects"))
        .run(([0, 0, 0, 0], 1337))
        .await;
}
