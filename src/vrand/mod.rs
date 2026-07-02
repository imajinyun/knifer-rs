//! Random string, token, and value helpers.
//!
//! This facade covers the everyday "give me a random id/token/string" need
//! without pulling a general-purpose random-number framework into the default
//! build.
//!
//! Module navigation:
//!
//! - The [`VRand`] pseudo-random generator and the free `random_*` helpers in
//!   this file are part of the default zero-dependency facade. They are seedable
//!   and reproducible, but **not** cryptographically secure: never use them for
//!   secrets, passwords, session tokens, API keys, or nonces.
//! - `secure` contains cryptographically secure helpers (`secure_bytes`,
//!   `secure_string`, `secure_string_from`, `secure_hex`) behind the
//!   `random-secure` feature, backed by the operating system CSPRNG. Use those
//!   for anything security-sensitive.
//!
//! # Security
//!
//! [`VRand`] is a deterministic [`SplitMix64`](https://prng.di.unimi.it/splitmix64.c)
//! generator. It is fast and reproducible from a seed, which is ideal for
//! sampling, fixtures, and tests, but its output is predictable and must not be
//! used where unpredictability matters. Enable the `random-secure` feature and
//! use the `secure_*` helpers for cryptographic randomness.
//!
//! # Examples
//!
//! ```
//! use kniferrs::vrand;
//!
//! // Non-crypto convenience helpers (fresh entropy per thread).
//! let id = vrand::random_string(10);
//! assert_eq!(id.chars().count(), 10);
//! assert!(id.chars().all(|ch| ch.is_ascii_alphanumeric()));
//!
//! // Reproducible sequences from an explicit seed.
//! use kniferrs::vrand::VRand;
//! let mut left = VRand::seeded(42);
//! let mut right = VRand::seeded(42);
//! assert_eq!(left.string(8), right.string(8));
//! ```

use std::cell::RefCell;

#[cfg(feature = "random-secure")]
mod secure;

#[cfg(feature = "random-secure")]
pub use secure::*;

/// ASCII decimal digits `0-9`.
pub const DIGITS: &str = "0123456789";

/// ASCII lowercase letters `a-z`.
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

/// ASCII uppercase letters `A-Z`.
pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// ASCII digits and letters (`0-9`, `A-Z`, `a-z`).
pub const ALPHANUMERIC: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Lowercase hexadecimal digits (`0-9`, `a-f`).
pub const HEX: &str = "0123456789abcdef";

/// URL-safe token alphabet: [`ALPHANUMERIC`] plus `-` and `_`.
pub const URL_SAFE: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";

/// A fast, seedable, **non-cryptographic** pseudo-random generator.
///
/// `VRand` implements the `SplitMix64` algorithm. Given the same seed it
/// produces the same sequence, which makes it reproducible for tests and
/// sampling. It is not suitable for security-sensitive randomness; use the
/// `secure_*` helpers (feature `random-secure`) for that.
///
/// # Examples
///
/// ```
/// use kniferrs::vrand::VRand;
///
/// let mut rng = VRand::seeded(1);
/// let a = rng.next_u64();
/// let b = rng.next_u64();
/// assert_ne!(a, b);
/// ```
#[derive(Clone, Debug)]
pub struct VRand {
    state: u64,
}

impl VRand {
    /// Creates a generator seeded from process entropy.
    ///
    /// The seed is derived from the standard library's per-process random hash
    /// state mixed with the current time and a monotonic counter, so separate
    /// calls yield different sequences. The result is still **not**
    /// cryptographically secure.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::from_entropy();
    /// let value = rng.next_u64();
    /// let _ = value;
    /// ```
    #[must_use]
    pub fn from_entropy() -> Self {
        Self::seeded(entropy_seed())
    }

