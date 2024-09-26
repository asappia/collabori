use collabori::crdt::RGA;
use collabori::data::Operation;
use collabori::ot::OT;
use collabori::sync::SyncManager; // Ensure SyncManager is accessible
use tokio::time::{sleep, timeout, Duration};
use tokio_tungstenite::connect_async;
use url::Url;

#[tokio::test]
async fn test_real_time_collaboration() {
    // Define the address for the WebSocket server
    let addr = "127.0.0.1:9001";

    // Initialize and start the WebSocket server
    let sync_manager = SyncManager::new();
    let mut shutdown_rx = sync_manager.start_server(addr).await;

    // Give the server a moment to start
    sleep(Duration::from_millis(100)).await;

    // Initialize CRDT instances for two clients
    let mut client1 = RGA::new();
    let mut client2 = RGA::new();

    // Client 1 inserts 'H' at position 0
    let op1 = client1.insert(0, 'H');

    // Client 2 inserts 'W' at position 0 concurrently
    let op2 = client2.insert(0, 'W');

    // Transform operations
    let transformed_op1 = OT::transform(&op1, &op2); // Transform op1 against op2
    let transformed_op2 = OT::transform(&op2, &op1); // Transform op2 against op1

    // Apply transformed_op2 to client1 (Client 1 receives Client 2's operation)
    match transformed_op2 {
        Operation::Insert { index, value, .. } => {
            client1.insert(index, value);
        }
        Operation::Delete { .. } => {
            panic!("Expected Insert operation for transformed_op2");
        }
    }

    // Apply transformed_op1 to client2 (Client 2 receives Client 1's operation)
    match transformed_op1 {
        Operation::Insert { index, value, .. } => {
            client2.insert(index, value);
        }
        Operation::Delete { .. } => {
            panic!("Expected Insert operation for transformed_op1");
        }
    }

    // Extract visible characters from both clients
    let client1_content: String = client1
        .elements
        .iter()
        .filter(|e| e.visible)
        .map(|e| e.value)
        .collect();
    let client2_content: String = client2
        .elements
        .iter()
        .filter(|e| e.visible)
        .map(|e| e.value)
        .collect();

    // Ensure both clients have the same visible content
    assert_eq!(
        client1_content, client2_content,
        "Client documents do not match"
    );

    // Optionally, print the document contents for debugging
    println!("Client1 Document: {}", client1_content);
    println!("Client2 Document: {}", client2_content);

    // Connect a client to the server
    let url = Url::parse(&format!("ws://{}", addr)).unwrap();
    let (_ws_stream, _) = connect_async(url.as_str())
        .await
        .expect("Failed to connect");
    // Send the operations to the server
    //sync_manager.send_operation(ws_stream, op1).await;
    //sync_manager.send_operation(ws_stream, op2).await;

    // Wait for the server to process the operations
    sleep(Duration::from_millis(100)).await;

    // Extract visible characters from both clients
    let client1_content: String = client1
        .elements
        .iter()
        .filter(|e| e.visible)
        .map(|e| e.value)
        .collect();
    let client2_content: String = client2
        .elements
        .iter()
        .filter(|e| e.visible)
        .map(|e| e.value)
        .collect();

    // Ensure both clients have the same visible content
    assert_eq!(
        client1_content, client2_content,
        "Client documents do not match"
    );

    // Optionally, print the document contents for debugging
    println!("Client1 Document: {}", client1_content);
    println!("Client2 Document: {}", client2_content);

    // Shutdown the server after the test
    sync_manager.shutdown().await;

    // Wait for the server to shut down
    // Implement a timeout to prevent hanging indefinitely
    // Stop the server
    sync_manager.shutdown().await;

    // Wait for the server to shut down
    timeout(Duration::from_secs(5), shutdown_rx.recv())
        .await
        .expect("Server didn't shut down in time");
}
