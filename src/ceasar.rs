
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
    check_ceasar_key(key);
    check_message_from_letters(message);

    message
        .iter()
        .map(|b| b - 97)
        .map(|b| (b + key) % 26)
        .map(|b| b + 97)
        .collect()
}

/// Decrypts the `encrypted_message` using ceasar cipher with `key`.
///
/// # Panics
/// Panics if `key > 25` or `encrypted_message` doesn't consist of ASCII lowercase letters.
///
/// # Examples
/// ```
/// use crypto::ceasar::ceasar_cipher_decrypt;
///
/// let encrypted_message = b"wtaad";
/// let message = ceasar_cipher_decrypt(encrypted_message, 15);
/// assert_eq!(b"hello", &message[..]);
///
/// ```
pub fn ceasar_cipher_decrypt(encrypted_message: &[u8], key: u8) -> Vec<u8> {
    check_ceasar_key(key);
    check_message_from_letters(encrypted_message);

    encrypted_message
        .iter()
        .map(|b| b - 97)
        .map(|b| b + 26)
        .map(|b| (b - key) % 26)
        .map(|b| b + 97)
        .collect()
}