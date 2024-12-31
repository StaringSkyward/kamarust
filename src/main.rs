use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

static BODY: &str = r#"
<html>
    <head>
        <title>Example Rust Web App Deployed with Kamal</title>
    </head>
    <body>
        <h1>Hello, World!</h1>
        <p>Version 0.2</p>
    </body>
</html>
"#;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(BODY)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .wrap(Logger::default())


    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
