#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Entry {
    Contains {
        hash: u64,
        depth: u8,
        score: i32,
        age: u8
    },
    Empty
}

impl Entry {
    pub fn is_better(&self, other_depth: u8) -> bool {
        match self {
            Entry::Contains { depth, .. } => { &other_depth >= depth }
            Entry::Empty => true,
        }
    }

    pub fn is_some_and_better(&self, other_depth: u8) -> bool {
        match self {
            Entry::Contains { depth, .. } => { &other_depth >= depth },
            Entry::Empty => false,
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            Entry::Contains { score, .. } => *score,
            Entry::Empty => panic!("This function is not implemented on empty entries!"),
        }
    }
}