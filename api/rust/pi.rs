use rand::Rng;

use std::time::Instant;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut rng = rand::thread_rng();
    let start = Instant::now();
    let interval = 10000;
    let mut circle_points = 0;
    let mut square_points = 0;
    let pi = 0.0;
    for _ in 0..interval * interval {
        let rand_x = rng.gen::<f64>() * interval as f64;
        let rand_y = rng.gen::<f64>() * interval as f64;
        let origin_dist = rand_x * rand_x + rand_y * rand_y;

        if origin_dist <= 1.0 {
            circle_points += 1;
        }
        square_points += 1;

        let pi = 4.0 * circle_points as f64 / square_points as f64;
    }
    let duration = start.elapsed();
    let time = format!("{}.{:06} ms", duration.as_secs(), duration.subsec_millis());
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(time.into())?)
}
