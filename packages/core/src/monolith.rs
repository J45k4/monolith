use std::net::SocketAddr;
use flexscript::Value;
use flexscript::Vm;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::server::conn;
use hyper::service::service_fn;
use tokio::net::TcpListener;

#[derive(Clone)]
enum ResBody {
    StaticHtml(String),
    StaticJSON(String),
}

#[derive(Clone)]
struct Route {
    path: String,
    blk: usize
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
                            
                            let text = format!("{:?}", res);

                            Response::new(Body::from(text))
                        },
                        None => {
                            Response::new(Body::from("Not found"))
                        }
                    };

                    async move { 
                        wrap_res(res).await
                    }

                    // async move {
                    //     match route {
                    //         Some(route) => {
                    //             handle_req(req, route).await
                    //         },
                    //         None => {
                    //             Ok(Response::new(Body::from("Not found")))
                    //         }
                    //     }
                    // }
                    // async move {
                    //     bail!("test");

                    //     Ok(Response::new(Body::from("Not found")))
                    // }
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

// async fn handle_req(req: Request<Body>, route: Route) -> anyhow::Result<Response<Body>> {
//     // let mut s = String::new();
//     // build_node_html(&mut s, &route.res_body);

//     let html = 
    
//     Ok(Response::new(Body::from(html)))
// }