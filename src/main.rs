/*
Reference : https://docs.rs/axum-server/latest/src/http_and_https/http_and_https.rs.html#48

IMPORTANT: This code will require you to install CMake (because of the tls_rustls dependency),
and you will have issues while running it if you do not have Visual Studio installed.
(as indicated from the trace when I do 'cargo run', under --stderr)

HTTP URL: http://127.0.0.1:3000
HTTPS URL: https://127.0.0.1:3443

You immediately notice that the HTTPS URL is served in HTTPS, as Firefox warns you about the website's security.
Also, if you take the same address and port and try to access it via HTTP (http://127.0.0.1:3443), you receive 
weird symbols as output because the data is encrypted!
*/

use axum::{http::uri::Uri, response::Redirect, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let http = tokio::spawn(http_server());
    let https = tokio::spawn(https_server());

    let _ = tokio::join!(http, https);
}

//IMPORTANT! This is the http server, which redirects to HTTPS. Now this may seem weird, because
//  the servers are operating on two different ports (3000 for HTTP, 3443 for HTTPS), but from the 
//  user's perspective, once the service has a domain, the process will be seamless (no port noticed!)
async fn http_server() {
    let app = Router::new().route("/", get(http_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("http listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn http_handler(uri: Uri) -> Redirect {
    let uri = format!("https://127.0.0.1:3443{}", uri.path());

    Redirect::temporary(&uri)
}

async fn https_server() {
    //basic handler that prints hello world and accepts no parameter
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    //paths are from project root (i.e. RustAPIHTTPSTutorial folder)
    let config = RustlsConfig::from_pem_file(
        "self_signed_certs/cert.pem",
        "self_signed_certs/key.pem",
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3443));
    println!("https listening on {}", addr);
    //This is the method that binds your encryption files to the server, and allows it to operate in HTTPS
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}