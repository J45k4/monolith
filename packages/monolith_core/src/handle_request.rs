use std::sync::{Arc, atomic::AtomicUsize};

use anyhow::bail;
use hyper::{Response, Body, Request};
use tokio::sync::mpsc;

use crate::{create_ui_client, Client};

pub fn serve_index() -> Response<Body>  {
    let str = format!(r#"
<html>
    <head>
        <title>Your app</title>
    </head>
    <body>
        <script src="/index.js"></script>
    </body>
</html>"#, );

    Response::new(Body::from(str))
}

const index_js_bytes: &[u8] = include_bytes!("../../../public/index.js");

#[derive(Clone)]
pub struct RequestContext {
    pub tx: mpsc::Sender<Client>,
    pub next_client_id: Arc<AtomicUsize>,
}

pub async fn handle_request(mut req: Request<Body>, ctx: RequestContext) -> Result<Response<Body>, anyhow::Error> {
    // Use the connection pool here

    log::info!("handle_request {}", req.uri());

    if hyper_tungstenite::is_upgrade_request(&req) {
        log::info!("there is upgrade request");

        let (response, websocket) = hyper_tungstenite::upgrade(&mut req, None)?;

        log::info!("websocket upgraded");

        let id = ctx.next_client_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        match req.uri().path() {
            "/ui" => {
                let client = create_ui_client(id, websocket);
            },
            &_ | _ => {
                bail!("not allowed url")
            }
        };

        return Ok(response);
    }

    match req.uri().path() {
        "/index.js" => {
            let response = Response::new(Body::from(index_js_bytes));
            Ok(response)
        },
        _ => {
            let response = serve_index();
            Ok(response)
        }
    }
}