use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use anyhow::Result;
use anyhow::bail;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use tokio::sync::mpsc;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::sync::oneshot;
use futures::stream::SelectAll;
use tokio::task::JoinHandle;

use crate::ClientEvent;
use crate::ClientReceiver;
use crate::ClientWriter;
use crate::create_ui_client;
use crate::handle_request::handle_request;
use crate::handle_request::RequestContext;
use crate::{gui::Client};

struct Worker {
    tx: mpsc::Sender<Client>,
    stopper_recv: oneshot::Receiver<()>,
    port: u16
}

impl Worker {
    async fn run(self) {
        let tx = self.tx.clone();

        let ctx = RequestContext {
            tx: self.tx,
            next_client_id: Arc::new(AtomicUsize::new(0)),
        };
        
        let make_scv = {
            let ctx = ctx.clone();

            make_service_fn(move |_| {
                let ctx = ctx.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        handle_request(req, ctx.clone())
                    }))
                }
            })
        };

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let server = Server::bind(&addr)
            .serve(make_scv)
            .with_graceful_shutdown(async move {

            match self.stopper_recv.await {
                Ok(k) => {},
                Err(e) => {
                    log::error!("error: {}", e);
                },
            }

            log::info!("shutting down server");
        });
    

        log::info!("binding server to port {}", self.port);

        // let (addr, fut) = server.bind_with_graceful_shutdown(([127, 0, 0, 1], self.port), async move {
        //     self.stopper_recv.await.unwrap();

        //     log::info!("shutting down server");
        // }); 

        // log::info!("websocket server listening on {}", addr);

        // fut.await;

        if let Err(e) = server.await {
            log::error!("server error: {}", e);
        }
    }
}

fn create_worker(port: u16) -> (mpsc::Receiver<Client>, oneshot::Sender<()>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel(100);

    let (stopper_send, stopper_recv) = oneshot::channel::<()>();

    let wait = tokio::spawn(async move {
        log::info!("starting worker");

        let worker = Worker {
            tx,
            stopper_recv,
            port: port
        };

        worker.run().await;
    });

    (rx, stopper_send, wait)
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
        let (rx, stopper_send, wait) = create_worker(self.port);

        Monolith {
            rx: rx,
            stopper: stopper_send,
            wait: wait,
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

pub struct Monolith {
    rx: mpsc::Receiver<Client>,
    stopper: oneshot::Sender<()>,
    wait: JoinHandle<()>,
}

impl Monolith
{
    pub async fn accept_client(&mut self) -> Option<Client> {
        self.rx.recv().await
    }

    pub async fn wait(self) {
        self.wait.await;
    }

    pub fn single_threaded(self) -> SingleMonolith {
        SingleMonolith::new(self.rx, self.stopper)
    }
}