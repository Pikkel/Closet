#[tokio::main]
async fn main() {
    warp::serve(warp::fs::dir("Private"))
    warp::serve(warp::fs::dir("Public"))
        .run(([0, 0, 0, 0], 1337))
        .await;
}
