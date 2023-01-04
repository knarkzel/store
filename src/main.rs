use store::*;
use std::convert::Infallible;
use warp::{Filter, reply::Html};

#[tokio::main]
async fn main() {
    let index = warp::path::end().and_then(index_route);
    let files = warp::path("static").and(warp::fs::dir("./static/"));
    
    println!("Starting server on http://0.0.0.0:8000");
    warp::serve(index.or(files))
        .run(([0, 0, 0, 0], 8000))
        .await;
}

async fn index_route() -> Result<Html<String>, Infallible> {
    let products = fetch_products().await.unwrap();
    Ok(template::Index { products }.render())
}
