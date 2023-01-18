use actix_cors::Cors;
use actix_web::{get, App, HttpServer, HttpResponse, Responder, http, middleware::Logger};
use std::{io::Result, env};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Awesome it works 🐻")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_host = env::var("SERVER_HOST").expect("SERVER_HOST Not Found !.");
    let app_port = env::var("SERVER_PORT").expect("SERVER_PORT Not Found !.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let origin = format!("http://{}", &app_url);
    let other_origin = format!("http://localhost:{}", &app_port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin(&origin)
                    .allowed_origin(&other_origin)
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(index)
    })
    .bind(&app_url)?
    .run()
    .await
}