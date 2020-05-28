use crate::prelude::*;
use crate::routes::ws::v1::controllers;
use hyper::{Body, Request, Response};
use routerify_websocket::WebSocket;

pub async fn ws_handler(ws: WebSocket) {
    info!("New WebSocket connection from: {}", ws.remote_addr());

    if let Err(err) = controllers::push_new_client_to_registry(ws).await {
        warn!("Failed to push a new WebSocket client to the registry: {}", err);
    }
}

pub async fn clients_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    let clients = controllers::get_ws_clients()
        .await
        .context("Failed to get WebSocket clients")?;

    resp_200!(clients)
}
