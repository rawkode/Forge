//! Forge Core Types and Utilities
//!
//! This crate contains the core domain types and shared utilities used across
//! the Forge application. It defines the fundamental data structures for
//! repositories, products, issues, reviews, releases, and other entities.

pub mod types;
pub mod error;
pub mod config;

pub use error::{Error, Result};
pub use types::*;