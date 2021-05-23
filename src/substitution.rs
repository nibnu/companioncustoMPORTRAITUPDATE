//! Methods for encrypting and decrypting messages using substitution cipher.

use crate::check_message_from_letters;

/// Encrypts `message` by substitution according to `permutation`.
/// Every letter is substituted for letter at position (letter's alphabet index)
/// in `permutation` array.
///
/// # Panics
/// Panics if `permutation` is not a valid ASCII lowercase permutation or
/// `message` doesn't consist of ASCII lowercase letters