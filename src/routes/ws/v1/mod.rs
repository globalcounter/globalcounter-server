use hyper::Body;
use routerify::Router;
use routerify_websocket::{upgrade_ws_with_config, WebSocketConfig};

pub use controllers::{publish_counter_value, shake_client_registry};

mod controllers;
mod handlers;
mod types;

pub fn router() -> Router<Body, crate::Error> {
    let ws_config = WebSocketConfig {
        max_send_queue: None,
        max_message_size: Some(1 * 1024 * 1024),
        max_frame_size: Some(1 * 1024 * 1024),
    };

    Router::builder()
        .get("/connect", upgrade_ws_with_config(handlers::ws_handler, ws_config))
        .get("/clients", handlers::clients_get)
        .build()
        .unwrap()
}
