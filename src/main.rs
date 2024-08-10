use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .sh export them anyway, but just to be sure both parties are independent
    dotenvy::dotenv().ok();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/etc/ssl/private/ammir.dev.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("/etc/ssl/certs/ammir.dev.pem")
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "static").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind_openssl(
        String::from("127.0.0.1:") + &dotenvy::var("LOCAL_PORT").expect("expected LOCAL_PORT"),
        builder,
    )?
    .run()
    .await
}
