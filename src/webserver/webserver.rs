use std::cell::OnceCell;
use std::thread::JoinHandle;
use axum::Router;
use crate::csctrl::types::CsctrlConfig;

pub struct Webserver {
    thread_restapi: OnceCell<JoinHandle<()>>
}

impl Webserver {
    pub fn webserver() -> Webserver {
        Webserver {
            thread_restapi: OnceCell::new(),
        }
    }

    pub fn init(&self, csctrl_config: &CsctrlConfig) {
        let _ = &self.start_rest_api(csctrl_config);
    }

    fn start_rest_api(&self, csctrl_config: &CsctrlConfig) {
        let ip_port = &csctrl_config.rest_api_address;
        let receive_cslog_path = &csctrl_config.cs_listen_path;

        let api = axum::Router::new()
            .route(receive_cslog_path, axum::routing::post(receive_cslog));

        let _ = &self.prepare_thread_restapi(ip_port.to_string(), api);
        tracing::debug!("Webserver REST API thread created");
        tracing::debug!("Listening to CS2 logs at '{}{}'", ip_port, receive_cslog_path);
    }

    fn prepare_thread_restapi(&self, address: String, router: Router) {
        let _ = &self.thread_restapi.get_or_init(move || {
            return std::thread::spawn(move || {
                boot_thread_restapi(address.to_string(), router);
            });
        });
    }

    pub fn shutdown(&self) {

    }
}

#[tokio::main]
async fn boot_thread_restapi(address: String, router: Router) {
    axum::Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service()).await.unwrap();
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