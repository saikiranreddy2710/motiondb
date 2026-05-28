//! Error handling for MotionDB.
//!
//! This module provides a unified error type and result alias used
//! across all MotionDB components.

mod database;

pub use database::{ErrorCode, MotionError};

/// Result type alias for MotionDB operations.
pub type MotionResult<T> = std::result::Result<T, MotionError>;
