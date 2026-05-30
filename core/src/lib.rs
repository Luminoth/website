//! Shared library for the EnergonSoftware website backend.
//!
//! Provides AWS client helpers, primarily DynamoDB wrappers with
//! pagination and callback-driven item processing.

#![deny(warnings)]
#![deny(missing_docs)]

/// AWS client utilities.
pub mod aws;