    /// Creates a generator with an explicit seed for reproducible sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut left = VRand::seeded(7);
    /// let mut right = VRand::seeded(7);
    /// assert_eq!(left.next_u64(), right.next_u64());
    /// ```
    #[must_use]
    pub const fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Returns the next pseudo-random `u64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(99);
    /// let _ = rng.next_u64();
    /// ```
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Returns the next pseudo-random `u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(3);
    /// let _ = rng.next_u32();
    /// ```
    #[must_use]
    pub fn next_u32(&mut self) -> u32 {
        u32::try_from(self.next_u64() >> 32).unwrap_or(u32::MAX)
    }

    /// Returns a pseudo-random `u64` in `[0, bound)`, or `0` when `bound` is `0`.
    ///
    /// Sampling uses rejection to avoid modulo bias, so every value in the
    /// range is equally likely.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(5);
    /// assert!(rng.below(10) < 10);
    /// assert_eq!(rng.below(0), 0);
    /// ```
    #[must_use]
    pub fn below(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            return 0;
        }
        let zone = (u64::MAX / bound) * bound;
        loop {
            let value = self.next_u64();
            if value < zone {
                return value % bound;
            }
        }
    }

    /// Returns a pseudo-random `i64` in `[low, high)`, or `low` when
    /// `high <= low`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(11);
    /// let value = rng.range(-5, 5);
    /// assert!((-5..5).contains(&value));
    /// assert_eq!(rng.range(3, 3), 3);
    /// ```
    #[must_use]
    pub fn range(&mut self, low: i64, high: i64) -> i64 {
        if high <= low {
            return low;
        }
        let span = u64::try_from(i128::from(high) - i128::from(low)).unwrap_or(u64::MAX);
        let offset = self.below(span);
        i64::try_from(i128::from(low) + i128::from(offset)).unwrap_or(high)
    }

    /// Returns a pseudo-random boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(13);
    /// let _ = rng.bool();
    /// ```
    #[must_use]
    pub fn bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Builds a random string of `len` characters from a custom `alphabet`.
    ///
    /// Returns an empty string when `alphabet` is empty or `len` is `0`.
    /// Characters are sampled uniformly by Unicode scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::{self, VRand};
    ///
    /// let mut rng = VRand::seeded(21);
    /// let value = rng.string_from(vrand::HEX, 6);
    /// assert_eq!(value.len(), 6);
    /// assert!(value.chars().all(|ch| ch.is_ascii_hexdigit()));
    /// assert!(rng.string_from("", 6).is_empty());
    /// ```
    #[must_use]
    pub fn string_from(&mut self, alphabet: &str, len: usize) -> String {
        let chars: Vec<char> = alphabet.chars().collect();
        if chars.is_empty() || len == 0 {
            return String::new();
        }
        let bound = u64::try_from(chars.len()).unwrap_or(u64::MAX);
        let mut output = String::with_capacity(len);
        for _ in 0..len {
            let index = usize::try_from(self.below(bound)).unwrap_or(0);
            if let Some(&ch) = chars.get(index) {
                output.push(ch);
            }
        }
        output
    }

    /// Builds a random alphanumeric string of `len` characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(33);
    /// let value = rng.string(12);
    /// assert_eq!(value.len(), 12);
    /// assert!(value.chars().all(|ch| ch.is_ascii_alphanumeric()));
    /// ```
    #[must_use]
    pub fn string(&mut self, len: usize) -> String {
        self.string_from(ALPHANUMERIC, len)
    }

    /// Returns a random element of `items`, or `None` when `items` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(4);
    /// let items = ["a", "b", "c"];
    /// assert!(items.contains(rng.choose(&items).unwrap()));
    /// assert_eq!(rng.choose::<u8>(&[]), None);
    /// ```
    pub fn choose<'items, T>(&mut self, items: &'items [T]) -> Option<&'items T> {
        if items.is_empty() {
            return None;
        }
        let bound = u64::try_from(items.len()).unwrap_or(u64::MAX);
        let index = usize::try_from(self.below(bound)).unwrap_or(0);
        items.get(index)
    }

    /// Shuffles `items` in place using an unbiased Fisher-Yates pass.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vrand::VRand;
    ///
    /// let mut rng = VRand::seeded(8);
    /// let mut items = [1, 2, 3, 4, 5];
    /// rng.shuffle(&mut items);
    /// let mut sorted = items;
    /// sorted.sort_unstable();
    /// assert_eq!(sorted, [1, 2, 3, 4, 5]);
    /// ```
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for upper in (1..items.len()).rev() {
            let bound = u64::try_from(upper + 1).unwrap_or(u64::MAX);
            let target = usize::try_from(self.below(bound)).unwrap_or(0);
            items.swap(upper, target);
        }
    }
}

