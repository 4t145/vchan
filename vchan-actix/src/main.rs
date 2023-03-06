use actix_cors::Cors;
use actix_web::{
    HttpServer,
    web,
    App
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod services;
mod init;
mod utils;
mod factories;
mod consts;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_data = web::Data::new(init::AppData::init());
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("ssl/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("ssl/cert.pem").unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive()
        .allowed_origin("http://localhost:5173")
        .allowed_origin("http://127.0.0.1")
        .allowed_origin("http://vrp.4t145.com")
        .allowed_origin("https://vrp.4t145.com")
        // .expose_headers(["access-control-allow-origin", "access-control-allow-credentials"])
        .allow_any_method()
        .allow_any_origin()
        .supports_credentials()
        .max_age(3500);

        App::new().app_data(app_data.clone())
        // .wrap(
        //     middleware::DefaultHeaders::new()
        //     .add(("Content-Type", "text/html; charset=utf-8"))
        // )
        .wrap(cors)
        .service(services::register)
        .service(services::user::check_auth)
        .service(services::board::create)
        .service(services::board::get)
        .service(services::thread::post)
        .service(services::thread::get)
        .service(services::post::post)
        .service(services::post::get)
        .service(services::post::batch)
    })
    // .bind_openssl("0.0.0.0:443", builder)?
    .bind(("0.0.0.0", env!("PORT").parse::<u16>().unwrap_or(80)))?
    .run()
    .await
}