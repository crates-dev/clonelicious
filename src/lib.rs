//! clonelicious
//!
//! A Rust macro library that simplifies cloning and closure execution.
//! The `clone!` macro automatically clones variables and immediately executes
//! the closure with the cloned values, streamlining common patterns in Rust programming.

pub(crate) mod cfg;
pub(crate) mod r#macro;
