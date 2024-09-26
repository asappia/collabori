pub mod crdt;
pub mod data;
pub mod ot;
pub mod sync;
pub mod utils;
pub mod client;

use crate::data::Operation;
use crate::sync::SyncManager;
use crate::client::SyncClient;
/// Trait for CRDT algorithms
pub trait CRDT {
    fn insert(&mut self, index: usize, value: char) -> Operation;
    fn delete(&mut self, index: usize) -> Operation;
    fn merge(&mut self, other: Self);
}

/// Trait for OT algorithms
pub trait OperationalTransform {
    fn transform(&self, op_a: &Operation, op_b: &Operation) -> Operation;
}

/// Initializes and starts the SyncManager WebSocket server
pub async fn start_sync_server(addr: &str) {
    let sync_manager = SyncManager::new();
    sync_manager.start_server(addr).await;
}

/// Connects to a SyncManager WebSocket server as a client
pub async fn connect_sync_client(addr: &str) -> SyncClient {
    SyncClient::connect(addr).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crdt::RGA;
    use crate::ot::OT;

    #[test]
    fn test_crdt_insert() {
        let mut rga = RGA::new();
        let op = rga.insert(0, 'a');
        assert_eq!(op, Operation::Insert { index: 0, value: 'a', id: op.id().clone() });
        assert_eq!(rga.elements.len(), 1);
        assert_eq!(rga.elements[0].value, 'a');
        assert!(rga.elements[0].visible);
    }

    #[test]
    fn test_crdt_delete() {
        let mut rga = RGA::new();
        rga.insert(0, 'a');
        let op = rga.delete(0);
        assert_eq!(op, Operation::Delete { index: 0, id: op.id().clone() });
        assert_eq!(rga.elements[0].visible, false);
    }

    #[test]
    fn test_crdt_merge() {
        let mut rga1 = RGA::new();
        rga1.insert(0, 'a');

        let mut rga2 = RGA::new();
        rga2.insert(0, 'b');

        rga1.merge(rga2);

        assert_eq!(rga1.elements.len(), 2);
        assert!(rga1.elements.iter().any(|e| e.value == 'a' && e.visible));
        assert!(rga1.elements.iter().any(|e| e.value == 'b' && e.visible));
    }

    #[test]
    fn test_ot_transform_insert_insert() {
        let op_a = Operation::Insert { index: 1, value: 'a', id: "1".into() };
        let op_b = Operation::Insert { index: 2, value: 'b', id: "2".into() };
        let result = OT::transform(&op_a, &op_b);
        assert_eq!(result, op_a);
    }

    #[test]
    fn test_ot_transform_insert_delete() {
        let op_a = Operation::Insert { index: 1, value: 'a', id: "1".into() };
        let op_b = Operation::Delete { index: 2, id: "2".into() };
        let result = OT::transform(&op_a, &op_b);
        assert_eq!(result, op_a);
    }

    #[test]
    fn test_ot_transform_delete_insert() {
        let op_a = Operation::Delete { index: 1, id: "1".into() };
        let op_b = Operation::Insert { index: 2, value: 'b', id: "2".into() };
        let result = OT::transform(&op_a, &op_b);
        assert_eq!(result, op_a);
    }

    #[test]
    fn test_ot_transform_delete_delete() {
        let op_a = Operation::Delete { index: 1, id: "1".into() };
        let op_b = Operation::Delete { index: 2, id: "2".into() };
        let result = OT::transform(&op_a, &op_b);
        assert_eq!(result, op_a);
    }
}
