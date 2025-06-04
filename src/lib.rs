//! Generated Bitcoin RPC client library.
//!
//! This library provides a strongly-typed interface to the Bitcoin RPC API.
//! It is generated from the Bitcoin Core RPC API documentation.

pub mod config;
pub mod node;
pub mod transport;
pub mod types;
pub mod test_node;

pub use config::Config;
pub use node::BitcoinNodeManager;
pub use transport::{DefaultTransport, TransportError};
pub use crate::test_node::test_node::BitcoinTestClient;
