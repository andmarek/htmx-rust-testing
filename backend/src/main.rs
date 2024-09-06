use warp::Filter;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["*"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .build();

    // Logging wrapper
    let log = warp::log::custom(|info| {
        println!("{} {} {}", info.method(), info.path(), info.status());
    });

    // Define the route
    let hello = warp::path!("hello" / "warp")
        .map(|| "Hello, Warp!")
        .with(warp::reply::with::header("Access-Control-Allow-Origin", "*"));

    // Combine routes
    let routes = hello
        .with(cors)
        .with(log);

    // Start the server
    println!("Server starting on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
