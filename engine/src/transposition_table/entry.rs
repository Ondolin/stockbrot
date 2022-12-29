use crate::search::NodeType;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Entry {
    Contains {
        hash: u64,
        depth: u8,
        score: i32,
        node_type: NodeType,
        age: u8
    },
    Empty
}