mod lib;
use std::path::Path;
use soundcloud::Client;
use tokio::{fs::File, runtime::Runtime};
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() {
    lib::run().await;
}
