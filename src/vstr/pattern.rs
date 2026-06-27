use std::error::Error;
use std::fmt;

/// Error returned when a regex-backed pattern cannot be compiled.
///
/// This type intentionally keeps the public error boundary small instead of
/// exposing the selected regex engine's concrete error type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternError {
    pattern: String,
    message: String,
}

impl PatternError {
    /// Returns the pattern that failed to compile.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr;
    ///
    /// let err = vstr::contains_pattern("rust", "[").unwrap_err();
    /// assert_eq!(err.pattern(), "[");
    /// # }
    /// ```
    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Returns the underlying pattern compilation message.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr;
    ///
    /// let err = vstr::find_pattern("rust", "[").unwrap_err();
    /// assert!(!err.message().is_empty());
    /// # }
    /// ```
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PatternError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid regex pattern {:?}: {}",
            self.pattern, self.message
        )
    }
}

impl Error for PatternError {}

impl From<regex::Error> for PatternError {
    fn from(error: regex::Error) -> Self {
        Self {
            pattern: String::new(),
            message: error.to_string(),
        }
    }
}

/// Returns `true` when `pattern` matches `input`.
///
/// This helper is available only with the `pattern-regex` feature. The default
/// crate remains dependency-free for runtime code.
///
/// # Errors
///
/// Returns [`PatternError`] when `pattern` is not a valid regex.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "pattern-regex")]
/// # {
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains_pattern("ticket-42", r"\d+").unwrap());
/// assert!(!vstr::contains_pattern("ticket", r"\d+").unwrap());
/// # }
/// ```
pub fn contains_pattern(input: &str, pattern: &str) -> Result<bool, PatternError> {
    Ok(compile(pattern)?.is_match(input))
}

/// Returns the first regex-backed match as a byte range.
///
/// The returned range uses byte offsets into `input`, matching Rust string
/// slicing conventions.
///
/// # Errors
///
/// Returns [`PatternError`] when `pattern` is not a valid regex.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "pattern-regex")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::find_pattern("ticket-42", r"\d+").unwrap(), Some((7, 9)));
/// # }
/// ```
pub fn find_pattern(input: &str, pattern: &str) -> Result<Option<(usize, usize)>, PatternError> {
    Ok(compile(pattern)?
        .find(input)
        .map(|matched| (matched.start(), matched.end())))
}

/// Returns all non-overlapping regex-backed match byte ranges.
///
/// # Errors
///
/// Returns [`PatternError`] when `pattern` is not a valid regex.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "pattern-regex")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(
///     vstr::find_all_patterns("a1 b22 c333", r"\d+").unwrap(),
///     vec![(1, 2), (4, 6), (8, 11)]
/// );
/// # }
/// ```
pub fn find_all_patterns(input: &str, pattern: &str) -> Result<Vec<(usize, usize)>, PatternError> {
    Ok(compile(pattern)?
        .find_iter(input)
        .map(|matched| (matched.start(), matched.end()))
        .collect())
}

/// Replaces all non-overlapping regex-backed matches with `replacement`.
///
/// Replacement syntax follows the Rust `regex` crate, including `$name` and
/// `$1` capture references.
///
/// # Errors
///
/// Returns [`PatternError`] when `pattern` is not a valid regex.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "pattern-regex")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(
///     vstr::replace_pattern("ticket-42 user-7", r"\d+", "#").unwrap(),
///     "ticket-# user-#"
/// );
/// # }
/// ```
pub fn replace_pattern(
    input: &str,
    pattern: &str,
    replacement: &str,
) -> Result<String, PatternError> {
    Ok(compile(pattern)?
        .replace_all(input, replacement)
        .into_owned())
}

fn compile(pattern: &str) -> Result<regex::Regex, PatternError> {
    regex::Regex::new(pattern).map_err(|error| PatternError {
        pattern: pattern.to_owned(),
        message: error.to_string(),
    })
}
