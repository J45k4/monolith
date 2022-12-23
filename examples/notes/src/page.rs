use monolith_core::{TextInput, Item, Text, does_route_match, View, Button};

use crate::state::State;

// fn navbar() -> Item {

// }

pub fn render_page(state: &State) -> Item {
    if let Some(map) = does_route_match("/note/:nodeId", &state.path) {
        let id = map.get("nodeId").unwrap();

        return match state.get_note(id) {
            Some(note) => {
                return (
                    Item::View(
                        View {
                            flex: None,
                            body: vec![
                                Item::Text(
                                    Text {
                                        text: note.title.clone(),
                                    }
                                ),
                            ],
                            ..Default::default()
                        }
                    )
                )
            },
            None => {
                return (
                    Item::View(
                        View {
                            flex: None,
                            body: vec![
                                Item::Text(
                                    Text {
                                        text: "Note not found".to_string(),
                                    }
                                ),
                            ],
                            ..Default::default()
                        }
                    )
                )
            },
        }
    }

    Item::View(
        View {
            flex: None,
            body: vec![
                Item::Button(
                    Button {
                        name: Some("create_note".to_string()),
                        id: Some("create_note".to_string()),
                        title: "Create note".to_string(),
                    }
                ),
            ],
            ..Default::default()
        }
    )
}