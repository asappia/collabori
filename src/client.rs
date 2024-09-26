use crate::data::Operation;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use url::Url;

#[derive(Debug)]
pub struct SyncClient {
    pub sender: mpsc::Sender<Operation>, // For sending operations to the server
    pub receiver: mpsc::Receiver<Operation>, // For receiving operations from the server
}

impl SyncClient {
    /// Connects to the synchronization server
    pub async fn connect(addr: &str) -> Self {
        // Parse the WebSocket URL
        let url = Url::parse(&format!("ws://{}", addr)).unwrap();

        // Establish the WebSocket connection
        let (ws_stream, _) = connect_async(url.as_str())
            .await
            .expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        // Create channels for sending and receiving operations
        let (send_tx, mut send_rx) = mpsc::channel::<Operation>(100); // Sender to send ops to server
        let (recv_tx, recv_rx) = mpsc::channel::<Operation>(100); // Receiver to receive ops from server

        // Spawn a task to handle sending operations to the server
        tokio::spawn(async move {
            while let Some(op) = send_rx.recv().await {
                let msg = serde_json::to_string(&op).unwrap();
                if write
                    .send(tokio_tungstenite::tungstenite::Message::Text(msg))
                    .await
                    .is_err()
                {
                    // If sending fails, exit the loop
                    println!("Failed to send message to server. Exiting send task.");
                    break;
                }
            }
            println!("Send task has been terminated.");
        });

        // Spawn a task to handle receiving operations from the server
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                        if let Ok(op) = serde_json::from_str::<Operation>(&text) {
                            println!("Received operation: {:?}", op);
                            let _ = recv_tx.send(op).await; // Send received op to the receiver channel
                        }
                    }
                    Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                        println!("Received close message from server.");
                        break;
                    }
                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                        break;
                    }
                    _ => {} // Ignore other message types
                }
            }
            println!("Receive task has been terminated.");
        });

        // Return the SyncClient instance with sender and receiver
        SyncClient {
            sender: send_tx,
            receiver: recv_rx,
        }
    }

    /// Sends an operation to the server
    pub async fn send_operation(&self, op: Operation) {
        self.sender
            .send(op)
            .await
            .expect("Failed to send operation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::SyncManager;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_sync_client() {
        let addr = "127.0.0.1:9002";

        // Initialize SyncManager
        let sync_manager = SyncManager::new();
        let mut shutdown_handle = sync_manager.start_server(addr).await;

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Connect the first client
        let client1 = SyncClient::connect(addr).await;

        // Send an operation from client1
        let op1 = Operation::Insert {
            index: 0,
            value: 'a',
            id: "1".into(),
        };
        client1.send_operation(op1.clone()).await;

        // Give the server time to process the operation
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Connect the second client
        let mut client2 = SyncClient::connect(addr).await; // Declare client2 as mutable

        // Send another operation from client1
        let op2 = Operation::Insert {
            index: 1,
            value: 'b',
            id: "2".into(),
        };
        client1.send_operation(op2.clone()).await;

        // Attempt to receive the operation on client2
        if let Some(received_op) = client2.receiver.recv().await {
            assert_eq!(
                received_op, op2,
                "client2 did not receive the expected operation"
            );
            println!("Test passed: client2 received the expected operation.");
        } else {
            panic!("Did not receive the expected operation on client2");
        }

        // Shutdown the server
        sync_manager.shutdown().await;

        // Wait for the server to confirm shutdown
        let shutdown_confirmation =
            tokio::time::timeout(Duration::from_secs(5), shutdown_handle.recv()).await;
        assert!(
            shutdown_confirmation.is_ok(),
            "Server did not shut down in time"
        );
        println!("Test completed successfully.");
    }
}
