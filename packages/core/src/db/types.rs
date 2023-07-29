
pub enum Value {
    String(String),
    Int(i32)
}

pub struct Field {
    key: String,
    value: Value
}

pub struct Join {
    pub table: String
}

pub struct WhereArgs {
    fields: Vec<Field>,
    joins: Vec<Join>
}

pub enum OrderDirection {
    Asc,
    Desc
}

pub struct OrderField {
    pub field: String,
    pub direc: OrderDirection
}

pub struct Payload {

}

pub struct FindArgs {
    wher: Option<WhereArgs>,
    order: Vec<OrderField>
}

pub struct CreateArgs {
    data: Payload
}

pub struct UpdateArgs {
    data: Payload,
    wher: WhereArgs
}

pub struct DeleteArgs {
    wher: WhereArgs
}