use crate::{gui::{types::{ClientAction, AddBack, RemoveInx, Replace}, edit_distance::{get_minimum_edits, EditOperation}}, AddFront, InsertAt, ReplaceAt};

use super::{gui::Item, types::ItemPath};



fn inner_diff(changes: &mut Vec<ClientAction>, old: &Item, new: &Item, path: ItemPath) {
    log::debug!("diff: {:?} -> {:?}", old, new);

    match (old, new) {
        (Item::View(old), Item::View(new)) => {
            let edits = get_minimum_edits(&old.body, &new.body);

            for edit in edits {
                match edit {
                    EditOperation::InsertFirst(item) => {
                        changes.push(
                            ClientAction::AddFront(
                                AddFront {
                                    path: path.clone(),
                                    item: item
                                }
                            )
                        );
                    },
                    EditOperation::InsertAt(index, item) => {
                        changes.push(
                            ClientAction::InsertAt(
                                InsertAt {
                                    path: path.clone(),
                                    inx: index,
                                    item: item
                                }
                            )
                        );
                    },
                    EditOperation::RemoveAt(index) => {
                        changes.push(
                            ClientAction::RemoveInx(
                                RemoveInx {
                                    path: path.clone(),
                                    inx: index
                                }
                            )
                        );
                    },
                    EditOperation::ReplaceAt(i, item) => {
                        let mut path = path.clone();
                        path.push(i);
    
                        inner_diff(changes, &old.body[i], &new.body[i], path);
                    },
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
    use crate::{gui::{gui::{Item, Text, View, Button, TextInput, Checkbox}, types::{ClientAction, Replace, RemoveInx, AddBack}}, AddFront, InsertAt};

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
    fn test_add_to_back() {
        let changes = diff(
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "Hello".to_string(),
                            }
                        )
                    ],
                    ..Default::default()
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
                    ],
                    ..Default::default()
                }
            )
        );

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], ClientAction::InsertAt(
            InsertAt {
                path: vec![],
                inx: 0,
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
                    ],
                    ..Default::default()
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
                    ],
                    ..Default::default()
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
                                ],
                                ..Default::default()
                            }
                        ),
                        Item::View(
                            View {
                                body: vec![],
                                ..Default::default()
                            }
                        )
                    ],
                    ..Default::default()
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
                                ],
                                ..Default::default()
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
                                            ],
                                            ..Default::default()
                                        }
                                    )
                                ],
                                ..Default::default()
                            }
                        )
                    ],
                    ..Default::default()
                }
            )
        );

        println!("{:?}", changes);

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

        assert_eq!(change, &ClientAction::AddFront(
            AddFront {
                path: vec![1],
                item: Item::View(
                    View {
                        body: vec![
                            Item::Text(
                                Text {
                                    text: "Newrow".to_string()
                                }
                            )
                        ],
                        ..Default::default()
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
                                ],
                                ..Default::default()
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
                                            ],
                                            ..Default::default()
                                        }
                                    )
                                ],
                                ..Default::default()
                            }
                        )
                    ],
                    ..Default::default()
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
                                ],
                                ..Default::default()
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
                                            ],
                                            ..Default::default()
                                        }
                                    )
                                ],
                                ..Default::default()
                            }
                        )
                    ],
                    ..Default::default()
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

    #[test]
    fn test_add_to_front() {
        let changes = diff(
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "1".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "2".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "3".to_string()
                            }
                        )
                    ],
                    ..Default::default()
                }
            ),
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "0".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "1".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "2".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "3".to_string()
                            }
                        )
                    ],
                    ..Default::default()
                }
            )
        );

        assert_eq!(changes.len(), 1);

        assert_eq!(changes[0], ClientAction::AddFront(
            AddFront {
                path: vec![],
                item: Item::Text(
                    Text {
                        text: "0".to_string()
                    }
                )
            }
        ));
    }

    #[test]
    fn test_add_to_middle() {
        let changes = diff(
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "1".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "2".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "3".to_string()
                            }
                        )
                    ],
                    ..Default::default()
                }
            ),
            &Item::View(
                View {
                    body: vec![
                        Item::Text(
                            Text {
                                text: "1".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "0".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "2".to_string()
                            }
                        ),
                        Item::Text(
                            Text {
                                text: "3".to_string()
                            }
                        )
                    ],
                    ..Default::default()
                }
            )
        );

        assert_eq!(changes.len(), 1);

        assert_eq!(changes[0], ClientAction::InsertAt(
            InsertAt {
                path: vec![],
                inx: 0,
                item: Item::Text(
                    Text {
                        text: "0".to_string()
                    }
                )
            }
        ));
    }
}