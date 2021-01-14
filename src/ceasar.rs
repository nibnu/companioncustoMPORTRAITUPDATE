
//! Methods for encrypting and decrypting messages using ceasar cipher.

use crate::check_message_from_letters;

/// Encrypts the `message` using ceasar cipher with `key`.
///
/// # Panics
/// Panics if `key > 25` or `message` doesn't consist of ASCII lowercase letters.
///
/// # Examples
/// ```
/// use crypto::ceasar::ceasar_cipher;
///
/// let message = b"hello";
/// let encrypted_message = ceasar_cipher(message, 15);
/// assert_eq!(b"wtaad", &encrypted_message[..]);
///
/// ```
pub fn ceasar_cipher(message: &[u8], key: u8) -> Vec<u8> {