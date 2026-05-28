//! # motion-common
//!
//! Common types, errors, and utilities for MotionDB.
//!
//! This crate provides the foundational types and abstractions used across
//! all MotionDB components. It includes:
//!
//! - **Types**: Core identifiers (`PageId`, `TxnId`, `LSN`), keys, values, and timestamps
//! - **Errors**: Unified error handling with `MotionError`
//! - **Config**: Database configuration structures
//! - **Constants**: System-wide constants and limits
//! - **Memory**: Aligned allocation, arena allocator, NUMA-aware allocation
//!
//! ## Example
//!
//! ```rust
//! use motion_common::types::{PageId, TxnId, Key, Value};
//! use motion_common::error::MotionResult;
//!
//! fn example() -> MotionResult<()> {
//!     let page_id = PageId::new(42);
//!     let txn_id = TxnId::new(1);
//!     let key = Key::from_bytes(b"hello");
//!     let value = Value::from_bytes(b"world");
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod config;
pub mod constants;
pub mod error;
pub mod memory;
pub mod types;

// Re-export commonly used items at the crate root
pub use constants::*;
pub use error::{MotionError, MotionResult};
pub use types::{Key, Lsn, NodeId, PageId, Timestamp, TxnId, Value};
