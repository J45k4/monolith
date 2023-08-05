use std::net::SocketAddr;
use std::sync::Arc;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::server::conn;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use crate::prog::Prog;
use crate::prog::ProgCtx;
use crate::prog::ToProg;

#[derive(Clone)]
enum ResBody {
    StaticHtml(String),
    StaticJSON(String),
}

#[derive(Clone)]
struct Route {
    path: String,
    prog: Arc<dyn Prog + Send + Sync>
}


pub struct Monolith {
    port: u16,
    routes: Vec<Route>
}

impl Monolith
{
    pub fn new() -> Self {
        Self {
            port: 80,
            routes: Vec::new()
        }
    }

    pub fn add(mut self, path: &str, to_prog: impl ToProg) -> Self {
        let prog = to_prog.to_prog();

        self.routes.push(Route {
            path: path.to_string(),
            prog: prog
        });

        self
    } 

    // pub fn add_script_path(mut self, path: impl AsRef<Path>) -> Self {
    //     let path = path.as_ref();

    //     let script = fs::read_to_string(path).unwrap();

    //     let mut parser = Parser::new(&script);
    //     let ast = parser.parse();

    //     self.process_ast(&ast);
        
    //     self
    // }

    // fn process_ast(&mut self, ast: &Vec<ASTNode>) {
    //     for node in ast {
    //         match node {
    //             ASTNode::StructIns(ins) => {
    //                 if ins.name == "PostRoute" {
    //                     let mut route = Route {
    //                         path: String::new(),
    //                         prog: ASTNode::Lit(Value::Str(String::new()))
    //                     };
    //                     for prop in &ins.probs {
    //                         match prop.name.as_str() {
    //                             "path" => {
    //                                 match &*prop.value {
    //                                     ASTNode::Lit(value) => {
    //                                         if let Value::Str(s) = value {
    //                                             println!("path: {}", s);
    //                                             route.path = s.clone();
    //                                         }
    //                                     },
    //                                     _ => {}
    //                                 }
    //                             }
    //                             "body" => {
    //                                 route.res_body = *prop.value.clone();
    //                             },
    //                             _ => {}
    //                         }
    //                     }

    //                     self.routes.push(route);
    //                 }
    //                 println!("struct: {}", ins.name);
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    pub fn listen(mut self, port: u16) -> Self {
        self.port = port;
        
        self
    }

    pub async fn start(self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            log::info!("new client connected");

            match conn::Http::new().serve_connection(stream, service_fn(
                |req| {
                    let uri = req.uri();

                    let mut route = {
                        let mut route = None;

                        for r in &self.routes {
                            if uri.path() == r.path {
                                route = Some(r.clone());
                                break;
                            }
                        }

                        route
                    };

                    async move {
                        match route {
                            Some(route) => {
                                handle_req(req, route).await
                            },
                            None => {
                                Ok(Response::new(Body::from("Not found")))
                            }
                        }
                    }
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

async fn handle_req(req: Request<Body>, route: Route) -> anyhow::Result<Response<Body>> {
    // let mut s = String::new();
    // build_node_html(&mut s, &route.res_body);

    let html = route.prog.html(ProgCtx {});
    
    Ok(Response::new(Body::from(html)))
}