mod api;
mod client;
mod error;
pub mod models;
pub(crate) mod serde_util;

pub use client::IfpaClient;
pub use error::IfpaError;
