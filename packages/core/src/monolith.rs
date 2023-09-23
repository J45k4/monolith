use std::net::SocketAddr;
use flexscript::RunResult;
use flexscript::Value;
use flexscript::Vm;
use hyper::Body;
use hyper::Response;
use hyper::server::conn;
use hyper::service::service_fn;
use tokio::net::TcpListener;

use crate::html::Html;

#[derive(Clone)]
struct Route {
    path: String,
    blk: u32
}


pub struct Monolith {
    port: u16,
    routes: Vec<Route>,
    vm: Vm
}

impl Monolith
{
    pub fn new() -> Self {
        Self {
            port: 80,
            routes: Vec::new(),
            vm: Vm::new()
        }
    }

    pub fn add(mut self, path: &str, code: &str) -> Self {
        let blk = self.vm.compile_code(code);

        self.routes.push(Route {
            path: path.to_string(),
            blk: blk
        });

        self
    }

    pub fn listen(mut self, port: u16) -> Self {
        self.port = port;
        
        self
    }

    pub async fn start(mut self) {
        log::info!("listening {}", self.port);
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            log::info!("new client connected");

            match conn::Http::new().serve_connection(stream, service_fn(
                |req| {
                    let uri = req.uri();

                    let route = {
                        let mut route = None;

                        for r in &self.routes {
                            if uri.path() == r.path {
                                route = Some(r.clone());
                                break;
                            }
                        }

                        route
                    };

                    let res = match route {
                        Some(route) => {
                            let res = self.vm.run_blk(route.blk, Value::None);

                            match res {
                                RunResult::Value(value) => {
                                    let text = Html::from(value).to_string();
                                    Response::new(Body::from(text))
                                },
                                RunResult::Await { stack_id, value } => {
                                    Response::new(Body::from("Not found"))
                                },
                                RunResult::None => {
                                    Response::new(Body::from("Not found"))
                                },
                            }
                        },
                        None => {
                            Response::new(Body::from("Not found"))
                        }
                    };

                    async move { 
                        wrap_res(res).await
                    }
                }
            )).await {
                Ok(_) => {},
                Err(err) => {
                    println!("error: {}", err);
                }
            }
        }
    }
}

async fn wrap_res(res: Response<Body>) -> anyhow::Result<Response<Body>> {
    Ok(res)
}