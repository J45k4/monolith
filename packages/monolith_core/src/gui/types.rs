use std::collections::HashMap;

use super::gui::Item;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct OnClick {
    pub id: String,
    pub name: String
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnTextChanged {
    pub id: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum MsgFromClient {
    OnClick(OnClick),
    OnTextChanged(OnTextChanged),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnKeyDown {
    pub id: String,
    pub keycode: String
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParametersChanged {
    pub query: HashMap<String, String>,
    pub params: Vec<String>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ClientEvent { 
    Disconnected,
    ParametersChanged(ParametersChanged),
    OnClick(OnClick),
    OnTextChanged(OnTextChanged),
    OnKeyDown(OnKeyDown)
}

pub type ItemPath = Vec<usize>;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Replace {
    pub path: ItemPath,
    pub item: Item
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplaceAt {
    pub path: ItemPath,
    pub item: Item,
    pub inx: usize
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddBack {
    pub path: ItemPath,
    pub item: Item
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddFront {
    pub path: ItemPath,
    pub item: Item
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct InsertAt {
    pub path: ItemPath,
    pub item: Item,
    pub inx: usize
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveInx {
    pub path: ItemPath,
    pub inx: usize
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ClientAction {
    Replace(Replace),
    ReplaceAt(ReplaceAt),
    AddBack(AddBack),
    AddFront(AddFront),
    InsertAt(InsertAt),
    RemoveInx(RemoveInx),
}