//! Generated Bitcoin RPC client library.
//!
//! This library provides a strongly-typed interface to the Bitcoin RPC API.
//! It is generated from the Bitcoin Core RPC API documentation.

// Core modules
pub mod config;
pub mod client_trait;
pub mod node;
pub mod test_node;
pub mod transport;
pub mod types;

// Re-exports for ergonomic access
pub use config::Config;
pub use client_trait::client::BitcoinClientV29_1;
pub use node::BitcoinNodeManager;
pub use bitcoin::Network;
pub use node::test_config::TestConfig;
pub use test_node::client::BitcoinTestClient;
pub use types::*;
pub use transport::{
    DefaultTransport,
    TransportError,
    RpcClient,
    BatchBuilder,
};

