//! Methods for encrypting and decrypting messages using substitution cipher.

use crate::check_message_from_letters;

/// Encrypts `message` by substitution according to `permutation`.
/// Every lett