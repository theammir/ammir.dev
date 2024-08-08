use std::net::SocketAddr;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // build our application with a route
    let static_dir = ServeDir::new("static");
    let app = Router::new().nest_service("/", static_dir);

    let tls_config = RustlsConfig::from_pem_file(
        "/etc/ssl/certs/ammir.dev.pem",
        "/etc/ssl/private/ammir.dev.pem",
    )
    .await
    .unwrap();

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4331));
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
