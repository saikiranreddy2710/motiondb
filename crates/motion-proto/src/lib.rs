//! MotionDB gRPC Protocol Definitions
//!
//! This crate provides the gRPC service definitions and generated code
//! for client-server communication in MotionDB.
//!
//! # Overview
//!
//! The protocol defines:
//! - Query execution (single and streaming)
//! - Prepared statements
//! - Transaction management
//! - Connection management
//!
//! # Example
//!
//! ```ignore
//! use motion_proto::motion_db_client::MotionDbClient;
//! use motion_proto::ExecuteRequest;
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = MotionDbClient::connect("http://localhost:5432").await?;
//!     
//!     let request = ExecuteRequest {
//!         sql: "SELECT * FROM users".to_string(),
//!         ..Default::default()
//!     };
//!     
//!     let response = client.execute(request).await?;
//!     println!("Result: {:?}", response);
//!     Ok(())
//! }
//! ```

#![warn(clippy::all)]

/// Generated protobuf types and gRPC service definitions.
#[allow(missing_docs)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    tonic::include_proto!("motion");
}

// Re-export commonly used types
pub use proto::*;

// Re-export server and client types
pub use proto::motion_db_client::MotionDbClient;
pub use proto::motion_db_server::{MotionDb, MotionDbServer};
