use ndarray::Array;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::time::Instant;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let x = 300;
    let y = 300;
    let a = Array::random((x, y), Uniform::new(0., 10.));
    let b = Array::random((x, y), Uniform::new(0., 10.));
    let start = Instant::now();
    a.dot(&b);
    let duration = start.elapsed();
    let time = format!("{:?} ms", duration.as_secs_f64() * 1000.0);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(time.into())?)
}
