use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawPair {
    pub data_id: String,
    pub structures: Vec<StructurePair>,
    pub custom_structures: Vec<CustomStructurePair>,
    pub published: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructurePair {
    pub id: String,
    pub value: String,
    pub rtype: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomStructurePair {
    pub id: String,
    pub structures: Vec<StructurePair>,
}
