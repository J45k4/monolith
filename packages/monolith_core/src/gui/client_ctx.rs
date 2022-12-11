use std::{sync::Arc, pin::Pin, task::{Context, Poll}};

use futures_util::Stream;
use tokio::sync::{mpsc, Mutex};

use crate::gui::{diff::{self, diff}, types::{ClientAction, Replace}};

use super::{types::ClientEvent, gui::Item};

#[derive(Debug)]
struct Shared {
    last_root: Option<Item>
}

fn render(
    tx: &mpsc::UnboundedSender<String>,
    shared: &mut Shared,
    new_root: Item
) {
    let changes = match shared.last_root.as_ref() {
        Some(last_root) => diff(last_root, &new_root),
        None => vec![ClientAction::Replace(Replace { path: vec![], item: new_root.clone() })]
    };

    if changes.len() == 0 {
        return;
    }

    shared.last_root.replace(new_root.clone());

    log::info!("sending changes: {:?}", changes);

    let str = serde_json::to_string(&changes).unwrap();

    match tx.send(str) {
        Ok(_) => {},
        Err(_) => {},
    }
}

pub struct ClientRenderer {
    tx: mpsc::UnboundedSender<String>,
    shared: Arc<Mutex<Shared>>
}

impl ClientRenderer {
    pub async fn render(&self, root: Item) {
        log::info!("render root");

        let mut shared = self.shared.lock().await;

        render(&self.tx, &mut shared, root);
    }
}

pub struct ClientWriter {
    tx: mpsc::UnboundedSender<String>,
    shared: Arc<Mutex<Shared>>
}

impl Clone for ClientWriter {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            shared: self.shared.clone()
        }
    }
}

impl Drop for ClientWriter {
    fn drop(&mut self) {
        log::info!("dropping ClientWriter");
    }
}

impl ClientWriter {
    pub async fn render(&self, root: Item) {
        log::info!("render root");

        let mut shared = self.shared.lock().await;

        render(&self.tx, &mut shared, root);
    }
}

pub struct ClientReceiver {
    id: usize,
    rx: mpsc::UnboundedReceiver<ClientEvent>
}

impl Stream for ClientReceiver {
    type Item = (usize, ClientEvent);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let r = self.rx.poll_recv(cx);

        match r {
            Poll::Ready(Some(event)) => Poll::Ready(Some((self.id, event))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending
        }
    }
}

impl ClientReceiver {
    pub async fn recv(&mut self) -> Option<ClientEvent> {
        self.rx.recv().await
    }
}

#[derive(Debug)]
pub struct Client {
    id: usize,
    tx: mpsc::UnboundedSender<String>,
    rx: mpsc::UnboundedReceiver<ClientEvent>,
    shared: Arc<Mutex<Shared>>
}

impl Client {
    pub fn new(
        id: usize,
        tx: mpsc::UnboundedSender<String>, 
        rx: mpsc::UnboundedReceiver<ClientEvent>
    ) -> Self {
        Self {
            id: id,
            tx,
            rx,
            shared: Arc::new(Mutex::new(Shared {
                last_root: None
            }))
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub async fn get_renderer(&self) -> ClientRenderer {
        ClientRenderer {
            tx: self.tx.clone(),
            shared: self.shared.clone()
        }
    }

    pub async fn render(&self, root: Item) {
        log::info!("render root");

        let mut shared = self.shared.lock().await;

        render(&self.tx, &mut shared, root);
    }

    pub async fn next(&mut self) -> Option<ClientEvent> {
        self.rx.recv().await
    }

    pub fn split(self) -> (ClientWriter, ClientReceiver) {
        (
            ClientWriter { tx: self.tx, shared: self.shared }, 
            ClientReceiver {
                id: self.id,
                rx: self.rx 
            }
        )
    }
}



        // let results = if let Some(last_root) = last_root.as_ref() {
        //     log::info!("comparing last root with new root");
             
        //     let changes = diff(last_root, &root);

        //     for change in changes {
        //         let msg = serde_json::to_string(&change).unwrap();
        //         self.tx.send(msg).unwrap();
        //     }
        // } else {
        //     let msg = serde_json::to_string(&ClientAction::Replace(Replace {
        //         path: Vec::new(),
        //         item: root.clone()
        //     })).unwrap();

        //     log::info!("Sending initial root: {}", msg);

            
        // }

        // let msg = ClientAction::Replace(
        //     Replace {
        //         path: vec![], 
        //         item: root
        //     }
        // );

        // let str = serde_json::to_string(&msg).unwrap();

        // self.tx.send(str).unwrap();