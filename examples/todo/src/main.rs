use std::process::Output;

use futures::future::SelectAll;
use futures::stream::select_all;
use futures::{stream::FuturesUnordered, Future};
use futures::StreamExt;
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
            body: vec![
                Item::View(
                    View {
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
                        body: rows
                    }
                )
            ]
        }
    )
}

struct TodoApp {
    todolist: Todolist,
    // client_futures: FuturesUnordered<Box<dyn Future<Output = (Option<ClientEvent>, Client)>>>
}

impl TodoApp {
    pub fn new() -> TodoApp {
        TodoApp {
            todolist: Todolist::new(),
            // client_futures: FuturesUnordered::new() 
        }
    }

    pub fn render(&self) -> Item {
        render_page(&self.todolist)
    }

    pub async fn handle_new_client(&mut self, mut client: Client) {
        // self.client_futures.push(Box::new(client.next()));
    }

    async fn handle_event(&mut self, event: ClientEvent, client: Client) {
        match event {
            ClientEvent::Disconnected => todo!(),
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
    }

    pub async fn run(mut self) {
        let mut monolith = MonolithBuilder::new()
            .port(8080)
            .build();

        let all: SelectAll<ClientReceiver> = SelectAll::new();

        let v: Vec<ClientReceiver> = vec![];

        let s = select_all(v);


        // loop {
        //     tokio::select! {
        //         Some(mut client) = monolith.accept_client() => {
        //             self.handle_new_client(client).await;
        //         }
        //         // Some((Some(event), client)) = self.client_futures.next() => {
        //         //     log::info!("event {:?}", event);

        //         //     self.handle_event(event, client).await;
        //         // }
        //     }
        // }
    }
}

#[tokio::main]
async fn main() {

    let app = TodoApp::new();

    app.run().await;

    // let mut todolist = Todolist::new();

    // // let client = .await.unwrap();

    // // let mut clients: Vec<ClientCtx> = vec![];
    // let mut futures = FuturesUnordered::new();

    // // for client in clients {
    // //     futures.push(async {
    // //         let next = client.next().await;

    // //         next
    // //     });
    // // }

    // loop {
        
    // }
}
