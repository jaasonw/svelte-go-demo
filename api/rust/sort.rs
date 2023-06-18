use ndarray::arr2;
use rand::Rng;
use std::time::Instant;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

fn fill_shape(x: i32, y: i32) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for x in 0..(x * y) {
        a.push(rng.gen_range(0.0..1.0));
    }
    ndarray::Array::from_shape_vec((x as usize, y as usize), a);
    return a;
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    let x = 300;
    let y = 300;
    let a = arr2(&fill_shape(x, y));
    let b = arr2(&fill_shape(x, y));
    let start = Instant::now();
    a.dot(&b);
    let duration = start.elapsed();
    let time = format!("{}.{:06} ms", duration.as_secs(), duration.subsec_millis());
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(time.into())?)
}
