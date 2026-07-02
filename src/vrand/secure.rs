//! Cryptographically secure random helpers (feature `random-secure`).
//!
//! These functions draw from the operating system CSPRNG through the
//! [`getrandom`](https://docs.rs/getrandom) crate and **fail closed**: when the
//! platform cannot supply entropy they return an error instead of falling back
//! to predictable data. Use them for tokens, secrets, session ids, nonces, and
//! anything else where predictability would be a vulnerability.
//!
//! The non-secure [`VRand`](super::VRand) generator and the free `random_*`
//! helpers are faster and reproducible, but must never be used for secrets.

use super::{ALPHANUMERIC, HEX};

/// Error returned when the operating system cannot supply secure entropy.
///
/// This is a re-export of [`getrandom::Error`], surfaced so callers do not need
/// to depend on `getrandom` directly.
pub type SecureError = getrandom::Error;

/// Returns `len` cryptographically secure random bytes.
///
/// Fails closed: if the operating system CSPRNG is unavailable the error is
/// propagated rather than substituting predictable bytes.
///
/// # Errors
///
/// Returns [`SecureError`] when the platform entropy source fails.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "random-secure")] {
/// use kniferrs::vrand;
///
/// let key = vrand::secure_bytes(32).expect("system entropy");
/// assert_eq!(key.len(), 32);
/// # }
/// ```
pub fn secure_bytes(len: usize) -> Result<Vec<u8>, SecureError> {
    let mut buffer = vec![0u8; len];
    getrandom::fill(&mut buffer)?;
    Ok(buffer)
}

/// Draws a secure `u64` from the operating system CSPRNG.
fn secure_u64() -> Result<u64, SecureError> {
    let mut buffer = [0u8; 8];
    getrandom::fill(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer))
}

/// Returns a secure uniform value in `[0, bound)`, or `0` when `bound` is `0`.
///
/// Uses rejection sampling so the distribution is unbiased.
fn secure_below(bound: u64) -> Result<u64, SecureError> {
    if bound == 0 {
        return Ok(0);
    }
    let zone = (u64::MAX / bound) * bound;
    loop {
        let value = secure_u64()?;
        if value < zone {
            return Ok(value % bound);
        }
    }
}

/// Builds a secure random string of `len` characters sampled from `alphabet`.
///
/// Returns an empty string when `alphabet` is empty or `len` is `0`. Characters
/// are sampled uniformly (unbiased rejection sampling) by Unicode scalar value.
///
/// # Errors
///
/// Returns [`SecureError`] when the platform entropy source fails.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "random-secure")] {
/// use kniferrs::vrand;
///
/// let token = vrand::secure_string_from(vrand::URL_SAFE, 24).expect("entropy");
/// assert_eq!(token.chars().count(), 24);
/// assert!(vrand::secure_string_from("", 8).expect("entropy").is_empty());
/// # }
/// ```
pub fn secure_string_from(alphabet: &str, len: usize) -> Result<String, SecureError> {
    let chars: Vec<char> = alphabet.chars().collect();
    if chars.is_empty() || len == 0 {
        return Ok(String::new());
    }
    let bound = u64::try_from(chars.len()).unwrap_or(u64::MAX);
    let mut output = String::with_capacity(len);
    for _ in 0..len {
        let index = usize::try_from(secure_below(bound)?).unwrap_or(0);
        if let Some(&ch) = chars.get(index) {
            output.push(ch);
        }
    }
    Ok(output)
}

/// Builds a secure random alphanumeric string of `len` characters.
///
/// # Errors
///
/// Returns [`SecureError`] when the platform entropy source fails.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "random-secure")] {
/// use kniferrs::vrand;
///
/// let id = vrand::secure_string(20).expect("entropy");
/// assert_eq!(id.len(), 20);
/// assert!(id.chars().all(|ch| ch.is_ascii_alphanumeric()));
/// # }
/// ```
pub fn secure_string(len: usize) -> Result<String, SecureError> {
    secure_string_from(ALPHANUMERIC, len)
}

/// Builds a secure random lowercase hexadecimal string of `len` characters.
///
/// # Errors
///
/// Returns [`SecureError`] when the platform entropy source fails.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "random-secure")] {
/// use kniferrs::vrand;
///
/// let token = vrand::secure_hex(32).expect("entropy");
/// assert_eq!(token.len(), 32);
/// assert!(token.chars().all(|ch| ch.is_ascii_hexdigit()));
/// # }
/// ```
pub fn secure_hex(len: usize) -> Result<String, SecureError> {
    secure_string_from(HEX, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secure_bytes_returns_requested_length() {
        assert!(secure_bytes(0).expect("entropy").is_empty());
        assert_eq!(secure_bytes(64).expect("entropy").len(), 64);
    }

    #[test]
    fn secure_bytes_are_not_all_zero() {
        let bytes = secure_bytes(128).expect("entropy");
        assert!(bytes.iter().any(|&byte| byte != 0));
    }

    #[test]
    fn secure_strings_honor_length_and_alphabet() {
        assert!(secure_string(0).expect("entropy").is_empty());
        assert!(secure_string_from("", 8).expect("entropy").is_empty());

        let alphanumeric = secure_string(64).expect("entropy");
        assert_eq!(alphanumeric.chars().count(), 64);
        assert!(alphanumeric.chars().all(|ch| ch.is_ascii_alphanumeric()));

        let hex = secure_hex(48).expect("entropy");
        assert_eq!(hex.len(), 48);
        assert!(hex.chars().all(|ch| ch.is_ascii_hexdigit()));

        let unicode = secure_string_from("你好🚀", 12).expect("entropy");
        assert_eq!(unicode.chars().count(), 12);
        assert!(unicode.chars().all(|ch| "你好🚀".contains(ch)));
    }

    #[test]
    fn secure_strings_are_effectively_unique() {
        let first = secure_string(32).expect("entropy");
        let second = secure_string(32).expect("entropy");
        assert_ne!(first, second);
    }
}
