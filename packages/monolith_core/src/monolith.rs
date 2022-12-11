use std::sync::Arc;

use tokio::sync::mpsc;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::sync::oneshot;

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

        let intertal = Arc::new(Internal {
            stopper: Some(stopper_send)
        });

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
            interal: intertal
        }
    }
}

pub struct Monolith {
    rx: mpsc::Receiver<Client>,
    interal: Arc<Internal>
}

impl Monolith {
    pub async fn accept_client(&mut self) -> Option<Client> {
        self.rx.recv().await
    }
}