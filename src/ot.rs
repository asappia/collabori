use crate::data::Operation;
use serde::{Deserialize, Serialize};

/// Operational Transformation (OT) implementation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OT;

impl OT {
    /// Transforms operation a against operation b
    pub fn transform(op_a: &Operation, op_b: &Operation) -> Operation {
        match (op_a, op_b) {
            (Operation::Insert { index: idx_a, .. }, Operation::Insert { index: idx_b, .. }) => {
                if idx_a < idx_b {
                    op_a.clone()
                } else if idx_a > idx_b {
                    match op_a {
                        Operation::Insert {
                            index: _,
                            value,
                            id,
                        } => Operation::Insert {
                            index: idx_a + 1,
                            value: *value,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    }
                } else {
                    // If both operations insert at the same index, we need to decide the order
                    // Here, we can use the id to decide the order deterministically
                    if op_a.id() < op_b.id() {
                        op_a.clone()
                    } else {
                        match op_a {
                            Operation::Insert {
                                index: _,
                                value,
                                id,
                            } => Operation::Insert {
                                index: idx_a + 1,
                                value: *value,
                                id: id.clone(),
                            },
                            _ => op_a.clone(),
                        }
                    }
                }
            }
            (Operation::Insert { index: idx_a, .. }, Operation::Delete { index: idx_b, .. }) => {
                if idx_a <= idx_b {
                    op_a.clone()
                } else {
                    match op_a {
                        Operation::Insert {
                            index: _,
                            value,
                            id,
                        } => Operation::Insert {
                            index: idx_a - 1,
                            value: *value,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    }
                }
            }
            (Operation::Delete { index: idx_a, .. }, Operation::Insert { index: idx_b, .. }) => {
                if idx_a < idx_b {
                    op_a.clone()
                } else {
                    match op_a {
                        Operation::Delete { index: _, id } => Operation::Delete {
                            index: idx_a + 1,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    }
                }
            }
            (Operation::Delete { index: idx_a, .. }, Operation::Delete { index: idx_b, .. }) => {
                if idx_a == idx_b {
                    // Both deletes target the same index
                    // Handle idempotency by keeping one delete
                    // Here, we return op_a
                    op_a.clone()
                } else if idx_a < idx_b {
                    op_a.clone()
                } else {
                    match op_a {
                        Operation::Delete { index: _, id } => Operation::Delete {
                            index: idx_a - 1,
                            id: id.clone(),
                        },
                        _ => op_a.clone(),
                    }
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
