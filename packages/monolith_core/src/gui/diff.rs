use crate::gui::types::{ClientAction, AddBack, RemoveInx, Replace};

use super::{gui::Item, types::ItemPath};



fn inner_diff(changes: &mut Vec<ClientAction>, old: &Item, new: &Item, path: ItemPath) {
    log::debug!("diff: {:?} -> {:?}", old, new);

    match (old, new) {
        (Item::View(old), Item::View(new)) => {
            let m = std::cmp::max(
                old.body.len(),
                new.body.len()
            );

            for i in 0..m {
                if i >= old.body.len() {
                    log::debug!("add back");
                    changes.push(ClientAction::AddBack(
                        AddBack {
                            path: path.clone(),
                            item: new.body[i].clone()
                        }
                    ));
                } else if i >= new.body.len() {
                    log::debug!("remove inx");
                    changes.push(
                        ClientAction::RemoveInx(
                            RemoveInx {
                                path: path.clone(),
                                inx: i
                            }
                        )
                    );
                } else {
                    log::debug!("diff child");
                    let mut path = path.clone();
                    path.push(i);

                    inner_diff(changes, &old.body[i], &new.body[i], path);
                }
            }
        }
        _ => {
            if old != new {
                changes.push(
                    ClientAction::Replace(
                        Replace {
                            path: path.clone(),
                            item: new.clone()
                        }
                    )
                );
            }
        }
    }
}

pub fn diff(old: &Item, new: &Item) -> Vec<ClientAction> {
    let mut changes = Vec::new();
    let mut path = Vec::new();

    inner_diff(&mut changes, old, new, path);

    log::debug!("diff changes: {:?}", changes);

    changes
}

#[cfg(test)]
mod tests {
    use crate::gui::{gui::{Item, Text, View, Button, TextInput, Checkbox}, types::{ClientAction, Replace, RemoveInx, AddBack}};

    use super::diff;

