use env_logger::Builder;
use log::LevelFilter;
use monolith_core::{SingleMonolith, MonolithBuilder, Item, FlexDirection, View, Checkbox, Text, TextInput, Button, ClientEvent};

pub struct TodoItem {
    pub name: String,
    pub completed: bool
}

struct Todolist {
    pub new_item_name: String,
    pub items: Vec<TodoItem>
}

impl Todolist {
    pub fn new() -> Todolist {
        Todolist { 
            new_item_name: "".to_string(),
            items: vec![] 
        }
    }

    pub fn toggle(&mut self, name: String) {
        for item in &mut self.items {
            if item.name == name {
                item.completed = !item.completed;
            }
        }
    }

    pub fn add(&mut self, name: String) {
        log::info!("add item to todolist {}", name);

        if name == "" {
            return;
        }
        
        self.items.push(
            TodoItem { 
                name: name, 
                completed: false 
            }
        )
    }
}

fn render_page(todolist: &Todolist) -> Item {
    let mut rows = vec![];
    for item in &todolist.items {
        rows.push(
            Item::View(
                View {
                    flex: None,
                    body: vec![
                        Item::Checkbox(
                            Checkbox {
                                name: "completed".to_string(),
                                id: item.name.to_string(),
                                checked: item.completed
                            }
                        ),
                        Item::Text(
                            Text {
                                text: item.name.clone()
                            }
                        )
                    ],
                    ..Default::default()
                }
            )
        )
    }

    Item::View(
        View {
            flex: None,
            body: vec![
                Item::View(
                    View {
                        flex: None,
                        body: vec![
                            Item::TextInput(
                                TextInput {
                                    name: "newTodoItemName".to_string(),
                                    id: "newTodoItemName".to_string(),
                                    placeholder: "Enter your name".to_string(),
                                    value: todolist.new_item_name.clone()
                                }
                            ),
                            Item::Button(
                                Button {
                                    name: Some("add".to_string()),
                                    title: "Add".to_string(),
                                    ..Default::default()
                                }
                            )
                        ],
                        ..Default::default()
                    }
                ),
                Item::View(
                    View {
                        flex: None,
                        body: rows,
                        ..Default::default()
                    },
                )
            ],
            ..Default::default()
        }
    )
}

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mut todolist = Todolist::new();

    let mut monolith = MonolithBuilder::new().build();

    while let Some((writer, event)) = monolith.recv_next().await {
        match event {
            ClientEvent::OnClick(o) => {
                if let Some(name) = o.name {
                    match name.as_str() {
                        "add" => {
                            todolist.add(todolist.new_item_name.clone());
                            todolist.new_item_name = "".to_string();
                        },
                        "completed" => {
                            todolist.toggle(o.id.unwrap());
                        },
                        _ => {}
                    }
                }
            },
            ClientEvent::OnTextChanged(o) => {
                if let Some(name) = o.name {
                    match name.as_str() {
                        "newTodoItemName" => {
                            todolist.new_item_name = o.value;
                        },
                        _ => {}
                    }
                }
            },
            ClientEvent::OnKeyDown(event) => {
                if event.keycode == "Enter" {
                    todolist.add(todolist.new_item_name.clone());
                    todolist.new_item_name = String::new();
                }
            },
            _ => {}
        }

        let item = render_page(&todolist);

        writer.render(item).await;
    }
}
