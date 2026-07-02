# Public API Inventory

This file tracks the public surface of `kniferrs` facade modules. It is
checked by `bin/check-public-api-inventory.sh` and `bin/check-api-semver.sh`;
update the signature snapshot only after reviewing semver impact.

The full signature snapshot below is collected from the all-features API
surface. The same check also computes the default zero-runtime-dependency API
surface and verifies that the feature-gated delta matches the optional snapshot
in this file.

`bin/check-public-api-inventory.sh` keeps curated file arrays for the default
and all-features surfaces. It also guards that those arrays cover every non-test
source file which declares a public item, so a new module cannot add public API
that silently escapes this inventory. When adding such a module, add it to
`default_api_files` or `all_features_api_files` in that script.

## API Stability Classes

`knifer-rs` is still pre-1.0, but public APIs are classified so callers know
which behavior is already treated as a strong contract.

### Core Stable Facade

The core stable facade is available in the default zero-runtime-dependency
build. For this class, signature changes here are treated as breaking unless
they are purely additive. Behavior is locked by unit tests, doctests, golden
fixtures, fuzz smoke, and project contract checks.

Core stable areas:

- `kniferrs::vstr`: trim, blank/empty predicates, substring helpers, scalar
  text helpers, case conversion, English word/identifier inflection
  (`pluralize`/`singularize`, `ordinalize`/`deordinalize`, `humanize`,
  `titleize`, `camelize`), replacement, literal search, escaping, Ant
  path matching, emoji helpers, and similarity helpers (Levenshtein, Jaro,
  Jaro-Winkler, Damerau, optimal string alignment, Sørensen-Dice, Jaccard,
  n-gram, and `SimHash`).
- `kniferrs::vbytes`: byte length, UTF-8 validation, byte slicing, ASCII trim,
  byte search, prefix/suffix stripping, byte replacement, lossy UTF-8 decoding
  (`chars`, `char_indices`), and byte line/field splitting (`lines`, `fields`).
- `kniferrs::vencoding`: BOM detection, BOM stripping, UTF-8 validation, and
  lossy UTF-8 decoding boundaries.

### Optional Feature Facade

The optional feature facade is public only when its feature is enabled. These APIs are absent from the default build and are listed in the optional signature snapshot below. Once enabled, their signatures are checked with the same semver-aware inventory gates as the all-features surface.

Optional feature areas:

- `pattern-regex`: regex-backed `contains_pattern`, `find_pattern`,
  `find_all_patterns`, `replace_pattern`, and `PatternError`, plus the reusable
  compiled `VRegex` facade (`is_match`, `find`, `find_all`, `captures`,
  `captures_named`, `replace_all`, `split`, `splitn`) for matching one pattern
  against many inputs.
- `unicode-segmentation`: grapheme, word, and sentence boundary helpers.
- `unicode-normalization`: UAX #15 normalization forms `nfc`, `nfd`, `nfkc`,
  `nfkd` and quick-check predicates `is_nfc`, `is_nfd`, `is_nfkc`, `is_nfkd`.
- `unicode-width`: display-cell width, width truncation, and width wrapping.
- `transliterate`: full ASCII transliteration `transliterate` for non-Latin
  scripts, plus `slugify_ascii` and `slugify_ascii_with_separator` for ASCII
  slugs from any script (the default `slugify` keeps diacritic folding only).

### Experimental-But-Public Facade

Some public APIs are intentionally useful now but may still receive pre-1.0
semantic tuning as long as signature and behavior changes are recorded.
`VStrMatcher`, `VStrMatch`, and `MatchKind` are in this class because the
default implementation and the optional `matcher-aho-corasick` backend must keep
the same public semantics, while backend internals can still evolve.

Experimental-but-public behavior is still covered by parity tests, fuzz smoke,
and the all-features inventory; it is not a private implementation detail.

## All-Features Public API Signature Snapshot

