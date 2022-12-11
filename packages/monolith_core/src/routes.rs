use std::{convert::Infallible, sync::{atomic::AtomicUsize, Arc}};

use tokio::sync::mpsc;
use warp::{path::Exact, Filter, Rejection};

use crate::gui::{Client, handle_ws_conn};

pub fn routes(tx: mpsc::Sender<Client>) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone  {
    let next_client_id = Arc::new(AtomicUsize::new(1));
    
    let ws_path = warp::path("ui")
        .and(warp::ws())
        .and(warp::any().map(move || next_client_id.clone()))
        .and(warp::any().map(move || tx.clone()))
        .map(|ws: warp::ws::Ws, next_client_id: Arc<AtomicUsize>, sender: mpsc::Sender<Client>| {
            log::info!("upgrade request");

            ws.on_upgrade(move |socket| {
                log::info!("New ws client connected");

                handle_ws_conn(socket, sender, next_client_id)
            })
        });
    
    let index = warp::any()
        .and(warp::fs::dir("./public"))
        .or(ws_path);

    index
}