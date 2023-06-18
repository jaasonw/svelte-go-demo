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
    let time = format!("{:?} ms", duration);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(time.into())?)
}
