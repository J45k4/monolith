use env_logger::Builder;
use log::LevelFilter;
use monolith_core::{MonolithBuilder, ClientEvent, Item, View, Text, TextInput, does_route_match};
use page::render_page;
use state::State;

mod state;
mod page;

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mut state = State::new();

    let mut monolith = MonolithBuilder::new().build();

    while let Some((writer, event)) = monolith.recv_next().await {
        match event {
            ClientEvent::PathChanged(p) => {
                log::info!("{:?}", p);

               state.path = p.path;
            },
            ClientEvent::OnClick(o) => {
                match o.name.as_str() {
                    "create_note" => {
                        let not = state.create_new_note();
                        let path = format!("/note/{}", not.id);

                        writer.navigate(path).await;
                    },
                    _ => {}
                }
            }
            _ => {}
        }

        let page = render_page(&state);

        writer.render(page).await;
    }

    monolith.wait().await;
}
