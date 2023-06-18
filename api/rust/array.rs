use std::time::Instant;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut v = Vec::new();
    let start = Instant::now();
    for _ in 0..1000000 {
        v.push("test");
    }
    let duration = start.elapsed();
    let time = format!("{}.{:03} ms", duration.as_secs(), duration.subsec_millis());
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/text")
        .body(time.into())?)
}