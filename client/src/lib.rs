//! A simple client for the [Rust Programming Language](https://www.rust-lang.org/) website.

#![warn(missing_docs)]

pub mod client;
pub mod data;
pub mod message;

pub use client::IrcClient;
