//! Methods for encrypting and decrypting messages using substitution cipher.

use crate::check_message_from_letters;

/// Encrypts `message` by substitution according to `permutation`.
/// Every letter is substituted for letter at position (letter's alphabet index)
/// in `permutation` array.
///
/// # Panics
/// Panics if `permutation` is not a valid ASCII lowercase permutation or
/// `message` doesn't consist of ASCII lowercase letters.
///
/// # Examples
/// ```
/// use crypto::substitution::substitution_cipher;
///
/// let message = b"hello";
/// let permutation = b"zyxwvutsrqponmlkjihgfedcba";
/// let encrypted_message = substitution_cipher(message, *permutation);

/// assert_eq!(b"svool", &encrypted_message[..]);
/// ```
pub fn substitution_cipher(message: &[u8], permutation: [u8; 26]) -> Vec<u8> {
    check_permutation(permutation);
    check_message_from_letters(message);

    message
        .iter()
        .map(|b| b - 97)
        .map(|b| permutation[b as usize])
        .collect()
}

/// Decrypts `encrypted_message` which was encrypted by `substitution_cipher`
/// function with `permutation`.
///
/// # Panics
/// Panics if `permutation` is not a valid ASCII lowercase permutation or
/// `message` doesn't consist of ASCII lowercase letters.
///
/// # Examples
/// ```
/// use crypto::substitution::substitution_cipher_decrypt;
///
/// let encrypted_message = b"svool";
/// let permutation = b"zyxwvutsrqponmlkjihgfedcba";
/// let message = substitution_cipher_decrypt(encrypted_message, *permutation);
///
/// assert_eq!(b"hello", &message[..]);
/// ```
pub fn substitution_cipher_decrypt(encrypted_message: &[u8], permutation: [u8; 26]) -> Vec<u8> {
    check_permutation(permutation);
    check_message_from_letters(encrypted_message);

    encrypted_message
        .iter()
        .map(|b| index_in_permutation(*b, permutation) as u8)
        .map(|b| b + 97)
        .collect()
}

/// Finds index of `letter` in the `permutation`.
/// You have to check before calling this function if `letter` is
/// valid ASCII lowercase letter and `permutation` is valid permutation.
fn index_in_permutation(letter: u8, permutation: [u8; 26]) -> usize {
    permutation
        .iter()
        .enumerate()
        .filter(|(_, &l)| letter == l)
        .next()
        .unwrap()
        .0
}

/// Checks if `permutation` is valid ASCII lowercase letters permutation.
fn check_permutation(permutation: [u8; 26]) {
    let mut letters = [0; 26];

    for letter in permutation {
        println!("{}", letter);
        assert!(letter >= 97 && letter <= 122, "not a permutation");

        let letter = letter - 97;

        assert!(letters[letter as usize] == 0, "not a permutation");

        letters[letter as usize] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn not_a_permutation() {
        let permutation = b"asdasdasdasdasdasdasdasdas";
        check_permutation(*permutation);
    }

    #[test]
    fn permutation() {
        let permutation = b"zyxwvutsrqponmlkjihgfedcba";
        check_permutation(*permutation);
    }

    #[test]
    fn substitution_works() {
        let message = b"hello";
        let permutation = b"zyxwvutsrqponmlkjihgfedcba";
        let encrypted_message = substitution_cipher(message, *permutation);
        let decrypted_message = substitution_cipher_decrypt(&encrypted_message, *permutation);

        assert_eq!(message, &decrypted_message[..]);
    }
}
