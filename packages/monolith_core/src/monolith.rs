use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::sync::oneshot;
use futures::stream::SelectAll;

use crate::ClientEvent;
use crate::ClientReceiver;
use crate::ClientWriter;
use crate::{gui::{self, Client}, routes::routes};

struct Worker {
    tx: mpsc::Sender<Client>,
    stopper_recv: oneshot::Receiver<()>,
    port: u16
}

impl Worker {
    async fn run(self) {
        let tx = self.tx.clone();

        let routes = routes(tx.clone());
        
        let server = warp::serve(routes);

        log::info!("binding server to port {}", self.port);

        let (addr, fut) = server.bind_with_graceful_shutdown(([127, 0, 0, 1], self.port), async move {
            self.stopper_recv.await.unwrap();

            log::info!("shutting down server");
        }); 

        log::info!("websocket server listening on {}", addr);

        fut.await;

        log::info!("websocket server stopped")
    }
}

struct Internal {
    stopper: Option<oneshot::Sender<()>>,
}

impl Drop for Internal {
    fn drop(&mut self) {
        if let Some(stopper) = self.stopper.take() {
            stopper.send(()).unwrap();
        }
    }
}

pub struct MonolithBuilder {
    port: u16
}

impl MonolithBuilder {
    pub fn new() -> Self {
        Self {
            port: 33445
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;

        self
    }

    pub fn build(self) -> Monolith {
        let (tx, rx) = mpsc::channel(100);

        let (stopper_send, stopper_recv) = oneshot::channel::<()>();

        // let intertal = Arc::new(Internal {
        //     stopper: Some(stopper_send)
        // });

        tokio::spawn(async move {
            log::info!("starting worker");

            let worker = Worker {
                tx,
                stopper_recv,
                port: self.port
            };

            worker.run().await;
        });

        Monolith {
            rx: rx,
            stopper: stopper_send
            // interal: intertal
        }
    }
}

pub enum NextEvent {
    NewClient(ClientWriter, ),
    NewEvent(ClientWriter, ClientEvent)
}

pub struct SingleMonolith {
    rx: mpsc::Receiver<Client>,
    receivers: SelectAll<ClientReceiver>,
    writers: HashMap<usize, ClientWriter>,
    stopper: oneshot::Sender<()>,
}

impl SingleMonolith {
    fn new(
        rx: mpsc::Receiver<Client>, 
        stopper: oneshot::Sender<()>
    ) -> Self {
        Self {
            rx: rx,
            stopper: stopper,
            receivers: SelectAll::new(),
            writers: HashMap::new()
        }
    }

    pub async fn recv_next(&mut self) -> Option<(ClientWriter, ClientEvent)> {
        loop {
            tokio::select! {
                Some(client) = self.rx.recv() => {
                    let id = client.id();

                    let (writer, receiver) = client.split();

                    self.receivers.push(receiver);

                    self.writers.insert(id, writer);
                },
                Some((id, event)) = self.receivers.next() => {
                    let writer = self.writers.get(&id).unwrap();

                    break Some((writer.clone(), event));
                }
            };
        }
    }
}

pub struct Monolith {
    rx: mpsc::Receiver<Client>,
    stopper: oneshot::Sender<()>,
}

impl Monolith {
    pub async fn accept_client(&mut self) -> Option<Client> {
        self.rx.recv().await
    }

    pub fn single_threaded(self) -> SingleMonolith {
        SingleMonolith::new(self.rx, self.stopper)
    }
}