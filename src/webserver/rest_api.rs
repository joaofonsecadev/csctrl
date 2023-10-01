#[tokio::main]
pub async fn start_rest_api() {
    let ip_port = "0.0.0.0:27016";
    let receive_cslog_path = "/cslog";

    let api = axum::Router::new()
        .route(receive_cslog_path, axum::routing::post(receive_cslog));

    tracing::info!("Starting REST API at '{}'", ip_port);
    tracing::info!("Listening to CS2 logs at '{}'", receive_cslog_path);
    axum::Server::bind(&ip_port.parse().unwrap())
        .serve(api.into_make_service()).await.unwrap();
}

async fn receive_cslog(request: axum::http::Request<axum::body::Body>) {
    let mut request_headers = "".to_string();
    for (key, value) in request.headers().iter() {
        request_headers += &format!("{}{:?}\n", key, value);
    }

    let request_body = std::str::from_utf8(&hyper::body::to_bytes(request.into_body())
        .await.unwrap()).unwrap().to_string();

    tracing::trace!("Received CS2 log. Content:\nHeaders\n{}\nBody\n{}", request_headers, request_body);
}