# Collabori

Collabori is a Rust library that provides a real-time collaborative editing engine using Conflict-free Replicated Data Types (CRDTs) and Operational Transformation (OT) algorithms. It is designed to facilitate seamless collaboration across multiple platforms, including web, desktop, and mobile applications.

## Features

- **CRDT Support**: Implements the Replicated Growable Array (RGA) CRDT for collaborative text editing.
- **Operational Transformation**: Handles insertion and deletion operations with proper transformation logic.
- **Synchronization Mechanism**: Real-time synchronization between clients and server using WebSockets.
- **Multi-Platform Integration**: Platform-agnostic design suitable for integration with various applications.
- **Robust Error Handling**: Utilizes Rust’s `Result` and `Error` types for graceful error management.
- **Extensible Traits**: Pluggable algorithms allowing customization and extension.

## Installation

Add `collabori` to your `Cargo.toml`:

```toml
[dependencies]
collabori = "0.1.0"
```


## Usage

### Basic Integration

To get started, you need to implement the `CRDT` and `OperationalTransform` traits for your specific use case. Here’s a simple example using the Replicated Growable Array (RGA) CRDT and a basic OT algorithm:

```rust
use collabori::crdt::RGA;
use collabori::data::Operation;
fn main() {
let mut rga = RGA::new();
let op = rga.insert(0, 'H');
println!("{:?}", rga);
}
```


### Real-Time Synchronization

Refer to the [Sync Module](./src/sync.rs) for setting up WebSocket servers and clients.

## Contributing

Contributions are welcome! Please open issues and submit pull requests for improvements and new features.

## License

This project is licensed under the MIT License.

## Repository

[GitHub Repository](https://github.com/yourusername/collabori)