    #[test]
    fn it_works() {
        let changes = diff(
            &Item::Text(
                Text {
                    text: "Hello".to_string(),
                }
            ),
            &Item::Text(
                Text {
                    text: "Hello World".to_string(),
                }
            )
        );

        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0],
            ClientAction::Replace(
                Replace {
                    path: vec![],
                    item: Item::Text(
                        Text {
                            text: "Hello World".to_string(),
                        }
                    )
                }
            )
        );
    }

    #[test]
    fn test_add_child() {
        let changes = diff(
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "Hello".to_string(),
                            }
                        )
                    ]
                }
            ),
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "Hello".to_string(),
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "World".to_string(),
                            }
                        )
                    ]
                }
            )
        );

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], ClientAction::AddBack(
            AddBack {
                path: vec![],
                item: Item::Text(
                    Text {
                        text: "World".to_string(),
                    }
                )
            }
        ));
    }

    #[test]
    fn test_remove_child() {
        let changes = diff(
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "Hello".to_string(),
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "World".to_string(),
                            }
                        )
                    ]
                }
            ),
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "Hello".to_string(),
                            }
                        )
                    ]
                }
            )
        );

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], ClientAction::RemoveInx(
            RemoveInx {
                path: vec![],
                inx: 1
            }
        ));
    }

    #[test]
    fn test_button_changed_to_text() {
        let changes = diff(
            &Item::Button(
                Button {
                    id: "qwerty".to_string(),
                    name: "qwerty".to_string(),
                    title: "Hello".to_string()
                }
            ),
            &Item::Text(
                Text {
                    text: "Hello".to_string()
                }
            )
        );

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], ClientAction::Replace(
            super::Replace {
                path: vec![],
                item: Item::Text(
                    Text {
                        text: "Hello".to_string()
                    }
                )
            }
        ));
    }

    #[test]
    fn test_diffing_more_complicated() {
        let changes = diff(
            &Item::View(
                View {
                    body: vec![
                        Item::View (
                            View {
                                body: vec![
                                    Item::TextInput(
                                        TextInput {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            value: "".to_string(),
                                            placeholder: "Hello".to_string()
                                        }
                                    ),
                                    Item::Button(
                                        Button {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            title: "Hello".to_string()
                                        }
                                    )
                                ]
                            }
                        ),
                        Item::View(
                            View {
                                body: vec![]
                            }
                        )
                    ]
                }
            ),
            &Item::View(
                View {
                    body: vec![
                        Item::View (
                            View {
                                body: vec![
                                    Item::TextInput(
                                        TextInput {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            value: "newvalue".to_string(),
                                            placeholder: "Hello".to_string()
                                        }
                                    ),
                                    Item::Button(
                                        Button {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            title: "Hello".to_string()
                                        }
                                    )
                                ]
                            }
                        ),
                        Item::View(
                            View {
                                body: vec![
                                    Item::View(
                                        View { 
                                            body: vec![
                                                Item::Text(
                                                    Text {
                                                        text: "Newrow".to_string()
                                                    }
                                                )
                                            ]
                                        }
                                    )
                                ]
                            }
                        )
                    ]
                }
            )
        );

        assert_eq!(changes.len(), 2);

        let change = &changes[0];

        assert_eq!(change, &ClientAction::Replace(
            super::Replace {
                path: vec![0, 0],
                item: Item::TextInput(
                    TextInput { 
                        id: "qwerty".to_string(), 
                        name: "qwerty".to_string(),
                        placeholder: "Hello".to_string(), 
                        value: "newvalue".to_string()
                    }
                )
            }
        ));

        let change = &changes[1];

        assert_eq!(change, &ClientAction::AddBack(
            AddBack {
                path: vec![1],
                item: Item::View(
                    View {
                        body: vec![
                            Item::Text(
                                Text {
                                    text: "Newrow".to_string()
                                }
                            )
                        ]
                    }
                )
            }
        ));
    }

    #[test]
    fn test_diffing_more_complicated2() {
        let changes = diff(
            &Item::View(
                View {
                    body: vec![
                        Item::View (
                            View {
                                body: vec![
                                    Item::TextInput(
                                        TextInput {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            value: "".to_string(),
                                            placeholder: "Hello".to_string()
                                        }
                                    ),
                                    Item::Button(
                                        Button {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            title: "Hello".to_string()
                                        }
                                    )
                                ]
                            }
                        ),
                        Item::View(
                            View {
                                body: vec![
                                    Item::View(
                                        View { 
                                            body: vec![
                                                Item::Checkbox(
                                                    Checkbox {
                                                        id: "qwerty".to_string(),
                                                        name: "qwerty".to_string(),
                                                        checked: false
                                                    }
                                                ),
                                                Item::Text(
                                                    Text {
                                                        text: "Makkara".to_string()
                                                    }
                                                )
                                            ]
                                        }
                                    )
                                ]
                            }
                        )
                    ]
                }
            ),
            &Item::View(
                View {
                    body: vec![
                        Item::View (
                            View {
                                body: vec![
                                    Item::TextInput(
                                        TextInput {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            value: "newvalue".to_string(),
                                            placeholder: "Hello".to_string()
                                        }
                                    ),
                                    Item::Button(
                                        Button {
                                            id: "qwerty".to_string(),
                                            name: "qwerty".to_string(),
                                            title: "Hello".to_string()
                                        }
                                    )
                                ]
                            }
                        ),
                        Item::View(
                            View {
                                body: vec![
                                    Item::View(
                                        View { 
                                            body: vec![
                                                Item::Checkbox(
                                                    Checkbox {
                                                        id: "qwerty".to_string(),
                                                        name: "qwerty".to_string(),
                                                        checked: false
                                                    }
                                                ),
                                                Item::Text(
                                                    Text {
                                                        text: "Makkara".to_string()
                                                    }
                                                )
                                            ]
                                        }
                                    )
                                ]
                            }
                        )
                    ]
                }
            )
        );

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], ClientAction::Replace(
            super::Replace {
                path: vec![0, 0],
                item: Item::TextInput(
                    TextInput { 
                        id: "qwerty".to_string(), 
                        name: "qwerty".to_string(),
                        placeholder: "Hello".to_string(), 
                        value: "newvalue".to_string()
                    }
                )
            }
        ));
    }
}