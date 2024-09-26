use crate::data::Operation;
use crate::utils::generate_unique_id;
use serde::{Deserialize, Serialize};

/// Replicated Growable Array (RGA) CRDT implementation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RGA {
    pub elements: Vec<Element>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    pub id: String,
    pub value: char,
    pub visible: bool,
}

impl Default for RGA {
    fn default() -> Self {
        Self::new()
    }
}

impl RGA {
    /// Creates a new RGA instance
    pub fn new() -> Self {
        RGA {
            elements: Vec::new(),
        }
    }

    /// Inserts a character at a specified position
    pub fn insert(&mut self, index: usize, value: char) -> Operation {
        let id = generate_unique_id();
        let element = Element {
            id: id.clone(),
            value,
            visible: true,
        };
        self.elements.insert(index, element.clone());
        Operation::Insert { id, value, index }
    }

    /// Deletes a character at a specified position
    pub fn delete(&mut self, index: usize) -> Operation {
        if let Some(element) = self.elements.get_mut(index) {
            element.visible = false;
            return Operation::Delete {
                id: element.id.clone(),
                index,
            };
        }
        panic!("Index out of bounds");
    }

    /// Merges another RGA state into this one
    pub fn merge(&mut self, other: RGA) {
        for elem in other.elements {
            if !self.elements.iter().any(|e| e.id == elem.id) {
                self.elements.push(elem);
            }
        }
        self.elements.sort_by_key(|e| e.id.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut rga = RGA::new();
        let _op = rga.insert(0, 'a');
        assert_eq!(rga.elements.len(), 1);
        assert_eq!(rga.elements[0].value, 'a');
        assert!(rga.elements[0].visible);
    }

    #[test]
    fn test_delete() {
        let mut rga = RGA::new();
        rga.insert(0, 'a');
        let _op = rga.delete(0);
        assert!(!rga.elements[0].visible);
    }

    #[test]
    fn test_merge() {
        let mut rga1 = RGA::new();
        rga1.insert(0, 'a');

        let mut rga2 = RGA::new();
        rga2.insert(0, 'b');

        rga1.merge(rga2.clone());
        assert_eq!(rga1.elements.len(), 2);
    }
}
