use std::collections::HashMap;

use env_logger::Builder;
use futures::stream::SelectAll;
use futures::StreamExt;
use log::LevelFilter;
use monolith_core::ClientWriter;
use monolith_core::FlexDirection;
use monolith_core::Monolith;
use monolith_core::{MonolithBuilder, Item, View, Checkbox, Text, TextInput, Button, Client, ClientEvent, ClientReceiver};

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
                    flex_direction: FlexDirection::Row,
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
                    ]
                }
            )
        )
    }

    Item::View(
        View {
            flex_direction: FlexDirection::Column,
            flex: None,
            body: vec![
                Item::View(
                    View {
                        flex_direction: FlexDirection::Row,
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
                                    name: "add".to_string(),
                                    id: "add".to_string(),
                                    title: "Add".to_string(),
                                }
                            )
                        ]
                    }
                ),
                Item::View(
                    View {
                        flex_direction: FlexDirection::Column,
                        flex: None,
                        body: rows
                    }
                )
            ]
        }
    )
}

struct TodoApp {
    todolist: Todolist,
    receivers: SelectAll<ClientReceiver>,
    monolith: Monolith,
    writers: HashMap<usize, ClientWriter>
    // client_futures: FuturesUnordered<Box<dyn Future<Output = (Option<ClientEvent>, Client)>>>
}

impl TodoApp {
    pub fn new() -> TodoApp {
        let monolith = MonolithBuilder::new()
            // .port(8080)
            .build();

        TodoApp {
            todolist: Todolist::new(),
            receivers: SelectAll::new(),
            writers: HashMap::new(),
            monolith: monolith
            // client_futures: FuturesUnordered::new() 
        }
    }

    pub async fn handle_new_client(&mut self, mut client: Client) {
        log::info!("handle new client");

        let item = render_page(&self.todolist);

        client.render(item).await;

        // self.client_futures.push(Box::new(client.next()));

        let id = client.id();

        let (w, r) = client.split();

        self.receivers.push(r);
        self.writers.insert(id, w);
    }

    async fn handle_event(&mut self, event: ClientEvent, client_id: usize) {
        match event {
            ClientEvent::Disconnected => {
                log::info!("client {} disconnected", client_id);
                self.writers.remove(&client_id);
            },
            ClientEvent::OnClick(o) => {
                match o.name.as_str() {
                    "add" => {
                        self.todolist.add(self.todolist.new_item_name.clone());
                        self.todolist.new_item_name = "".to_string();
                    },
                    "completed" => {
                        self.todolist.toggle(o.id);
                    },
                    _ => {}
                }
            },
            ClientEvent::OnTextChanged(o) => {
                match o.name.as_str() {
                    "newTodoItemName" => {
                        self.todolist.new_item_name = o.value;
                    },
                    _ => {}
                }
            },
            ClientEvent::OnKeyDown(event) => {
                if event.keycode == "Enter" {
                    self.todolist.add(self.todolist.new_item_name.clone());
                    self.todolist.new_item_name = String::new();
                }
            },
        }

        match self.writers.get(&client_id) {
            Some(w) => {
                let item = render_page(&self.todolist);

                w.render(item).await;
            },
            None => {
                log::info!("no writer for client {}", client_id);
            }
        }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(client) = self.monolith.accept_client() => {
                    self.handle_new_client(client).await;
                }
                Some((id, event)) = self.receivers.next() => {
                    log::info!("event {:?}", event);

                    self.handle_event(event, id).await;
                }
                else => {
                    log::info!("no more clients");

                    break;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // env_logger::init();

    Builder::new().filter_level(LevelFilter::Info).init();

    log::info!("does this work");

    let app = TodoApp::new();

    app.run().await;
}
