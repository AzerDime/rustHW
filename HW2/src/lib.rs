//!Homework 2 for CS410P - Rust
//! Ian Guy 2022

use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

pub fn lambda(p: u32, q: u32) -> u64 {
    return lcm(u64::from(p - 1), u64::from(q - 1));
}

/// Generate a pair of primes in the range `2**31..2**32`
/// /// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    return modexp(u64::from(msg), EXP, u64::from(key));
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d = modinverse(EXP, lambda(key.0, key.1));
    let pq = u64::from(key.0) * u64::from(key.1);
    return u32::try_from(modexp(msg, d, pq)).unwrap();
}
