//! CSC API v2.2 client for Qualified Electronic Signatures (QES).
//!
//! This crate provides a typed HTTP client for the CSC API (ETSI TS 119 432),
//! enabling wallet applications to request qualified electronic signatures from
//! remote QTSPs (Qualified Trust Service Providers).
//!
//! # Architecture
//!
//! The CSC client is a **pure HTTP client** — it does NOT:
//! - Own OAuth flows (receives access tokens from the caller)
//! - Manage keys (signing keys live in the QTSP's QSCD)
//! - Render documents (returns metadata; caller decides UX)
//!
//! The only cryptographic operation it performs is DPoP proof generation
//! via an injected [`DPopSigner`] trait, which can be backed by any WSCD.
//!
//! # Minimum viable scope
//!
//! - `credentials/list` — enumerate available signing credentials
//! - `credentials/info` — get metadata for a specific credential
//! - `signatures/signHash` — request hash signing

pub mod client;
pub mod error;
pub mod ffi;
pub mod types;

uniffi::setup_scaffolding!();

pub use client::CscClient;
pub use error::{CscError, Result};
pub use types::*;
