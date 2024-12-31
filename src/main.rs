#![deny(warnings)]
use warp::Filter;

static BODY: &str = r#"
<html>
    <head>
        <title>Rust Web App Deployed with Kamal</title>
    </head>
    <body>
        <h1>Hello, World!</h1>
    </body>
</html>
"#;

#[tokio::main]
async fn main() {
    // Match any request and return the Hello World body
    let routes = warp::any().map(move || {
        warp::reply::html(BODY)
    });

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}