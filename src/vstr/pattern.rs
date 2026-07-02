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

/// A compiled, reusable regex-backed pattern.
///
/// The free `*_pattern` helpers compile a fresh regex on every call, which is
/// convenient for one-off matching. When the same pattern is reused across many
/// inputs, compile it once into a [`VRegex`] and reuse the handle instead. The
/// concrete engine type stays private so the public API surface does not depend
/// on the selected regex crate.
///
/// # Complexity
///
/// Compilation is a one-time cost paid by [`VRegex::new`]. Each match call runs
/// in linear time in the length of the input (the backing engine uses finite
/// automata and does not backtrack), so reusing a compiled `VRegex` avoids
/// repeating the compile cost of the free helpers.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "pattern-regex")]
/// # {
/// use knifer_rs::vstr::VRegex;
///
/// let digits = VRegex::new(r"\d+").unwrap();
/// assert!(digits.is_match("ticket-42"));
/// assert_eq!(digits.find("ticket-42"), Some((7, 9)));
/// assert_eq!(digits.find_all("a1 b22 c333"), vec![(1, 2), (4, 6), (8, 11)]);
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct VRegex {
    regex: regex::Regex,
}

impl VRegex {
    /// Compiles `pattern` into a reusable matcher.
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
    /// use knifer_rs::vstr::VRegex;
    ///
    /// assert!(VRegex::new(r"\d+").is_ok());
    /// assert_eq!(VRegex::new("[").unwrap_err().pattern(), "[");
    /// # }
    /// ```
    pub fn new(pattern: &str) -> Result<Self, PatternError> {
        Ok(Self {
            regex: compile(pattern)?,
        })
    }

    /// Returns the source pattern the matcher was compiled from.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr::VRegex;
    ///
    /// assert_eq!(VRegex::new(r"\d+").unwrap().as_str(), r"\d+");
    /// # }
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.regex.as_str()
    }

    /// Returns the number of capture groups, including the implicit whole-match
    /// group at index `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr::VRegex;
    ///
    /// assert_eq!(VRegex::new(r"(\d{4})-(\d{2})").unwrap().capture_count(), 3);
    /// # }
    /// ```
    #[must_use]
    pub fn capture_count(&self) -> usize {
        self.regex.captures_len()
    }

    /// Returns `true` when the pattern matches anywhere in `input`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr::VRegex;
    ///
    /// let digits = VRegex::new(r"\d+").unwrap();
    /// assert!(digits.is_match("ticket-42"));
    /// assert!(!digits.is_match("ticket"));
    /// # }
    /// ```
    #[must_use]
    pub fn is_match(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }

    /// Returns the first match as a byte range into `input`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr::VRegex;
    ///
    /// let digits = VRegex::new(r"\d+").unwrap();
    /// assert_eq!(digits.find("ticket-42"), Some((7, 9)));
    /// assert_eq!(digits.find("ticket"), None);
    /// # }
    /// ```
    #[must_use]
    pub fn find(&self, input: &str) -> Option<(usize, usize)> {
        self.regex
            .find(input)
            .map(|matched| (matched.start(), matched.end()))
    }

    /// Returns all non-overlapping matches as byte ranges into `input`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr::VRegex;
    ///
    /// let digits = VRegex::new(r"\d+").unwrap();
    /// assert_eq!(digits.find_all("a1 b22 c333"), vec![(1, 2), (4, 6), (8, 11)]);
    /// # }
    /// ```
    #[must_use]
    pub fn find_all(&self, input: &str) -> Vec<(usize, usize)> {
        self.regex
            .find_iter(input)
            .map(|matched| (matched.start(), matched.end()))
            .collect()
    }

    /// Returns the capture groups of the first match as byte ranges.
    ///
    /// Index `0` is the whole match; later indices are the parenthesized groups.
    /// A group that did not participate in the match is `None`. Returns `None`
    /// when the pattern does not match at all.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr::VRegex;
    ///
    /// let date = VRegex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    /// assert_eq!(
    ///     date.captures("2026-06-27"),
    ///     Some(vec![Some((0, 10)), Some((0, 4)), Some((5, 7)), Some((8, 10))])
    /// );
    /// assert_eq!(date.captures("not-a-date"), None);
    /// # }
    /// ```
    #[must_use]
    pub fn captures(&self, input: &str) -> Option<Vec<Option<(usize, usize)>>> {
        self.regex.captures(input).map(|caps| {
            caps.iter()
                .map(|group| group.map(|matched| (matched.start(), matched.end())))
                .collect()
        })
    }

    /// Replaces every non-overlapping match with `replacement`, returning a new
    /// string.
    ///
    /// Replacement syntax follows the Rust `regex` crate, including `$name` and
    /// `$1` capture references.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "pattern-regex")]
    /// # {
    /// use knifer_rs::vstr::VRegex;
    ///
    /// let digits = VRegex::new(r"\d+").unwrap();
    /// assert_eq!(digits.replace_all("ticket-42 user-7", "#"), "ticket-# user-#");
    ///
    /// let date = VRegex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    /// assert_eq!(date.replace_all("2026-06-27", "$2/$3/$1"), "06/27/2026");
    /// # }
    /// ```
    #[must_use]
    pub fn replace_all(&self, input: &str, replacement: &str) -> String {
        self.regex.replace_all(input, replacement).into_owned()
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
