use crate::data::Operation;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Operational Transformation (OT) implementation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OT;

impl OT {
    /// Transforms operation a against operation b
    pub fn transform(op_a: &Operation, op_b: &Operation) -> Operation {
        match (op_a, op_b) {
            (Operation::Insert { index: idx_a, .. }, Operation::Insert { index: idx_b, .. }) => {
                match idx_a.cmp(idx_b) {
                    Ordering::Less => op_a.clone(),
                    Ordering::Greater => match op_a {
                        Operation::Insert { value, id, .. } => Operation::Insert {
                            index: idx_a + 1,
                            value: *value,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    },
                    Ordering::Equal => match op_a.id().cmp(&op_b.id()) {
                        Ordering::Less => op_a.clone(),
                        Ordering::Greater => match op_a {
                            Operation::Insert { value, id, .. } => Operation::Insert {
                                index: idx_a + 1,
                                value: *value,
                                id: id.clone(),
                            },
                            _ => op_a.clone(),
                        },
                        Ordering::Equal => op_a.clone(),
                    },
                }
            }
            (Operation::Insert { index: idx_a, .. }, Operation::Delete { index: idx_b, .. }) => {
                match idx_a.cmp(idx_b) {
                    Ordering::Less | Ordering::Equal => op_a.clone(),
                    Ordering::Greater => match op_a {
                        Operation::Insert { value, id, .. } => Operation::Insert {
                            index: idx_a - 1,
                            value: *value,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    },
                }
            }
            (Operation::Delete { index: idx_a, .. }, Operation::Insert { index: idx_b, .. }) => {
                match idx_a.cmp(idx_b) {
                    Ordering::Less => op_a.clone(),
                    Ordering::Greater => match op_a {
                        Operation::Delete { id, .. } => Operation::Delete {
                            index: idx_a + 1,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    },
                    Ordering::Equal => op_a.clone(),
                }
            }
            (Operation::Delete { index: idx_a, .. }, Operation::Delete { index: idx_b, .. }) => {
                match idx_a.cmp(idx_b) {
                    Ordering::Equal => op_a.clone(),
                    Ordering::Less => op_a.clone(),
                    Ordering::Greater => match op_a {
                        Operation::Delete { id, .. } => Operation::Delete {
                            index: idx_a - 1,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Operation;

    #[test]
    fn test_transform_insert_insert() {
        let op_a = Operation::Insert {
            index: 1,
            value: 'a',
            id: "1".into(),
        };
        let op_b = Operation::Insert {
            index: 2,
            value: 'b',
            id: "2".into(),
        };
        let transformed = OT::transform(&op_a, &op_b);
        assert_eq!(transformed, op_a);
    }

    #[test]
    fn test_transform_insert_delete() {
        let op_a = Operation::Insert {
            index: 3,
            value: 'a',
            id: "1".into(),
        };
        let op_b = Operation::Delete {
            index: 2,
            id: "2".into(),
        };
        let transformed = OT::transform(&op_a, &op_b);
        if let Operation::Insert { index, .. } = transformed {
            assert_eq!(index, 2);
        } else {
            panic!("Expected Insert operation");
        }
    }

    #[test]
    fn test_transform_delete_delete() {
        let op_a = Operation::Delete {
            index: 2,
            id: "1".into(),
        };
        let op_b = Operation::Delete {
            index: 2,
            id: "2".into(),
        };
        let transformed = OT::transform(&op_a, &op_b);
        assert_eq!(transformed, op_a);
    }
}
