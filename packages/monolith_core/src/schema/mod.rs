
pub struct Field {
    pub name: String
}

pub struct Relation {

}

pub struct Model {
    pub fields: Vec<ModelField>,
    pub relations: Vec<Relation>
}