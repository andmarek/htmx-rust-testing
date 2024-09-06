use warp::Filter;
use warp::http::Method;

#[tokio::main]
async fn main() {
    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "X-Requested-With", "X-PINGOTHER"])
        .allow_methods(&[Method::GET, Method::POST, Method::OPTIONS])
        .max_age(3600);

    // Define the route
    let hello = warp::path!("hello" / "warp")
        .map(|| "Hello, Warp!");

    // Combine routes and apply CORS
    let routes = hello.with(cors);

    // Start the server
    println!("Server starting on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
