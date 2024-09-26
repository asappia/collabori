use crate::data::Operation;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::accept_async;

#[derive(Debug)]
pub struct SyncManager {
    pub broadcaster: broadcast::Sender<Operation>,
    shutdown: broadcast::Sender<()>,
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncManager {
    /// Initializes the synchronization manager
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        let (shutdown_tx, _) = broadcast::channel(1);
        SyncManager {
            broadcaster: tx,
            shutdown: shutdown_tx,
        }
    }

    /// Starts the WebSocket server
    pub async fn start_server(&self, addr: &str) -> mpsc::Receiver<()> {
        let listener = TcpListener::bind(addr).await.expect("Failed to bind");
        println!("WebSocket server listening on {}", addr);

        let (shutdown_confirmation_tx, shutdown_confirmation_rx) = mpsc::channel(1);
        let mut shutdown_rx = self.shutdown.subscribe();
        let broadcaster = self.broadcaster.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok((stream, _)) = listener.accept() => {
                        let ws_stream = accept_async(stream).await.expect("Failed to accept");
                        let broadcaster = broadcaster.clone();
                        tokio::spawn(handle_connection(ws_stream, broadcaster));
                    }
                    _ = shutdown_rx.recv() => {
                        println!("Shutting down server");
                        break;
                    }
                }
            }
            // Notify that the server has shut down
            let _ = shutdown_confirmation_tx.send(()).await;
        });

        shutdown_confirmation_rx
    }

    /// Sends a shutdown signal to the server
    pub async fn shutdown(&self) {
        if let Err(err) = self.shutdown.send(()) {
            println!("Failed to send shutdown signal: {}", err);
        }
    }
}

async fn handle_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    broadcaster: broadcast::Sender<Operation>,
) {
    let (mut write, mut read) = ws_stream.split();
    let mut rx = broadcaster.subscribe();

    // Spawn a task to forward broadcast messages to the client
    let forward = tokio::spawn(async move {
        while let Ok(op) = rx.recv().await {
            let msg = serde_json::to_string(&op).unwrap();
            if write
                .send(tokio_tungstenite::tungstenite::Message::Text(msg))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Read messages from the client and broadcast them
    while let Some(msg) = read.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                if let Ok(op) = serde_json::from_str::<Operation>(&text) {
                    let _ = broadcaster.send(op);
                }
            }
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => break,
            _ => (),
        }
    }

    forward.abort();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
    use url::Url;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_sync_manager() {
        let sync_manager = SyncManager::new();
        let addr = "127.0.0.1:9001";

        // Start the server in a background task
        let mut shutdown_rx = sync_manager.start_server(addr).await;

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Connect a client to the server
        let url = Url::parse(&format!("ws://{}", addr)).unwrap();
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        // Send a message from the client
        let op = Operation::Insert {
            index: 0,
            value: 'a',
            id: "1".into(),
        };
        let msg = serde_json::to_string(&op).unwrap();
        ws_stream.send(Message::Text(msg)).await.unwrap();

        // Receive the broadcasted message
        let result = timeout(Duration::from_secs(1), ws_stream.next()).await;
        match result {
            Ok(Some(Ok(Message::Text(received_msg)))) => {
                let received_op: Operation = serde_json::from_str(&received_msg).unwrap();
                assert_eq!(received_op, op);
            }
            _ => panic!("Did not receive the expected message"),
        }

        // Stop the server
        sync_manager.shutdown().await;

        // Wait for the server to shut down
        timeout(Duration::from_secs(5), shutdown_rx.recv()).await
            .expect("Server didn't shut down in time");
    }
}
