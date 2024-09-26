use collabori::crdt::RGA;
use collabori::data::Operation;
use collabori::ot::OT;

#[tokio::test]
async fn test_real_time_collaboration() {
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
}
