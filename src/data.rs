use serde::{Deserialize, Serialize};

/// Represents a collaborative document
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub content: String, // Could be extended to support rich data types like JSON
}

/// Represents an operation in the document
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Operation {
    Insert {
        index: usize,
        value: char,
        id: String,
    },
    Delete {
        index: usize,
        id: String,
    },
}

impl Operation {
    pub fn id(&self) -> &String {
        match self {
            Operation::Insert { id, .. } => id,
            Operation::Delete { id, .. } => id,
        }
    }
}

/// Represents a user action
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAction {
    pub user_id: String,
    pub operation: Operation,
}
