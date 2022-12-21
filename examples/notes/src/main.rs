use env_logger::Builder;
use log::LevelFilter;
use monolith_core::MonolithBuilder;

struct Router {

}

impl Router {
    pub fn new() -> Router {
        Router {}
    }

    pub fn use_route(&self) {

    }
}

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();
    // let router = Router::new();

    // router.use_route();

    let monolith = MonolithBuilder::new().build();

    monolith.wait().await;

    // monolith.
}