impl Default for VRand {
    fn default() -> Self {
        Self::from_entropy()
    }
}

fn entropy_seed() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let mut hasher = RandomState::new().build_hasher();
    if let Ok(elapsed) = SystemTime::now().duration_since(UNIX_EPOCH) {
        hasher.write_u128(elapsed.as_nanos());
    }
    hasher.write_u64(COUNTER.fetch_add(1, Ordering::Relaxed));
    // Guard against a degenerate all-zero SplitMix64 state.
    hasher.finish() ^ 0x9E37_79B9_7F4A_7C15
}

thread_local! {
    static THREAD_RNG: RefCell<VRand> = RefCell::new(VRand::from_entropy());
}

fn with_thread_rng<R>(action: impl FnOnce(&mut VRand) -> R) -> R {
    THREAD_RNG.with_borrow_mut(action)
}

/// Returns a random alphanumeric string of `len` characters.
///
/// This is a convenience wrapper over a thread-local [`VRand`]. It is **not**
/// cryptographically secure; use `secure_string` (feature `random-secure`) for
/// tokens and secrets.
///
/// # Examples
///
/// ```
/// use kniferrs::vrand;
///
/// let value = vrand::random_string(16);
/// assert_eq!(value.chars().count(), 16);
/// assert!(value.chars().all(|ch| ch.is_ascii_alphanumeric()));
/// ```
#[must_use]
pub fn random_string(len: usize) -> String {
    with_thread_rng(|rng| rng.string(len))
}

/// Returns a random string of `len` characters sampled from `alphabet`.
///
/// Not cryptographically secure. See `secure_string_from` (feature
/// `random-secure`) for secrets.
///
/// # Examples
///
/// ```
/// use kniferrs::vrand;
///
/// let value = vrand::random_string_from("ATCG", 8);
/// assert_eq!(value.len(), 8);
/// assert!(value.chars().all(|ch| "ATCG".contains(ch)));
/// ```
#[must_use]
pub fn random_string_from(alphabet: &str, len: usize) -> String {
    with_thread_rng(|rng| rng.string_from(alphabet, len))
}

/// Returns a random numeric string of `len` digits.
///
/// The result may have leading zeros, so treat it as text rather than a number.
///
/// # Examples
///
/// ```
/// use kniferrs::vrand;
///
/// let value = vrand::random_digits(6);
/// assert_eq!(value.len(), 6);
/// assert!(value.chars().all(|ch| ch.is_ascii_digit()));
/// ```
#[must_use]
pub fn random_digits(len: usize) -> String {
    with_thread_rng(|rng| rng.string_from(DIGITS, len))
}

