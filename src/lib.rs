
//! Collection of many various encrypting functions implementations
//! in pure Rust. Every contained encryption method has its own module
//! with functions for encrypting and decrypting appropriate type of message.

pub mod ceasar;
pub mod substitution;

fn check_message_from_letters(message: &[u8]) {