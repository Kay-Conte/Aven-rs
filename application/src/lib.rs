//! Core module contains developer facing data models, considering new workspace structure as the current one is nonsensicly derived.

pub use async_trait::async_trait;

pub mod application;
pub mod connection;
pub mod error;
mod shard;