/// Returns a random lowercase hexadecimal string of `len` characters.
///
/// Not cryptographically secure. See `secure_hex` (feature `random-secure`) for
/// tokens and secrets.
///
/// # Examples
///
/// ```
/// use kniferrs::vrand;
///
/// let value = vrand::random_hex(12);
/// assert_eq!(value.len(), 12);
/// assert!(value.chars().all(|ch| ch.is_ascii_hexdigit()));
/// ```
#[must_use]
pub fn random_hex(len: usize) -> String {
    with_thread_rng(|rng| rng.string_from(HEX, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_generators_are_reproducible() {
        let mut left = VRand::seeded(0x1234_5678);
        let mut right = VRand::seeded(0x1234_5678);
        for _ in 0..64 {
            assert_eq!(left.next_u64(), right.next_u64());
        }
        assert_eq!(VRand::seeded(1).string(32), VRand::seeded(1).string(32));
    }

    #[test]
    fn distinct_seeds_diverge() {
        let mut left = VRand::seeded(1);
        let mut right = VRand::seeded(2);
        assert_ne!(left.next_u64(), right.next_u64());
    }

    #[test]
    fn below_respects_bounds_and_zero() {
        let mut rng = VRand::seeded(7);
        assert_eq!(rng.below(0), 0);
        assert_eq!(rng.below(1), 0);
        for _ in 0..1_000 {
            assert!(rng.below(37) < 37);
        }
    }

    #[test]
    fn range_stays_within_half_open_interval() {
        let mut rng = VRand::seeded(9);
        assert_eq!(rng.range(5, 5), 5);
        assert_eq!(rng.range(10, 3), 10);
        for _ in 0..1_000 {
            let value = rng.range(-20, 20);
            assert!((-20..20).contains(&value));
        }
        assert!((i64::MIN..i64::MAX).contains(&rng.range(i64::MIN, i64::MAX)));
    }

    #[test]
    fn string_helpers_honor_length_and_alphabet() {
        let mut rng = VRand::seeded(21);
        assert!(rng.string(0).is_empty());
        assert!(rng.string_from("", 8).is_empty());

        let alphanumeric = rng.string(64);
        assert_eq!(alphanumeric.chars().count(), 64);
        assert!(alphanumeric.chars().all(|ch| ch.is_ascii_alphanumeric()));

        let hex = rng.string_from(HEX, 40);
        assert_eq!(hex.len(), 40);
        assert!(hex.chars().all(|ch| ch.is_ascii_hexdigit()));

        let unicode = rng.string_from("你好🚀", 12);
        assert_eq!(unicode.chars().count(), 12);
        assert!(unicode.chars().all(|ch| "你好🚀".contains(ch)));
    }

    #[test]
    fn string_from_covers_the_whole_alphabet() {
        let mut rng = VRand::seeded(0xABCD);
        let sample = rng.string_from(DIGITS, 4_000);
        for digit in DIGITS.chars() {
            assert!(sample.contains(digit), "missing digit {digit}");
        }
    }

    #[test]
    fn choose_returns_member_or_none() {
        let mut rng = VRand::seeded(4);
        assert_eq!(rng.choose::<u8>(&[]), None);
        let items = ['a', 'b', 'c', 'd'];
        for _ in 0..100 {
            let chosen = rng.choose(&items).copied().expect("non-empty slice");
            assert!(items.contains(&chosen));
        }
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut rng = VRand::seeded(8);
        let mut items: Vec<u32> = (0..64).collect();
        rng.shuffle(&mut items);
        let mut sorted = items.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..64).collect::<Vec<_>>());
        assert_ne!(items, (0..64).collect::<Vec<_>>());

        let mut single = [42];
        rng.shuffle(&mut single);
        assert_eq!(single, [42]);
    }

    #[test]
    fn free_helpers_produce_expected_shape() {
        assert_eq!(random_string(0), "");
        assert!(
            random_string(24)
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric())
        );
        assert!(random_digits(10).chars().all(|ch| ch.is_ascii_digit()));
        assert!(random_hex(16).chars().all(|ch| ch.is_ascii_hexdigit()));
        assert_eq!(random_string_from("", 8), "");
        assert!(
            random_string_from("XY", 8)
                .chars()
                .all(|ch| "XY".contains(ch))
        );
    }

    #[test]
    fn entropy_seeded_generators_are_effectively_unique() {
        let first = VRand::from_entropy().next_u64();
        let second = VRand::from_entropy().next_u64();
        assert_ne!(first, second);
    }
}
