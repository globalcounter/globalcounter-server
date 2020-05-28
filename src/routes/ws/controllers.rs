use crate::prelude::*;
use crate::routes::ws::types::{ClientOutput, WsMessage};
use crate::utils;
use futures::{stream::FuturesUnordered, SinkExt, StreamExt, TryStreamExt};
use lazy_static::lazy_static;
use routerify_websocket::{Message, WebSocket};
use std::collections::HashMap;
use std::convert::TryInto;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

lazy_static! {
    static ref CLIENT_REGISTRY: Mutex<HashMap<String, WebSocket>> = Mutex::new(HashMap::new());
}

pub async fn publish_counter_value(val: i64) -> crate::Result<()> {
    let mut lock = CLIENT_REGISTRY.lock().await;

    let msg: Message = WsMessage { counter: val }.try_into()?;

    let mut all_fut_stream = lock
        .iter_mut()
        .map(|(id, client)| {
            let id = sendify::wrap(id);
            let client = sendify::wrap_mut(client);
            let msg = msg.clone();

            tokio::spawn(async move {
                let client = unsafe { client.unwrap() };

                if let Err(err) = send_counter_value_to_client(client, msg.clone()).await {
                    warn!("Failed to publish counter value to a client: {}", err);

                    if let Err(err) = client.close().await {
                        warn!("Failed to close a client connection: {}", err);
                    }

                    let id = unsafe { id.unwrap().clone() };
                    tokio::spawn(async move {
                        cleanup_client(id).await;
                    });
                }
            })
        })
        .collect::<FuturesUnordered<JoinHandle<()>>>()
        .map_err(|err| err.wrap());

    while let Some(result) = all_fut_stream.next().await {
        if let Err(err) = result {
            warn!("Failed to publish counter value to a client: {}", err);
        }
    }

    drop(lock);

    Ok(())
}

async fn send_counter_value_to_client(client: &mut WebSocket, msg: Message) -> crate::Result<()> {
    client
        .send(msg)
        .await
        .context("Failed to send counter value message to a WebSocket client")
}

async fn cleanup_client(id: String) {
    let mut lock = CLIENT_REGISTRY.lock().await;
    lock.remove(&id);
    info!("Active WS Clients After cleanup: {}", lock.len());
}

pub async fn shake_client_registry() {
    let mut lock = CLIENT_REGISTRY.lock().await;

    for (id, client) in lock.iter_mut() {
        if let Err(err) = client.send(Message::ping(vec![])).await {
            warn!("Failed to ping a client: {}", err);

            if let Err(err) = client.close().await {
                warn!("Failed to close a client connection: {}", err);
            }

            let id = id.clone();
            tokio::spawn(async move {
                cleanup_client(id).await;
            });
        }
    }

    drop(lock);
}

pub async fn push_new_client_to_registry(client: WebSocket) -> crate::Result<()> {
    let mut lock = CLIENT_REGISTRY.lock().await;
    lock.insert(utils::gen_uuid(), client);
    Ok(())
}

pub async fn get_ws_clients() -> crate::Result<Vec<ClientOutput>> {
    let lock = CLIENT_REGISTRY.lock().await;

    let clients = lock
        .iter()
        .map(|(id, client)| ClientOutput {
            id: id.clone(),
            ip_address: client.remote_addr().ip().to_string(),
        })
        .collect();

    Ok(clients)
}
