use rand::{thread_rng, Rng};
use std::time::Instant;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut a: Vec<u64> = Vec::new();
    for _ in 0..5000000 {
        a.push(thread_rng().gen_range(0.0..10.0) as u64);
    }
    let start = Instant::now();
    a.sort();
    let duration = start.elapsed();
    let time = format!("{:?} ms", duration.as_secs_f64() * 1000.0);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(time.into())?)
}
