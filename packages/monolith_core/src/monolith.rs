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
use crate::{gui::Client, routes::routes};

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

fn create_worker(port: u16) -> (mpsc::Receiver<Client>, oneshot::Sender<()>) {
    let (tx, rx) = mpsc::channel(100);

    let (stopper_send, stopper_recv) = oneshot::channel::<()>();

    tokio::spawn(async move {
        log::info!("starting worker");

        let worker = Worker {
            tx,
            stopper_recv,
            port: port
        };

        worker.run().await;
    });

    (rx, stopper_send)
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

    pub fn build(self) -> Monolith<()> {
        let (rx, stopper_send) = create_worker(self.port);

        Monolith {
            rx: rx,
            stopper: stopper_send,
            ctx: ()
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

    pub fn get_writers(&self) -> &HashMap<usize, ClientWriter> {
        &self.writers
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

pub struct Monolith<T>
where T: Send + Clone
{
    rx: mpsc::Receiver<Client>,
    stopper: oneshot::Sender<()>,
    ctx: T
}

impl<T> Monolith<T>
where T: Send + Clone
{
    pub async fn accept_client(&mut self) -> Option<Client> {
        self.rx.recv().await
    }

    pub fn single_threaded(self) -> SingleMonolith {
        SingleMonolith::new(self.rx, self.stopper)
    }
}