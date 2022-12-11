use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use tokio::sync::mpsc;
use futures_util::StreamExt;
use futures_util::SinkExt;
use warp::{ws::{Message, WebSocket}, Error};

use super::{types::ClientEvent, Client};


async fn handle_socket(
    msg: Option<Result<Message, Error>>, 
    tx: &mut mpsc::UnboundedSender<ClientEvent>
) -> bool {
    log::info!("handle_socket");
    
    match msg {
        Some(msg) => {
            match msg {
                Ok(msg) => {
                    if msg.is_text() {
                        let text = msg.to_str().unwrap();

                        let msgs: Vec<ClientEvent> = serde_json::from_str(text).unwrap();

                        for msg in msgs {
                            tx.send(msg).unwrap();
                        } 
                    }

                    true
                },
                Err(err) => {
                    log::error!("Websocket read error {}", err);

                    tx.send(ClientEvent::Disconnected).unwrap();

                    false
                },
            }
        },
        None => {
            log::error!("Websocket closed");

            tx.send(ClientEvent::Disconnected).unwrap();

            false
        },
    }
}

pub async fn handle_ws_conn(
    mut socket: WebSocket, 
    tx: mpsc::Sender<Client>,
    next_client_id: Arc<AtomicUsize>
) {
    log::info!("handle_ws_conn");

    let (sender_tx, sender_rx) = mpsc::unbounded_channel::<ClientEvent>();
    let (receiver_tx, mut receiver_rx) = mpsc::unbounded_channel::<String>();

    let id = next_client_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let ctx = Client::new(id, receiver_tx, sender_rx);

    tx.send(ctx).await.unwrap();

    tokio::pin!(sender_tx);

    log::info!("start listen loop");

    loop {
        tokio::select! {
            msg = socket.next() => {
                log::info!("socket.next");

                if !handle_socket(msg, &mut sender_tx).await {
                    break;
                }
            },
            msg = receiver_rx.recv() => {
                log::info!("receiver_rx.recv");

                match msg {
                    Some(msg) => {
                        socket.send(Message::text(msg)).await.unwrap();
                    },
                    None => {
                        break;
                    }
                }
            }
        }
    }
}