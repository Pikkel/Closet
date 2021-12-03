#[tokio::main]
async fn main() {
    warp::serve(warp::fs::dir("Projects/Private"));
    warp::serve(warp::fs::dir("Projects/Public"))
        .run(([0, 0, 0, 0], 1337))
        .await;
}