<!-- public-api-signatures:start -->
kniferrs::vbytes = pub mod vbytes
kniferrs::vbytes::byte_len = pub const fn byte_len(input: &[u8]) -> usize
kniferrs::vbytes::char_indices = pub fn char_indices(input: &[u8]) -> Vec<(usize, usize, char)>
kniferrs::vbytes::chars = pub fn chars(input: &[u8]) -> Vec<char>
kniferrs::vbytes::contains = pub fn contains(input: &[u8], needle: &[u8]) -> bool
kniferrs::vbytes::fields = pub fn fields(input: &[u8]) -> Vec<&[u8]>
kniferrs::vbytes::find = pub fn find(input: &[u8], needle: &[u8]) -> Option<(usize, usize)>
kniferrs::vbytes::find_all = pub fn find_all(input: &[u8], needle: &[u8]) -> Vec<(usize, usize)>
kniferrs::vbytes::is_empty = pub const fn is_empty(input: &[u8]) -> bool
kniferrs::vbytes::is_utf8 = pub const fn is_utf8(input: &[u8]) -> bool
kniferrs::vbytes::lines = pub fn lines(input: &[u8]) -> Vec<&[u8]>
kniferrs::vbytes::replace_all = pub fn replace_all(input: &[u8], from: &[u8], to: &[u8]) -> Vec<u8>
kniferrs::vbytes::strip_prefix = pub fn strip_prefix<'src>(input: &'src [u8], prefix: &[u8]) -> Option<&'src [u8]>
kniferrs::vbytes::strip_suffix = pub fn strip_suffix<'src>(input: &'src [u8], suffix: &[u8]) -> Option<&'src [u8]>
kniferrs::vbytes::sub = pub fn sub(input: &[u8], from_index: isize, to_index: isize) -> &[u8]
kniferrs::vbytes::to_str = pub const fn to_str(input: &[u8]) -> Result<&str, core::str::Utf8Error>
kniferrs::vbytes::trim_ascii = pub fn trim_ascii(input: &[u8]) -> &[u8]
kniferrs::vbytes::trim_ascii_end = pub fn trim_ascii_end(input: &[u8]) -> &[u8]
kniferrs::vbytes::trim_ascii_start = pub fn trim_ascii_start(input: &[u8]) -> &[u8]
kniferrs::vencoding = pub mod vencoding
kniferrs::vencoding::Bom = pub enum Bom
kniferrs::vencoding::Bom::byte_len = pub const fn byte_len(self) -> usize
kniferrs::vencoding::Bom::encoding_name = pub const fn encoding_name(self) -> &'static str
kniferrs::vencoding::BomScan = pub struct BomScan<'src>
kniferrs::vencoding::decode_utf8_lossy = pub fn decode_utf8_lossy(input: &[u8]) -> Cow<'_, str>
kniferrs::vencoding::decode_utf8_lossy_without_bom = pub fn decode_utf8_lossy_without_bom(input: &[u8]) -> Cow<'_, str>
kniferrs::vencoding::detect_bom = pub const fn detect_bom(input: &[u8]) -> Option<Bom>
kniferrs::vencoding::is_utf8 = pub const fn is_utf8(input: &[u8]) -> bool
kniferrs::vencoding::scan_bom = pub fn scan_bom(input: &[u8]) -> BomScan<'_>
kniferrs::vencoding::strip_bom = pub fn strip_bom(input: &[u8]) -> &[u8]
kniferrs::vencoding::validate_utf8 = pub const fn validate_utf8(input: &[u8]) -> Result<&str, core::str::Utf8Error>
kniferrs::vencoding::validate_utf8_without_bom = pub fn validate_utf8_without_bom(input: &[u8]) -> Result<&str, core::str::Utf8Error>
kniferrs::vstr = pub mod vstr
kniferrs::vstr::EmojiOptions = pub struct EmojiOptions<'src>
kniferrs::vstr::EmojiOptions::new = pub const fn new() -> Self
kniferrs::vstr::EmojiOptions::with_matcher = pub fn with_matcher(mut self, matcher: impl Fn(&str) -> bool + 'src) -> Self
kniferrs::vstr::EmojiOptions::with_replacer = pub fn with_replacer(mut self, replacer: impl Fn(&str) -> String + 'src) -> Self
kniferrs::vstr::LongWordPolicy = pub enum LongWordPolicy
kniferrs::vstr::MatchKind = pub enum MatchKind
kniferrs::vstr::PatternError = pub struct PatternError
kniferrs::vstr::PatternError::message = pub fn message(&self) -> &str
kniferrs::vstr::PatternError::pattern = pub fn pattern(&self) -> &str
kniferrs::vstr::VRegex = pub struct VRegex
kniferrs::vstr::VRegex::as_str = pub fn as_str(&self) -> &str
kniferrs::vstr::VRegex::capture_count = pub fn capture_count(&self) -> usize
kniferrs::vstr::VRegex::captures = pub fn captures(&self, input: &str) -> Option<Vec<Option<(usize, usize)>>>
kniferrs::vstr::VRegex::captures_named = pub fn captures_named(&self, input: &str) -> Option<Vec<(String,(usize, usize))>>
kniferrs::vstr::VRegex::find = pub fn find(&self, input: &str) -> Option<(usize, usize)>
kniferrs::vstr::VRegex::find_all = pub fn find_all(&self, input: &str) -> Vec<(usize, usize)>
kniferrs::vstr::VRegex::is_match = pub fn is_match(&self, input: &str) -> bool
kniferrs::vstr::VRegex::new = pub fn new(pattern: &str) -> Result<Self, PatternError>
kniferrs::vstr::VRegex::replace_all = pub fn replace_all(&self, input: &str, replacement: &str) -> String
kniferrs::vstr::VRegex::split = pub fn split<'input>(&self, input: &'input str) -> Vec<&'input str>
kniferrs::vstr::VRegex::splitn = pub fn splitn<'input>(&self, input: &'input str, limit: usize) -> Vec<&'input str>
kniferrs::vstr::VStrMatch = pub struct VStrMatch<'needle>
kniferrs::vstr::VStrMatcher = pub struct VStrMatcher<'needle>
kniferrs::vstr::VStrMatcher::find = pub fn find(&self, input: &str) -> Option<VStrMatch<'needle>>
kniferrs::vstr::VStrMatcher::find_all = pub fn find_all(&self, input: &str) -> Vec<VStrMatch<'needle>>
kniferrs::vstr::VStrMatcher::find_overlapping = pub fn find_overlapping(&self, input: &str) -> Vec<VStrMatch<'needle>>
kniferrs::vstr::VStrMatcher::is_empty = pub fn is_empty(&self) -> bool
kniferrs::vstr::VStrMatcher::kind = pub const fn kind(&self) -> MatchKind
kniferrs::vstr::VStrMatcher::len = pub fn len(&self) -> usize
kniferrs::vstr::VStrMatcher::new = pub fn new<I>(needles: I) -> Self where I: IntoIterator<Item = &'needle str>
kniferrs::vstr::VStrMatcher::replace_all = pub fn replace_all<'replacement, I>(&self, input: &str, replacements: I) -> String where I: IntoIterator<Item = &'replacement str>
kniferrs::vstr::VStrMatcher::with_kind = pub fn with_kind<I>(needles: I, kind: MatchKind) -> Self where I: IntoIterator<Item = &'needle str>
kniferrs::vstr::WhitespaceMode = pub enum WhitespaceMode
kniferrs::vstr::WrapOptions = pub struct WrapOptions<'src>
kniferrs::vstr::WrapOptions::new = pub const fn new(width: usize) -> Self
kniferrs::vstr::WrapOptions::with_indent = pub const fn with_indent(mut self, initial_indent: &'src str, subsequent_indent: &'src str) -> Self
kniferrs::vstr::WrapOptions::with_long_word_policy = pub const fn with_long_word_policy(mut self, long_word_policy: LongWordPolicy) -> Self
kniferrs::vstr::WrapOptions::with_whitespace_mode = pub const fn with_whitespace_mode(mut self, whitespace_mode: WhitespaceMode) -> Self
kniferrs::vstr::WrapOptions::with_word_separators = pub const fn with_word_separators(mut self, word_separators: &'src [char]) -> Self
kniferrs::vstr::abbreviate_middle = pub fn abbreviate_middle(input: &str, max_chars: usize, marker: &str) -> String
kniferrs::vstr::add_prefix_if_not = pub fn add_prefix_if_not(input: &str, prefix: &str) -> String
kniferrs::vstr::add_prefix_if_not_ignore_case = pub fn add_prefix_if_not_ignore_case(input: &str, prefix: &str) -> String
kniferrs::vstr::add_suffix_if_not = pub fn add_suffix_if_not(input: &str, suffix: &str) -> String
kniferrs::vstr::add_suffix_if_not_ignore_case = pub fn add_suffix_if_not_ignore_case(input: &str, suffix: &str) -> String
kniferrs::vstr::after = pub fn after<'src>(input: &'src str, separator: &str) -> &'src str
kniferrs::vstr::after_last = pub fn after_last<'src>(input: &'src str, separator: &str) -> &'src str
kniferrs::vstr::ant_path_match = pub fn ant_path_match(pattern: &str, path: &str) -> bool
kniferrs::vstr::ant_path_match_with_separator = pub fn ant_path_match_with_separator(pattern: &str, path: &str, separator: &str) -> bool
kniferrs::vstr::before = pub fn before<'src>(input: &'src str, separator: &str) -> &'src str
kniferrs::vstr::before_last = pub fn before_last<'src>(input: &'src str, separator: &str) -> &'src str
kniferrs::vstr::between = pub fn between<'src>(input: &'src str, start: &str, end: &str) -> Option<&'src str>
kniferrs::vstr::byte_len = pub const fn byte_len(input: &str) -> usize
kniferrs::vstr::camelize = pub fn camelize(input: &str) -> String
kniferrs::vstr::capitalize = pub fn capitalize(input: &str) -> String
kniferrs::vstr::center = pub fn center(input: &str, width: usize, pad: char) -> String
kniferrs::vstr::char_len = pub fn char_len(input: &str) -> usize
kniferrs::vstr::chars = pub fn chars(input: &str) -> Vec<char>
kniferrs::vstr::collapse_repeated_char = pub fn collapse_repeated_char(input: &str, ch: char) -> String
kniferrs::vstr::common_prefix = pub fn common_prefix<'src>(left: &'src str, right: &str) -> &'src str
kniferrs::vstr::common_suffix = pub fn common_suffix<'src>(left: &'src str, right: &str) -> &'src str
kniferrs::vstr::contains = pub fn contains(input: &str, needle: &str) -> bool
kniferrs::vstr::contains_all = pub fn contains_all<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::contains_all_ignore_case = pub fn contains_all_ignore_case<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::contains_any = pub fn contains_any<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::contains_any_ignore_case = pub fn contains_any_ignore_case<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::contains_emoji = pub fn contains_emoji(input: &str) -> bool
kniferrs::vstr::contains_emoji_with_options = pub fn contains_emoji_with_options(input: &str, options: &EmojiOptions<'_>) -> bool
kniferrs::vstr::contains_ignore_case = pub fn contains_ignore_case(input: &str, needle: &str) -> bool
kniferrs::vstr::contains_none = pub fn contains_none<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::contains_pattern = pub fn contains_pattern(input: &str, pattern: &str) -> Result<bool, PatternError>
kniferrs::vstr::count_matches = pub fn count_matches(input: &str, needle: &str) -> usize
kniferrs::vstr::damerau_levenshtein_distance = pub fn damerau_levenshtein_distance(left: &str, right: &str) -> usize
kniferrs::vstr::deburr = pub fn deburr(input: &str) -> String
kniferrs::vstr::dedent = pub fn dedent(input: &str) -> String
kniferrs::vstr::default_if_blank = pub fn default_if_blank<'src>(input: &'src str, default: &'src str) -> &'src str
kniferrs::vstr::default_if_empty = pub fn default_if_empty<'src>(input: &'src str, default: &'src str) -> &'src str
kniferrs::vstr::deordinalize = pub fn deordinalize(input: &str) -> String
kniferrs::vstr::difference = pub fn difference<'src>(left: &str, right: &'src str) -> &'src str
kniferrs::vstr::display_width = pub fn display_width(input: &str) -> usize
kniferrs::vstr::drop_chars = pub fn drop_chars(input: &str, count: usize) -> &str
kniferrs::vstr::ends_with = pub fn ends_with(input: &str, suffix: &str) -> bool
kniferrs::vstr::ends_with_ignore_case = pub fn ends_with_ignore_case(input: &str, suffix: &str) -> bool
kniferrs::vstr::equals_ignore_case = pub fn equals_ignore_case(left: &str, right: &str) -> bool
kniferrs::vstr::escape_html = pub fn escape_html(input: &str) -> String
kniferrs::vstr::escape_regex = pub fn escape_regex(input: &str) -> String
kniferrs::vstr::escape_unicode = pub fn escape_unicode(input: &str) -> String
kniferrs::vstr::excerpt = pub fn excerpt(input: &str, needle: &str, max_chars: usize, marker: &str) -> String
kniferrs::vstr::extract_digits = pub fn extract_digits(input: &str) -> String
kniferrs::vstr::find_all = pub fn find_all(input: &str, needle: &str) -> Vec<(usize, usize)>
kniferrs::vstr::find_all_ignore_case = pub fn find_all_ignore_case(input: &str, needle: &str) -> Vec<(usize, usize)>
kniferrs::vstr::find_all_patterns = pub fn find_all_patterns(input: &str, pattern: &str) -> Result<Vec<(usize, usize)>, PatternError>
kniferrs::vstr::find_any = pub fn find_any<'needle, I>(input: &str, needles: I) -> Option<(&'needle str, usize, usize)> where I: IntoIterator<Item = &'needle str>
kniferrs::vstr::find_pattern = pub fn find_pattern(input: &str, pattern: &str) -> Result<Option<(usize, usize)>, PatternError>
kniferrs::vstr::format = pub fn format(template: &str, args: &[&dyn std::fmt::Display]) -> String
kniferrs::vstr::grapheme_len = pub fn grapheme_len(input: &str) -> usize
kniferrs::vstr::graphemes = pub fn graphemes(input: &str) -> Vec<&str>
kniferrs::vstr::hamming_distance64 = pub const fn hamming_distance64(left: u64, right: u64) -> u32
kniferrs::vstr::has_blank = pub fn has_blank<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::has_empty = pub fn has_empty<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::human_bytes = pub fn human_bytes(bytes: u64) -> String
kniferrs::vstr::human_count = pub fn human_count(value: i64) -> String
kniferrs::vstr::human_duration = pub fn human_duration(duration: Duration) -> String
kniferrs::vstr::humanize = pub fn humanize(input: &str) -> String
kniferrs::vstr::indent = pub fn indent(input: &str, prefix: &str) -> String
kniferrs::vstr::initials = pub fn initials(input: &str) -> String
kniferrs::vstr::is_all_blank = pub fn is_all_blank<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::is_all_empty = pub fn is_all_empty<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
kniferrs::vstr::is_ascii = pub const fn is_ascii(ch: char) -> bool
kniferrs::vstr::is_blank = pub fn is_blank(input: &str) -> bool
kniferrs::vstr::is_blank_char = pub fn is_blank_char(ch: char) -> bool
kniferrs::vstr::is_digit = pub fn is_digit(ch: char) -> bool
kniferrs::vstr::is_empty = pub const fn is_empty(input: &str) -> bool
kniferrs::vstr::is_letter = pub fn is_letter(ch: char) -> bool
kniferrs::vstr::is_letter_or_digit = pub fn is_letter_or_digit(ch: char) -> bool
kniferrs::vstr::is_nfc = pub fn is_nfc(input: &str) -> bool
kniferrs::vstr::is_nfd = pub fn is_nfd(input: &str) -> bool
kniferrs::vstr::is_nfkc = pub fn is_nfkc(input: &str) -> bool
kniferrs::vstr::is_nfkd = pub fn is_nfkd(input: &str) -> bool
kniferrs::vstr::is_not_blank = pub fn is_not_blank(input: &str) -> bool
kniferrs::vstr::is_not_empty = pub const fn is_not_empty(input: &str) -> bool
kniferrs::vstr::is_palindrome = pub fn is_palindrome(input: &str) -> bool
kniferrs::vstr::jaccard_similarity = pub fn jaccard_similarity(left: &str, right: &str) -> f64
kniferrs::vstr::jaro_similarity = pub fn jaro_similarity(left: &str, right: &str) -> f64
kniferrs::vstr::jaro_winkler_similarity = pub fn jaro_winkler_similarity(left: &str, right: &str) -> f64
kniferrs::vstr::length = pub fn length(input: &str) -> usize
kniferrs::vstr::levenshtein_distance = pub fn levenshtein_distance(left: &str, right: &str) -> usize
kniferrs::vstr::levenshtein_similarity = pub fn levenshtein_similarity(left: &str, right: &str) -> f64
kniferrs::vstr::limit_words = pub fn limit_words(input: &str, max_words: usize, suffix: &str) -> String
kniferrs::vstr::line_count = pub fn line_count(input: &str) -> usize
kniferrs::vstr::lines = pub fn lines(input: &str) -> Vec<&str>
kniferrs::vstr::mask = pub fn mask(input: &str, visible_start: usize, visible_end: usize, mask: char) -> String
kniferrs::vstr::nfc = pub fn nfc(input: &str) -> String
kniferrs::vstr::nfd = pub fn nfd(input: &str) -> String
kniferrs::vstr::nfkc = pub fn nfkc(input: &str) -> String
kniferrs::vstr::nfkd = pub fn nfkd(input: &str) -> String
kniferrs::vstr::ngram_similarity = pub fn ngram_similarity(left: &str, right: &str, n: usize) -> f64
kniferrs::vstr::non_blank_lines = pub fn non_blank_lines(input: &str) -> Vec<&str>
kniferrs::vstr::normalize_newlines = pub fn normalize_newlines(input: &str) -> String
kniferrs::vstr::normalize_whitespace = pub fn normalize_whitespace(input: &str) -> String
kniferrs::vstr::number_format = pub fn number_format(value: i64) -> String
kniferrs::vstr::number_format_float = pub fn number_format_float(value: f64, decimals: usize) -> String
kniferrs::vstr::number_format_with = pub fn number_format_with(value: i64, separator: char) -> String
kniferrs::vstr::optimal_string_alignment = pub fn optimal_string_alignment(left: &str, right: &str) -> usize
kniferrs::vstr::ordinalize = pub fn ordinalize(value: i64) -> String
kniferrs::vstr::pad_left = pub fn pad_left(input: &str, target_len: usize, pad: char) -> String
kniferrs::vstr::pad_right = pub fn pad_right(input: &str, target_len: usize, pad: char) -> String
kniferrs::vstr::pluralize = pub fn pluralize(word: &str, count: i64) -> String
kniferrs::vstr::quote_meta = pub fn quote_meta(input: &str) -> String
kniferrs::vstr::remove_accents = pub fn remove_accents(input: &str) -> String
kniferrs::vstr::remove_ascii_punctuation = pub fn remove_ascii_punctuation(input: &str) -> String
kniferrs::vstr::remove_emoji = pub fn remove_emoji(input: &str) -> String
kniferrs::vstr::remove_emoji_with_options = pub fn remove_emoji_with_options(input: &str, options: &EmojiOptions<'_>) -> String
kniferrs::vstr::remove_prefix = pub fn remove_prefix<'src>(input: &'src str, prefix: &str) -> &'src str
kniferrs::vstr::remove_suffix = pub fn remove_suffix<'src>(input: &'src str, suffix: &str) -> &'src str
kniferrs::vstr::remove_whitespace = pub fn remove_whitespace(input: &str) -> String
kniferrs::vstr::repeat = pub fn repeat(input: &str, count: usize) -> String
kniferrs::vstr::replace_first = pub fn replace_first(input: &str, from: &str, to: &str) -> String
kniferrs::vstr::replace_ignore_case = pub fn replace_ignore_case(input: &str, from: &str, to: &str) -> String
kniferrs::vstr::replace_last = pub fn replace_last(input: &str, from: &str, to: &str) -> String
kniferrs::vstr::replace_many = pub fn replace_many<'src, I>(input: &str, replacements: I) -> String where I: IntoIterator<Item = (&'src str, &'src str)>
kniferrs::vstr::replace_pattern = pub fn replace_pattern(input: &str, pattern: &str, replacement: &str) -> Result<String, PatternError>
kniferrs::vstr::reverse = pub fn reverse(input: &str) -> String
kniferrs::vstr::rotate = pub fn rotate(input: &str, shift: isize) -> String
kniferrs::vstr::rune_len = pub fn rune_len(input: &str) -> usize
kniferrs::vstr::sim_hash = pub fn sim_hash(input: &str) -> u64
kniferrs::vstr::singularize = pub fn singularize(word: &str) -> String
kniferrs::vstr::slugify = pub fn slugify(input: &str) -> String
kniferrs::vstr::slugify_ascii = pub fn slugify_ascii(input: &str) -> String
kniferrs::vstr::slugify_ascii_with_separator = pub fn slugify_ascii_with_separator(input: &str, separator: char) -> String
kniferrs::vstr::slugify_with_separator = pub fn slugify_with_separator(input: &str, separator: char) -> String
kniferrs::vstr::sorensen_dice = pub fn sorensen_dice(left: &str, right: &str) -> f64
kniferrs::vstr::split = pub fn split<'src>(input: &'src str, separator: &str) -> Vec<&'src str>
kniferrs::vstr::split_once = pub fn split_once<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)>
kniferrs::vstr::split_once_last = pub fn split_once_last<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)>
kniferrs::vstr::split_sentence_bound_indices = pub fn split_sentence_bound_indices(input: &str) -> Vec<(usize, &str)>
kniferrs::vstr::split_sentence_bounds = pub fn split_sentence_bounds(input: &str) -> Vec<&str>
kniferrs::vstr::split_trim = pub fn split_trim<'src>(input: &'src str, separator: &str) -> Vec<&'src str>
kniferrs::vstr::split_word_bound_indices = pub fn split_word_bound_indices(input: &str) -> Vec<(usize, &str)>
kniferrs::vstr::split_word_bounds = pub fn split_word_bounds(input: &str) -> Vec<&str>
kniferrs::vstr::starts_with = pub fn starts_with(input: &str, prefix: &str) -> bool
kniferrs::vstr::starts_with_ignore_case = pub fn starts_with_ignore_case(input: &str, prefix: &str) -> bool
kniferrs::vstr::strip_prefix_ignore_case = pub fn strip_prefix_ignore_case<'src>(input: &'src str, prefix: &str) -> Option<&'src str>
kniferrs::vstr::strip_suffix_ignore_case = pub fn strip_suffix_ignore_case<'src>(input: &'src str, suffix: &str) -> Option<&'src str>
kniferrs::vstr::sub = pub fn sub(input: &str, from_index: isize, to_index: isize) -> String
kniferrs::vstr::sub_after = pub fn sub_after<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str
kniferrs::vstr::sub_before = pub fn sub_before<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str
kniferrs::vstr::surround = pub fn surround(input: &str, left: &str, right: &str) -> String
kniferrs::vstr::swap_case = pub fn swap_case(input: &str) -> String
kniferrs::vstr::take_chars = pub fn take_chars(input: &str, count: usize) -> &str
kniferrs::vstr::take_graphemes = pub fn take_graphemes(input: &str, count: usize) -> &str
kniferrs::vstr::take_width = pub fn take_width(input: &str, max_width: usize) -> &str
kniferrs::vstr::titleize = pub fn titleize(input: &str) -> String
kniferrs::vstr::to_camel_case = pub fn to_camel_case(input: &str) -> String
kniferrs::vstr::to_cobol_case = pub fn to_cobol_case(input: &str) -> String
kniferrs::vstr::to_dot_case = pub fn to_dot_case(input: &str) -> String
kniferrs::vstr::to_kebab_case = pub fn to_kebab_case(input: &str) -> String
kniferrs::vstr::to_pascal_case = pub fn to_pascal_case(input: &str) -> String
kniferrs::vstr::to_path_case = pub fn to_path_case(input: &str) -> String
kniferrs::vstr::to_screaming_kebab_case = pub fn to_screaming_kebab_case(input: &str) -> String
kniferrs::vstr::to_screaming_snake_case = pub fn to_screaming_snake_case(input: &str) -> String
kniferrs::vstr::to_sentence_case = pub fn to_sentence_case(input: &str) -> String
kniferrs::vstr::to_snake_case = pub fn to_snake_case(input: &str) -> String
kniferrs::vstr::to_title_case = pub fn to_title_case(input: &str) -> String
kniferrs::vstr::to_train_case = pub fn to_train_case(input: &str) -> String
kniferrs::vstr::to_underline_case = pub fn to_underline_case(input: &str) -> String
kniferrs::vstr::transliterate = pub fn transliterate(input: &str) -> String
kniferrs::vstr::trim = pub fn trim(input: &str) -> &str
kniferrs::vstr::trim_blank_lines = pub fn trim_blank_lines(input: &str) -> &str
kniferrs::vstr::trim_end = pub fn trim_end(input: &str) -> &str
kniferrs::vstr::trim_lines = pub fn trim_lines(input: &str) -> String
kniferrs::vstr::trim_start = pub fn trim_start(input: &str) -> &str
kniferrs::vstr::trim_to_empty = pub fn trim_to_empty(input: &str) -> &str
kniferrs::vstr::trim_to_string = pub fn trim_to_string(input: &str) -> String
kniferrs::vstr::truncate = pub fn truncate(input: &str, max_chars: usize) -> &str
kniferrs::vstr::truncate_graphemes = pub fn truncate_graphemes(input: &str, max_graphemes: usize, suffix: &str) -> String
kniferrs::vstr::truncate_width = pub fn truncate_width(input: &str, max_width: usize, suffix: &str) -> String
kniferrs::vstr::truncate_with_suffix = pub fn truncate_with_suffix(input: &str, max_chars: usize, suffix: &str) -> String
kniferrs::vstr::uncapitalize = pub fn uncapitalize(input: &str) -> String
kniferrs::vstr::unescape_html = pub fn unescape_html(input: &str) -> String
kniferrs::vstr::unescape_unicode = pub fn unescape_unicode(input: &str) -> String
kniferrs::vstr::unicode_sentence_len = pub fn unicode_sentence_len(input: &str) -> usize
kniferrs::vstr::unicode_sentences = pub fn unicode_sentences(input: &str) -> Vec<&str>
kniferrs::vstr::unicode_word_indices = pub fn unicode_word_indices(input: &str) -> Vec<(usize, &str)>
kniferrs::vstr::unicode_word_len = pub fn unicode_word_len(input: &str) -> usize
kniferrs::vstr::unicode_words = pub fn unicode_words(input: &str) -> Vec<&str>
kniferrs::vstr::unsurround = pub fn unsurround<'src>(input: &'src str, left: &str, right: &str) -> Option<&'src str>
kniferrs::vstr::with_emoji_matcher = pub fn with_emoji_matcher<'src>(matcher: impl Fn(&str) -> bool + 'src) -> EmojiOptions<'src>
kniferrs::vstr::with_emoji_replacer = pub fn with_emoji_replacer<'src>(replacer: impl Fn(&str) -> String + 'src) -> EmojiOptions<'src>
kniferrs::vstr::word_count = pub fn word_count(input: &str) -> usize
kniferrs::vstr::words = pub fn words(input: &str) -> Vec<&str>
kniferrs::vstr::wrap = pub fn wrap(input: &str, width: usize) -> String
kniferrs::vstr::wrap_if_missing = pub fn wrap_if_missing(input: &str, marker: &str) -> String
kniferrs::vstr::wrap_width = pub fn wrap_width(input: &str, width: usize) -> String
kniferrs::vstr::wrap_width_with_indent = pub fn wrap_width_with_indent(input: &str, width: usize, initial_indent: &str, subsequent_indent: &str) -> String
kniferrs::vstr::wrap_width_with_options = pub fn wrap_width_with_options(input: &str, options: &WrapOptions<'_>) -> String
kniferrs::vstr::wrap_with_indent = pub fn wrap_with_indent(input: &str, width: usize, initial_indent: &str, subsequent_indent: &str) -> String
kniferrs::vstr::wrap_with_options = pub fn wrap_with_options(input: &str, options: &WrapOptions<'_>) -> String
<!-- public-api-signatures:end -->

## Optional Feature Inventory

- `pattern-regex`
- `unicode-normalization`
- `unicode-segmentation`
- `unicode-width`

The default build does not expose the following signatures. These APIs appear
only when the matching optional feature is enabled, while the all-features
snapshot keeps their release signature stable.

<!-- public-api-optional-signatures:start -->
kniferrs::vstr::PatternError = pub struct PatternError
kniferrs::vstr::PatternError::message = pub fn message(&self) -> &str
kniferrs::vstr::PatternError::pattern = pub fn pattern(&self) -> &str
kniferrs::vstr::VRegex = pub struct VRegex
kniferrs::vstr::VRegex::as_str = pub fn as_str(&self) -> &str
kniferrs::vstr::VRegex::capture_count = pub fn capture_count(&self) -> usize
kniferrs::vstr::VRegex::captures = pub fn captures(&self, input: &str) -> Option<Vec<Option<(usize, usize)>>>
kniferrs::vstr::VRegex::captures_named = pub fn captures_named(&self, input: &str) -> Option<Vec<(String,(usize, usize))>>
kniferrs::vstr::VRegex::find = pub fn find(&self, input: &str) -> Option<(usize, usize)>
kniferrs::vstr::VRegex::find_all = pub fn find_all(&self, input: &str) -> Vec<(usize, usize)>
kniferrs::vstr::VRegex::is_match = pub fn is_match(&self, input: &str) -> bool
kniferrs::vstr::VRegex::new = pub fn new(pattern: &str) -> Result<Self, PatternError>
kniferrs::vstr::VRegex::replace_all = pub fn replace_all(&self, input: &str, replacement: &str) -> String
kniferrs::vstr::VRegex::split = pub fn split<'input>(&self, input: &'input str) -> Vec<&'input str>
kniferrs::vstr::VRegex::splitn = pub fn splitn<'input>(&self, input: &'input str, limit: usize) -> Vec<&'input str>
kniferrs::vstr::contains_pattern = pub fn contains_pattern(input: &str, pattern: &str) -> Result<bool, PatternError>
kniferrs::vstr::display_width = pub fn display_width(input: &str) -> usize
kniferrs::vstr::find_all_patterns = pub fn find_all_patterns(input: &str, pattern: &str) -> Result<Vec<(usize, usize)>, PatternError>
kniferrs::vstr::find_pattern = pub fn find_pattern(input: &str, pattern: &str) -> Result<Option<(usize, usize)>, PatternError>
kniferrs::vstr::grapheme_len = pub fn grapheme_len(input: &str) -> usize
kniferrs::vstr::graphemes = pub fn graphemes(input: &str) -> Vec<&str>
kniferrs::vstr::is_nfc = pub fn is_nfc(input: &str) -> bool
kniferrs::vstr::is_nfd = pub fn is_nfd(input: &str) -> bool
kniferrs::vstr::is_nfkc = pub fn is_nfkc(input: &str) -> bool
kniferrs::vstr::is_nfkd = pub fn is_nfkd(input: &str) -> bool
kniferrs::vstr::nfc = pub fn nfc(input: &str) -> String
kniferrs::vstr::nfd = pub fn nfd(input: &str) -> String
kniferrs::vstr::nfkc = pub fn nfkc(input: &str) -> String
kniferrs::vstr::nfkd = pub fn nfkd(input: &str) -> String
kniferrs::vstr::replace_pattern = pub fn replace_pattern(input: &str, pattern: &str, replacement: &str) -> Result<String, PatternError>
kniferrs::vstr::slugify_ascii = pub fn slugify_ascii(input: &str) -> String
kniferrs::vstr::slugify_ascii_with_separator = pub fn slugify_ascii_with_separator(input: &str, separator: char) -> String
kniferrs::vstr::split_sentence_bound_indices = pub fn split_sentence_bound_indices(input: &str) -> Vec<(usize, &str)>
kniferrs::vstr::split_sentence_bounds = pub fn split_sentence_bounds(input: &str) -> Vec<&str>
kniferrs::vstr::split_word_bound_indices = pub fn split_word_bound_indices(input: &str) -> Vec<(usize, &str)>
kniferrs::vstr::split_word_bounds = pub fn split_word_bounds(input: &str) -> Vec<&str>
kniferrs::vstr::take_graphemes = pub fn take_graphemes(input: &str, count: usize) -> &str
kniferrs::vstr::take_width = pub fn take_width(input: &str, max_width: usize) -> &str
kniferrs::vstr::transliterate = pub fn transliterate(input: &str) -> String
kniferrs::vstr::truncate_graphemes = pub fn truncate_graphemes(input: &str, max_graphemes: usize, suffix: &str) -> String
kniferrs::vstr::truncate_width = pub fn truncate_width(input: &str, max_width: usize, suffix: &str) -> String
kniferrs::vstr::unicode_sentence_len = pub fn unicode_sentence_len(input: &str) -> usize
kniferrs::vstr::unicode_sentences = pub fn unicode_sentences(input: &str) -> Vec<&str>
kniferrs::vstr::unicode_word_indices = pub fn unicode_word_indices(input: &str) -> Vec<(usize, &str)>
kniferrs::vstr::unicode_word_len = pub fn unicode_word_len(input: &str) -> usize
kniferrs::vstr::unicode_words = pub fn unicode_words(input: &str) -> Vec<&str>
kniferrs::vstr::wrap_width = pub fn wrap_width(input: &str, width: usize) -> String
kniferrs::vstr::wrap_width_with_indent = pub fn wrap_width_with_indent(input: &str, width: usize, initial_indent: &str, subsequent_indent: &str) -> String
kniferrs::vstr::wrap_width_with_options = pub fn wrap_width_with_options(input: &str, options: &WrapOptions<'_>) -> String
<!-- public-api-optional-signatures:end -->

## Facade Boundaries

Byte and encoding helpers stay separate from `kniferrs::vstr`.

## `vbytes` API Shape

`kniferrs::vbytes` is implemented as a byte-slice facade for data that may not
be valid UTF-8. All byte ranges are byte offsets. Byte-oriented helpers should
not reinterpret invalid UTF-8 as `&str`; use `to_str` only when explicit UTF-8
validation is desired.

For `bstr`-style lax-UTF-8 traversal, `chars` and `char_indices` decode using
the Unicode "substitution of maximal subparts" rule (matching
`String::from_utf8_lossy`), so each maximal invalid subsequence becomes one
`U+FFFD` while `char_indices` still reports the original byte range. `lines`
follows `str::lines` semantics on bytes (`\n` split, optional trailing `\r`
stripped, no terminators in the output) and `fields` splits on ASCII-whitespace
runs without emitting empty fields. All four preserve invalid bytes rather than
requiring valid UTF-8, which is the key difference from the `vstr` equivalents.

## `vencoding` API Shape

`kniferrs::vencoding` is implemented as an encoding-boundary facade for BOM
sniffing, UTF-8 validation, and lossy UTF-8 decoding. The MVP does not perform
general transcoding or add a default dependency on `encoding_rs`.

## Current Function Groups

Core names currently include `vbytes`, `vbytes::byte_len`, `vbytes::is_utf8`,
`vbytes::to_str`, `vbytes::sub`, `vbytes::trim_ascii`, `vbytes::find`,
`vbytes::find_all`, `vbytes::strip_prefix`, `vbytes::strip_suffix`,
`vbytes::replace_all`, `vbytes::chars`, `vbytes::char_indices`,
`vbytes::lines`, `vbytes::fields`, `vencoding`, `vencoding::Bom`,
`vencoding::detect_bom`, `vencoding::strip_bom`, `vencoding::validate_utf8`,
`vencoding::decode_utf8_lossy`,
`EmojiOptions`, `EmojiOptions::with_matcher`,
`MatchKind`, `VStrMatch`, `VStrMatcher`, `find_overlapping`,
`PatternError`, `contains_pattern`, `find_pattern`, `find_all_patterns`,
`replace_pattern`, `VRegex`, `graphemes`, `grapheme_len`, `take_graphemes`,
`truncate_graphemes`, `unicode_words`, `unicode_word_len`,
`unicode_word_indices`, `split_word_bounds`, `split_word_bound_indices`,
`unicode_sentences`, `unicode_sentence_len`, `split_sentence_bounds`,
`split_sentence_bound_indices`,
`display_width`, `take_width`, `truncate_width`, `wrap_width`,
`wrap_width_with_indent`, `wrap_width_with_options`,
`WrapOptions`, `WhitespaceMode`, `LongWordPolicy`, `wrap_with_options`,
`to_screaming_snake_case`, `to_dot_case`, `to_path_case`, `to_train_case`,
`to_cobol_case`, `to_sentence_case`, `capitalize`, `uncapitalize`,
`swap_case`, `normalize_whitespace`, `remove_whitespace`, `between`,
`contains_none`, `contains_any_ignore_case`, `find_any`, `count_matches`,
`find_all`, `find_all_ignore_case`, `replace_first`, `replace_last`,
`replace_ignore_case`, `replace_many`, `escape_regex`, `quote_meta`,
`split_once_last`, `strip_prefix_ignore_case`, `slugify_with_separator`,
`take_chars`, `drop_chars`, `normalize_newlines`, `trim_lines`,
`trim_blank_lines`, `abbreviate_middle`, `limit_words`, `excerpt`, `mask`,
`collapse_repeated_char`, `center`, `dedent`, `wrap_with_indent`,
`non_blank_lines`, `initials`, `is_palindrome`, `extract_digits`,
`remove_ascii_punctuation`, `surround`, `unsurround`, `word_count`,
`ant_path_match_with_separator`, `levenshtein_distance`,
`optimal_string_alignment`, `damerau_levenshtein_distance`, `jaro_similarity`,
`jaro_winkler_similarity`, `sorensen_dice`, `pluralize`, `singularize`,
`ordinalize`, `deordinalize`, `humanize`, `titleize`, `camelize`,
`number_format_with`, `number_format_float`, `human_count`, `transliterate`,
`slugify_ascii`, and `slugify_ascii_with_separator`.

Generic iterator APIs use forms such as `where I: IntoIterator`.

## Open Inventory Work

The local semver-aware check classifies removed APIs and changed signatures as
breaking, while newly exported APIs are additive inventory work.
`bin/check-release-api-semver.sh` layers `cargo-semver-checks check-release`
on top of the local check when a release baseline is configured with
`API_SEMVER_BASELINE_REF`, `API_SEMVER_BASELINE_ROOT`, or
`API_SEMVER_BASELINE_RUSTDOC`.

## Release Baseline Procedure

Before publishing the first `0.1.x` release, run the local inventory and semver
checks against the working tree:

```bash
bash bin/check-public-api-inventory.sh
bash bin/check-api-semver.sh
```

After the release is tagged, use that tag as the baseline for future release
reviews:

```bash
API_SEMVER_BASELINE_REF=v0.1.0 \
API_SEMVER_REQUIRED=true \
bash bin/check-release-api-semver.sh
```

For published crate comparisons, use `API_SEMVER_BASELINE_CRATES_IO=true`
instead of a local tag. Release branches should use exactly one baseline source
so `cargo-semver-checks` reports a single explainable compatibility decision.
