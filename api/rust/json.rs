use serde_json::{Map, Value};
use std::time::Instant;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let response =
        reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon?limit=100000&offset=0").unwrap();
    let json = response.text().unwrap();

    let start = Instant::now();
    let parsed: Value = serde_json::from_str(&json)?;
    let _obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    let duration = start.elapsed();

    let time = format!("{:?} ms", duration.as_secs_f64() * 1000.0);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(time.into())?)
}
