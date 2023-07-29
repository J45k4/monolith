use crate::{gui::{types::{ClientAction, AddBack, RemoveInx, Replace}, edit_distance::{get_minimum_edits, EditOperation}}, AddFront, InsertAt, ReplaceAt};

use super::{gui::Item, types::ItemPath};



fn inner_diff(changes: &mut Vec<ClientAction>, old: &Item, new: &Item, path: ItemPath) {
    log::trace!("{:?} inner_dif", path);

    match (old, new) {
        (Item::View(old), Item::View(new)) => {
            log::trace!("{:?} inner_diff calculating view minumum edits", path);

            let edits = get_minimum_edits(&old.body, &new.body);

            for edit in edits {
                match edit {
                    EditOperation::InsertFirst(item) => {
                        log::trace!("{:?} insert first", path);

                        changes.push(
                            ClientAction::AddFront(
                                AddFront {
                                    path: path.clone(),
                                    item: item
                                }
                            )
                        );
                    },
                    EditOperation::InsertAfter(index, item) => {
                        log::trace!("{:?} insert after {}", path, index);

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
                        log::trace!("{:?} remove at index {}", path, index);

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
                        log::trace!("{:?} replace at {}", path, i);

                        let mut path = path.clone();
                        path.push(i);

                        log::trace!("{:?} new path: {:?}", path, path);
    
                        inner_diff(changes, &old.body[i], &item, path);
                    },
                    EditOperation::InsertBack(item) => {
                        log::trace!("{:?} insert back", path);

                        todo!();
                    }
                }
            }
        }
        _ => {
            log::trace!("{:?} comparing old and new", path);

            if old != new {
                log::trace!("{:?} old and new are different", path);

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
    log::trace!("diff");
    log::trace!("{:?}", old);
    log::trace!("{:?}", new);

    let mut changes = Vec::new();
    let mut path = Vec::new();

    inner_diff(&mut changes, old, new, path);

    log::debug!("diff changes: {:?}", changes);

    changes
}

#[cfg(test)]
mod tests {
    // use crate::{gui::{gui::{Item, Text, View, Button, TextInput, Checkbox}, types::{ClientAction, Replace, RemoveInx, AddBack}}, AddFront, InsertAt, ReplaceAt, test_util::enable_trace};

    // use super::diff;

    // #[test]
    // fn it_works() {
    //     let changes = diff(
    //         &Item::Text(
    //             Text {
    //                 text: "Hello".to_string(),
    //             }
    //         ),
    //         &Item::Text(
    //             Text {
    //                 text: "Hello World".to_string(),
    //             }
    //         )
    //     );

    //     assert_eq!(changes.len(), 1);
    //     assert_eq!(
    //         changes[0],
    //         ClientAction::Replace(
    //             Replace {
    //                 path: vec![],
    //                 item: Item::Text(
    //                     Text {
    //                         text: "Hello World".to_string(),
    //                     }
    //                 )
    //             }
    //         )
    //     );
    // }

    // #[test]
    // fn test_add_to_back() {
    //     let changes = diff(
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "Hello".to_string(),
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         ),
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "Hello".to_string(),
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "World".to_string(),
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         )
    //     );

    //     assert_eq!(changes.len(), 1);
    //     assert_eq!(changes[0], ClientAction::InsertAt(
    //         InsertAt {
    //             path: vec![],
    //             inx: 0,
    //             item: Item::Text(
    //                 Text {
    //                     text: "World".to_string(),
    //                 }
    //             )
    //         }
    //     ));
    // }

    // #[test]
    // fn test_remove_child() {
    //     let changes = diff(
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "Hello".to_string(),
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "World".to_string(),
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         ),
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "Hello".to_string(),
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         )
    //     );

    //     assert_eq!(changes.len(), 1);
    //     assert_eq!(changes[0], ClientAction::RemoveInx(
    //         RemoveInx {
    //             path: vec![],
    //             inx: 1
    //         }
    //     ));
    // }

    // #[test]
    // fn test_button_changed_to_text() {
    //     let changes = diff(
    //         &Item::Button(
    //             Button {
    //                 id: None,
    //                 name: None,
    //                 title: "Hello".to_string()
    //             }
    //         ),
    //         &Item::Text(
    //             Text {
    //                 text: "Hello".to_string()
    //             }
    //         )
    //     );

    //     assert_eq!(changes.len(), 1);
    //     assert_eq!(changes[0], ClientAction::Replace(
    //         super::Replace {
    //             path: vec![],
    //             item: Item::Text(
    //                 Text {
    //                     text: "Hello".to_string()
    //                 }
    //             )
    //         }
    //     ));
    // }

    // #[test]
    // fn test_diffing_more_complicated() {
    //     let changes = diff(
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::View (
    //                         View {
    //                             body: vec![
    //                                 Item::TextInput(
    //                                     TextInput {
    //                                         id: "qwerty".to_string(),
    //                                         name: "qwerty".to_string(),
    //                                         value: "".to_string(),
    //                                         placeholder: "Hello".to_string()
    //                                     }
    //                                 ),
    //                                 Item::Button(
    //                                     Button {
    //                                         id: Some("qwerty".to_string()),
    //                                         name: Some("qwerty".to_string()),
    //                                         title: "Hello".to_string()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     ),
    //                     Item::View(
    //                         View {
    //                             body: vec![],
    //                             ..Default::default()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         ),
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::View (
    //                         View {
    //                             body: vec![
    //                                 Item::TextInput(
    //                                     TextInput {
    //                                         id: "qwerty".to_string(),
    //                                         name: "qwerty".to_string(),
    //                                         value: "newvalue".to_string(),
    //                                         placeholder: "Hello".to_string()
    //                                     }
    //                                 ),
    //                                 Item::Button(
    //                                     Button {
    //                                         id: Some("qwerty".to_string()),
    //                                         name: Some("qwerty".to_string()),
    //                                         title: "Hello".to_string()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     ),
    //                     Item::View(
    //                         View {
    //                             body: vec![
    //                                 Item::View(
    //                                     View { 
    //                                         body: vec![
    //                                             Item::Text(
    //                                                 Text {
    //                                                     text: "Newrow".to_string()
    //                                                 }
    //                                             )
    //                                         ],
    //                                         ..Default::default()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         )
    //     );

    //     println!("{:?}", changes);

    //     assert_eq!(changes.len(), 2);

    //     let change = &changes[0];

    //     assert_eq!(change, &ClientAction::Replace(
    //         super::Replace {
    //             path: vec![0, 0],
    //             item: Item::TextInput(
    //                 TextInput { 
    //                     id: "qwerty".to_string(), 
    //                     name: "qwerty".to_string(),
    //                     placeholder: "Hello".to_string(), 
    //                     value: "newvalue".to_string()
    //                 }
    //             )
    //         }
    //     ));

    //     let change = &changes[1];

    //     assert_eq!(change, &ClientAction::AddFront(
    //         AddFront {
    //             path: vec![1],
    //             item: Item::View(
    //                 View {
    //                     body: vec![
    //                         Item::Text(
    //                             Text {
    //                                 text: "Newrow".to_string()
    //                             }
    //                         )
    //                     ],
    //                     ..Default::default()
    //                 }
    //             )
    //         }
    //     ));
    // }

    // #[test]
    // fn test_diffing_more_complicated2() {
    //     let changes = diff(
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::View (
    //                         View {
    //                             body: vec![
    //                                 Item::TextInput(
    //                                     TextInput {
    //                                         id: "qwerty".to_string(),
    //                                         name: "qwerty".to_string(),
    //                                         value: "".to_string(),
    //                                         placeholder: "Hello".to_string()
    //                                     }
    //                                 ),
    //                                 Item::Button(
    //                                     Button {
    //                                         id: Some("qwerty".to_string()),
    //                                         name: Some("qwerty".to_string()),
    //                                         title: "Hello".to_string()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     ),
    //                     Item::View(
    //                         View {
    //                             body: vec![
    //                                 Item::View(
    //                                     View { 
    //                                         body: vec![
    //                                             Item::Checkbox(
    //                                                 Checkbox {
    //                                                     id: "qwerty".to_string(),
    //                                                     name: "qwerty".to_string(),
    //                                                     checked: false
    //                                                 }
    //                                             ),
    //                                             Item::Text(
    //                                                 Text {
    //                                                     text: "Makkara".to_string()
    //                                                 }
    //                                             )
    //                                         ],
    //                                         ..Default::default()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         ),
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::View (
    //                         View {
    //                             body: vec![
    //                                 Item::TextInput(
    //                                     TextInput {
    //                                         id: "qwerty".to_string(),
    //                                         name: "qwerty".to_string(),
    //                                         value: "newvalue".to_string(),
    //                                         placeholder: "Hello".to_string()
    //                                     }
    //                                 ),
    //                                 Item::Button(
    //                                     Button {
    //                                         id: Some("qwerty".to_string()),
    //                                         name: Some("qwerty".to_string()),
    //                                         title: "Hello".to_string()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     ),
    //                     Item::View(
    //                         View {
    //                             body: vec![
    //                                 Item::View(
    //                                     View { 
    //                                         body: vec![
    //                                             Item::Checkbox(
    //                                                 Checkbox {
    //                                                     id: "qwerty".to_string(),
    //                                                     name: "qwerty".to_string(),
    //                                                     checked: false
    //                                                 }
    //                                             ),
    //                                             Item::Text(
    //                                                 Text {
    //                                                     text: "Makkara".to_string()
    //                                                 }
    //                                             )
    //                                         ],
    //                                         ..Default::default()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         )
    //     );

    //     assert_eq!(changes.len(), 1);
    //     assert_eq!(changes[0], ClientAction::Replace(
    //         super::Replace {
    //             path: vec![0, 0],
    //             item: Item::TextInput(
    //                 TextInput { 
    //                     id: "qwerty".to_string(), 
    //                     name: "qwerty".to_string(),
    //                     placeholder: "Hello".to_string(), 
    //                     value: "newvalue".to_string()
    //                 }
    //             )
    //         }
    //     ));
    // }

    // #[test]
    // fn test_add_to_front() {
    //     let changes = diff(
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "1".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "2".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "3".to_string()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         ),
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "0".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "1".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "2".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "3".to_string()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         )
    //     );

    //     assert_eq!(changes.len(), 1);

    //     assert_eq!(changes[0], ClientAction::AddFront(
    //         AddFront {
    //             path: vec![],
    //             item: Item::Text(
    //                 Text {
    //                     text: "0".to_string()
    //                 }
    //             )
    //         }
    //     ));
    // }

    // #[test]
    // fn test_add_to_middle() {
    //     let changes = diff(
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "1".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "2".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "3".to_string()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         ),
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "1".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "0".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "2".to_string()
    //                         }
    //                     ),
    //                     Item::Text(
    //                         Text {
    //                             text: "3".to_string()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         )
    //     );

    //     assert_eq!(changes.len(), 1);

    //     assert_eq!(changes[0], ClientAction::InsertAt(
    //         InsertAt {
    //             path: vec![],
    //             inx: 0,
    //             item: Item::Text(
    //                 Text {
    //                     text: "0".to_string()
    //                 }
    //             )
    //         }
    //     ));
    // }

    // #[test]
    // fn test_bug_fixed() {
    //     //View(View { flex: None, height: None, width: None, body: [Text(Text { text: "Not found" })] }) -> View(View { flex: None, height: None, width: None, body: [View(View { flex: None, height: None, width: None, body: [TextInput(TextInput { id: "searchWord", name: "searchWord", placeholder: "searchword", value: "" }), Button(Button { id: None, name: Some("searchButton"), title: "Search" })] }), View(View { flex: None, height: Some(200), width: None, body: [] })] })

    //     enable_trace();

    //     let changes = diff(
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::Text(
    //                         Text {
    //                             text: "Not found".to_string()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         ),
    //         &Item::View(
    //             View {
    //                 body: vec![
    //                     Item::View(
    //                         View {
    //                             body: vec![
    //                                 Item::TextInput(
    //                                     TextInput {
    //                                         id: "searchWord".to_string(),
    //                                         name: "searchWord".to_string(),
    //                                         placeholder: "searchword".to_string(),
    //                                         value: "".to_string()
    //                                     }
    //                                 ),
    //                                 Item::Button(
    //                                     Button {
    //                                         id: None,
    //                                         name: Some("searchButton".to_string()),
    //                                         title: "Search".to_string()
    //                                     }
    //                                 )
    //                             ],
    //                             ..Default::default()
    //                         }
    //                     ),
    //                     Item::View(
    //                         View {
    //                             body: vec![],
    //                             height: Some(200),
    //                             ..Default::default()
    //                         }
    //                     )
    //                 ],
    //                 ..Default::default()
    //             }
    //         )
    //     );

    //     println!("change[0] {:?}", changes[0]);
    //     println!("change[1] {:?}", changes[1]);

    //     assert_eq!(changes.len(), 2);

    //     assert_eq!(changes[0], ClientAction::Replace(
    //         Replace {
    //             path: vec![0],
    //             item: Item::View(
    //                 View {
    //                     height: Some(200),
    //                     body: vec![],
    //                     ..Default::default()
    //                 }
    //             )
    //         }
    //     ));
        
    //     assert_eq!(changes[1], ClientAction::AddFront(
    //         AddFront {
    //             path: vec![],
    //             item: Item::View(
    //                 View {
    //                     body: vec![
    //                         Item::TextInput(
    //                             TextInput {
    //                                 id: "searchWord".to_string(),
    //                                 name: "searchWord".to_string(),
    //                                 placeholder: "searchword".to_string(),
    //                                 value: "".to_string()
    //                             }
    //                         ),
    //                         Item::Button(
    //                             Button {
    //                                 id: None,
    //                                 name: Some("searchButton".to_string()),
    //                                 title: "Search".to_string()
    //                             }
    //                         )
    //                     ],
    //                     ..Default::default()
    //                 }
    //             )
    //         }
    //     ));
    // }
}