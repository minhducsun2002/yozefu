//! The core of the tool:
//! - List topics, and consume records,
//! - Fetch information about a given topic,
//! - Consume records.
mod app;
mod config;
pub mod search;

pub use app::App;
pub use config::ClusterConfig;
pub use config::Config;
pub use config::SchemaRegistryConfig;

/// Name of the application
pub const APPLICATION_NAME: &str = "yozefu";
