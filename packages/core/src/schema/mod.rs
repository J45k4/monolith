
pub struct Decorator {
    pub name: String,
    pub args: Vec<DecoratorArg>,
}

pub struct Field {
    pub name: String,
    pub nullable: bool,
    pub opt: bool,
    pub decorators: Vec<Decorator>,
}

pub struct Relation {
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
    pub target: String,
}

pub struct Model {
    pub name: String,
    pub fields: Vec<ModelField>,
    pub relations: Vec<Relation>,
    pub decorators: Vec<Decorator>,
}