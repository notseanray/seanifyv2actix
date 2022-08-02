pub(crate) async fn run() {
    warp::serve(warp::fs::dir("./music"))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
