# Public API Inventory

This file tracks the public surface of `knifer_rs` facade modules. It is
checked by `bin/check-public-api-inventory.sh` and `bin/check-api-semver.sh`;
update the signature snapshot only after reviewing semver impact.

The full signature snapshot below is collected from the all-features API
surface. The same check also computes the default zero-runtime-dependency API
surface and verifies that the feature-gated delta matches the optional snapshot
in this file.

## API Stability Classes

`knifer-rs` is still pre-1.0, but public APIs are classified so callers know
which behavior is already treated as a strong contract.

### Core Stable Facade

The core stable facade is available in the default zero-runtime-dependency
build. For this class, signature changes here are treated as breaking unless
they are purely additive. Behavior is locked by unit tests, doctests, golden
fixtures, fuzz smoke, and project contract checks.

Core stable areas:

- `knifer_rs::vstr`: trim, blank/empty predicates, substring helpers, scalar
  text helpers, case conversion, replacement, literal search, escaping, Ant
  path matching, emoji helpers, and similarity helpers.
- `knifer_rs::vbytes`: byte length, UTF-8 validation, byte slicing, ASCII trim,
  byte search, prefix/suffix stripping, and byte replacement.
- `knifer_rs::vencoding`: BOM detection, BOM stripping, UTF-8 validation, and
  lossy UTF-8 decoding boundaries.

### Optional Feature Facade

The optional feature facade is public only when its feature is enabled. These APIs are absent from the default build and are listed in the optional signature snapshot below. Once enabled, their signatures are checked with the same semver-aware inventory gates as the all-features surface.

Optional feature areas:

- `pattern-regex`: regex-backed `contains_pattern`, `find_pattern`,
  `find_all_patterns`, `replace_pattern`, and `PatternError`.
- `unicode-segmentation`: grapheme, word, and sentence boundary helpers.
- `unicode-width`: display-cell width, width truncation, and width wrapping.

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
knifer_rs::vbytes = pub mod vbytes
knifer_rs::vbytes::byte_len = pub const fn byte_len(input: &[u8]) -> usize
knifer_rs::vbytes::contains = pub fn contains(input: &[u8], needle: &[u8]) -> bool
knifer_rs::vbytes::find = pub fn find(input: &[u8], needle: &[u8]) -> Option<(usize, usize)>
knifer_rs::vbytes::find_all = pub fn find_all(input: &[u8], needle: &[u8]) -> Vec<(usize, usize)>
knifer_rs::vbytes::is_empty = pub const fn is_empty(input: &[u8]) -> bool
knifer_rs::vbytes::is_utf8 = pub const fn is_utf8(input: &[u8]) -> bool
knifer_rs::vbytes::replace_all = pub fn replace_all(input: &[u8], from: &[u8], to: &[u8]) -> Vec<u8>
knifer_rs::vbytes::strip_prefix = pub fn strip_prefix<'src>(input: &'src [u8], prefix: &[u8]) -> Option<&'src [u8]>
knifer_rs::vbytes::strip_suffix = pub fn strip_suffix<'src>(input: &'src [u8], suffix: &[u8]) -> Option<&'src [u8]>
knifer_rs::vbytes::sub = pub fn sub(input: &[u8], from_index: isize, to_index: isize) -> &[u8]
knifer_rs::vbytes::to_str = pub const fn to_str(input: &[u8]) -> Result<&str, core::str::Utf8Error>
knifer_rs::vbytes::trim_ascii = pub fn trim_ascii(input: &[u8]) -> &[u8]
knifer_rs::vbytes::trim_ascii_end = pub fn trim_ascii_end(input: &[u8]) -> &[u8]
knifer_rs::vbytes::trim_ascii_start = pub fn trim_ascii_start(input: &[u8]) -> &[u8]
knifer_rs::vencoding = pub mod vencoding
knifer_rs::vencoding::Bom = pub enum Bom
knifer_rs::vencoding::Bom::byte_len = pub const fn byte_len(self) -> usize
knifer_rs::vencoding::Bom::encoding_name = pub const fn encoding_name(self) -> &'static str
knifer_rs::vencoding::BomScan = pub struct BomScan<'src>
knifer_rs::vencoding::decode_utf8_lossy = pub fn decode_utf8_lossy(input: &[u8]) -> Cow<'_, str>
knifer_rs::vencoding::decode_utf8_lossy_without_bom = pub fn decode_utf8_lossy_without_bom(input: &[u8]) -> Cow<'_, str>
knifer_rs::vencoding::detect_bom = pub const fn detect_bom(input: &[u8]) -> Option<Bom>
knifer_rs::vencoding::is_utf8 = pub const fn is_utf8(input: &[u8]) -> bool
knifer_rs::vencoding::scan_bom = pub fn scan_bom(input: &[u8]) -> BomScan<'_>
knifer_rs::vencoding::strip_bom = pub fn strip_bom(input: &[u8]) -> &[u8]
knifer_rs::vencoding::validate_utf8 = pub const fn validate_utf8(input: &[u8]) -> Result<&str, core::str::Utf8Error>
knifer_rs::vencoding::validate_utf8_without_bom = pub fn validate_utf8_without_bom(input: &[u8]) -> Result<&str, core::str::Utf8Error>
knifer_rs::vstr = pub mod vstr
knifer_rs::vstr::EmojiOptions = pub struct EmojiOptions<'src>
knifer_rs::vstr::EmojiOptions::new = pub const fn new() -> Self
knifer_rs::vstr::EmojiOptions::with_matcher = pub fn with_matcher(mut self, matcher: impl Fn(&str) -> bool + 'src) -> Self
knifer_rs::vstr::EmojiOptions::with_replacer = pub fn with_replacer(mut self, replacer: impl Fn(&str) -> String + 'src) -> Self
knifer_rs::vstr::LongWordPolicy = pub enum LongWordPolicy
knifer_rs::vstr::MatchKind = pub enum MatchKind
knifer_rs::vstr::PatternError = pub struct PatternError
knifer_rs::vstr::PatternError::message = pub fn message(&self) -> &str
knifer_rs::vstr::PatternError::pattern = pub fn pattern(&self) -> &str
knifer_rs::vstr::VStrMatch = pub struct VStrMatch<'needle>
knifer_rs::vstr::VStrMatcher = pub struct VStrMatcher<'needle>
knifer_rs::vstr::VStrMatcher::find = pub fn find(&self, input: &str) -> Option<VStrMatch<'needle>>
knifer_rs::vstr::VStrMatcher::find_all = pub fn find_all(&self, input: &str) -> Vec<VStrMatch<'needle>>
knifer_rs::vstr::VStrMatcher::find_overlapping = pub fn find_overlapping(&self, input: &str) -> Vec<VStrMatch<'needle>>
knifer_rs::vstr::VStrMatcher::is_empty = pub fn is_empty(&self) -> bool
knifer_rs::vstr::VStrMatcher::kind = pub const fn kind(&self) -> MatchKind
knifer_rs::vstr::VStrMatcher::len = pub fn len(&self) -> usize
knifer_rs::vstr::VStrMatcher::new = pub fn new<I>(needles: I) -> Self where I: IntoIterator<Item = &'needle str>
knifer_rs::vstr::VStrMatcher::replace_all = pub fn replace_all<'replacement, I>(&self, input: &str, replacements: I) -> String where I: IntoIterator<Item = &'replacement str>
knifer_rs::vstr::VStrMatcher::with_kind = pub fn with_kind<I>(needles: I, kind: MatchKind) -> Self where I: IntoIterator<Item = &'needle str>
knifer_rs::vstr::WhitespaceMode = pub enum WhitespaceMode
knifer_rs::vstr::WrapOptions = pub struct WrapOptions<'src>
knifer_rs::vstr::WrapOptions::new = pub const fn new(width: usize) -> Self
knifer_rs::vstr::WrapOptions::with_indent = pub const fn with_indent(mut self, initial_indent: &'src str, subsequent_indent: &'src str) -> Self
knifer_rs::vstr::WrapOptions::with_long_word_policy = pub const fn with_long_word_policy(mut self, long_word_policy: LongWordPolicy) -> Self
knifer_rs::vstr::WrapOptions::with_whitespace_mode = pub const fn with_whitespace_mode(mut self, whitespace_mode: WhitespaceMode) -> Self
knifer_rs::vstr::WrapOptions::with_word_separators = pub const fn with_word_separators(mut self, word_separators: &'src [char]) -> Self
knifer_rs::vstr::abbreviate_middle = pub fn abbreviate_middle(input: &str, max_chars: usize, marker: &str) -> String
knifer_rs::vstr::add_prefix_if_not = pub fn add_prefix_if_not(input: &str, prefix: &str) -> String
knifer_rs::vstr::add_suffix_if_not = pub fn add_suffix_if_not(input: &str, suffix: &str) -> String
knifer_rs::vstr::after = pub fn after<'src>(input: &'src str, separator: &str) -> &'src str
knifer_rs::vstr::after_last = pub fn after_last<'src>(input: &'src str, separator: &str) -> &'src str
knifer_rs::vstr::ant_path_match = pub fn ant_path_match(pattern: &str, path: &str) -> bool
knifer_rs::vstr::ant_path_match_with_separator = pub fn ant_path_match_with_separator(pattern: &str, path: &str, separator: &str) -> bool
knifer_rs::vstr::before = pub fn before<'src>(input: &'src str, separator: &str) -> &'src str
knifer_rs::vstr::before_last = pub fn before_last<'src>(input: &'src str, separator: &str) -> &'src str
knifer_rs::vstr::between = pub fn between<'src>(input: &'src str, start: &str, end: &str) -> Option<&'src str>
knifer_rs::vstr::byte_len = pub const fn byte_len(input: &str) -> usize
knifer_rs::vstr::capitalize = pub fn capitalize(input: &str) -> String
knifer_rs::vstr::center = pub fn center(input: &str, width: usize, pad: char) -> String
knifer_rs::vstr::char_len = pub fn char_len(input: &str) -> usize
knifer_rs::vstr::chars = pub fn chars(input: &str) -> Vec<char>
knifer_rs::vstr::collapse_repeated_char = pub fn collapse_repeated_char(input: &str, ch: char) -> String
knifer_rs::vstr::contains = pub fn contains(input: &str, needle: &str) -> bool
knifer_rs::vstr::contains_all = pub fn contains_all<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::contains_all_ignore_case = pub fn contains_all_ignore_case<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::contains_any = pub fn contains_any<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::contains_any_ignore_case = pub fn contains_any_ignore_case<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::contains_emoji = pub fn contains_emoji(input: &str) -> bool
knifer_rs::vstr::contains_emoji_with_options = pub fn contains_emoji_with_options(input: &str, options: &EmojiOptions<'_>) -> bool
knifer_rs::vstr::contains_ignore_case = pub fn contains_ignore_case(input: &str, needle: &str) -> bool
knifer_rs::vstr::contains_none = pub fn contains_none<'src, I>(input: &str, needles: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::contains_pattern = pub fn contains_pattern(input: &str, pattern: &str) -> Result<bool, PatternError>
knifer_rs::vstr::count_matches = pub fn count_matches(input: &str, needle: &str) -> usize
knifer_rs::vstr::dedent = pub fn dedent(input: &str) -> String
knifer_rs::vstr::default_if_blank = pub fn default_if_blank<'src>(input: &'src str, default: &'src str) -> &'src str
knifer_rs::vstr::default_if_empty = pub fn default_if_empty<'src>(input: &'src str, default: &'src str) -> &'src str
knifer_rs::vstr::display_width = pub fn display_width(input: &str) -> usize
knifer_rs::vstr::drop_chars = pub fn drop_chars(input: &str, count: usize) -> &str
knifer_rs::vstr::ends_with = pub fn ends_with(input: &str, suffix: &str) -> bool
knifer_rs::vstr::ends_with_ignore_case = pub fn ends_with_ignore_case(input: &str, suffix: &str) -> bool
knifer_rs::vstr::equals_ignore_case = pub fn equals_ignore_case(left: &str, right: &str) -> bool
knifer_rs::vstr::escape_html = pub fn escape_html(input: &str) -> String
knifer_rs::vstr::escape_regex = pub fn escape_regex(input: &str) -> String
knifer_rs::vstr::escape_unicode = pub fn escape_unicode(input: &str) -> String
knifer_rs::vstr::excerpt = pub fn excerpt(input: &str, needle: &str, max_chars: usize, marker: &str) -> String
knifer_rs::vstr::extract_digits = pub fn extract_digits(input: &str) -> String
knifer_rs::vstr::find_all = pub fn find_all(input: &str, needle: &str) -> Vec<(usize, usize)>
knifer_rs::vstr::find_all_ignore_case = pub fn find_all_ignore_case(input: &str, needle: &str) -> Vec<(usize, usize)>
knifer_rs::vstr::find_all_patterns = pub fn find_all_patterns(input: &str, pattern: &str) -> Result<Vec<(usize, usize)>, PatternError>
knifer_rs::vstr::find_any = pub fn find_any<'needle, I>(input: &str, needles: I) -> Option<(&'needle str, usize, usize)> where I: IntoIterator<Item = &'needle str>
knifer_rs::vstr::find_pattern = pub fn find_pattern(input: &str, pattern: &str) -> Result<Option<(usize, usize)>, PatternError>
knifer_rs::vstr::format = pub fn format(template: &str, args: &[&dyn std::fmt::Display]) -> String
knifer_rs::vstr::grapheme_len = pub fn grapheme_len(input: &str) -> usize
knifer_rs::vstr::graphemes = pub fn graphemes(input: &str) -> Vec<&str>
knifer_rs::vstr::hamming_distance64 = pub const fn hamming_distance64(left: u64, right: u64) -> u32
knifer_rs::vstr::has_blank = pub fn has_blank<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::has_empty = pub fn has_empty<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::indent = pub fn indent(input: &str, prefix: &str) -> String
knifer_rs::vstr::initials = pub fn initials(input: &str) -> String
knifer_rs::vstr::is_all_blank = pub fn is_all_blank<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::is_all_empty = pub fn is_all_empty<'src, I>(values: I) -> bool where I: IntoIterator<Item = &'src str>
knifer_rs::vstr::is_ascii = pub const fn is_ascii(ch: char) -> bool
knifer_rs::vstr::is_blank = pub fn is_blank(input: &str) -> bool
knifer_rs::vstr::is_blank_char = pub fn is_blank_char(ch: char) -> bool
knifer_rs::vstr::is_digit = pub fn is_digit(ch: char) -> bool
knifer_rs::vstr::is_empty = pub const fn is_empty(input: &str) -> bool
knifer_rs::vstr::is_letter = pub fn is_letter(ch: char) -> bool
knifer_rs::vstr::is_letter_or_digit = pub fn is_letter_or_digit(ch: char) -> bool
knifer_rs::vstr::is_not_blank = pub fn is_not_blank(input: &str) -> bool
knifer_rs::vstr::is_not_empty = pub const fn is_not_empty(input: &str) -> bool
knifer_rs::vstr::is_palindrome = pub fn is_palindrome(input: &str) -> bool
knifer_rs::vstr::jaccard_similarity = pub fn jaccard_similarity(left: &str, right: &str) -> f64
knifer_rs::vstr::length = pub fn length(input: &str) -> usize
knifer_rs::vstr::levenshtein_distance = pub fn levenshtein_distance(left: &str, right: &str) -> usize
knifer_rs::vstr::levenshtein_similarity = pub fn levenshtein_similarity(left: &str, right: &str) -> f64
knifer_rs::vstr::limit_words = pub fn limit_words(input: &str, max_words: usize, suffix: &str) -> String
knifer_rs::vstr::line_count = pub fn line_count(input: &str) -> usize
knifer_rs::vstr::lines = pub fn lines(input: &str) -> Vec<&str>
knifer_rs::vstr::mask = pub fn mask(input: &str, visible_start: usize, visible_end: usize, mask: char) -> String
knifer_rs::vstr::ngram_similarity = pub fn ngram_similarity(left: &str, right: &str, n: usize) -> f64
knifer_rs::vstr::non_blank_lines = pub fn non_blank_lines(input: &str) -> Vec<&str>
knifer_rs::vstr::normalize_newlines = pub fn normalize_newlines(input: &str) -> String
knifer_rs::vstr::normalize_whitespace = pub fn normalize_whitespace(input: &str) -> String
knifer_rs::vstr::pad_left = pub fn pad_left(input: &str, target_len: usize, pad: char) -> String
knifer_rs::vstr::pad_right = pub fn pad_right(input: &str, target_len: usize, pad: char) -> String
knifer_rs::vstr::quote_meta = pub fn quote_meta(input: &str) -> String
knifer_rs::vstr::remove_ascii_punctuation = pub fn remove_ascii_punctuation(input: &str) -> String
knifer_rs::vstr::remove_emoji = pub fn remove_emoji(input: &str) -> String
knifer_rs::vstr::remove_emoji_with_options = pub fn remove_emoji_with_options(input: &str, options: &EmojiOptions<'_>) -> String
knifer_rs::vstr::remove_prefix = pub fn remove_prefix<'src>(input: &'src str, prefix: &str) -> &'src str
knifer_rs::vstr::remove_suffix = pub fn remove_suffix<'src>(input: &'src str, suffix: &str) -> &'src str
knifer_rs::vstr::remove_whitespace = pub fn remove_whitespace(input: &str) -> String
knifer_rs::vstr::repeat = pub fn repeat(input: &str, count: usize) -> String
knifer_rs::vstr::replace_first = pub fn replace_first(input: &str, from: &str, to: &str) -> String
knifer_rs::vstr::replace_ignore_case = pub fn replace_ignore_case(input: &str, from: &str, to: &str) -> String
knifer_rs::vstr::replace_last = pub fn replace_last(input: &str, from: &str, to: &str) -> String
knifer_rs::vstr::replace_many = pub fn replace_many<'src, I>(input: &str, replacements: I) -> String where I: IntoIterator<Item = (&'src str, &'src str)>
knifer_rs::vstr::replace_pattern = pub fn replace_pattern(input: &str, pattern: &str, replacement: &str) -> Result<String, PatternError>
knifer_rs::vstr::reverse = pub fn reverse(input: &str) -> String
knifer_rs::vstr::rune_len = pub fn rune_len(input: &str) -> usize
knifer_rs::vstr::sim_hash = pub fn sim_hash(input: &str) -> u64
knifer_rs::vstr::slugify = pub fn slugify(input: &str) -> String
knifer_rs::vstr::slugify_with_separator = pub fn slugify_with_separator(input: &str, separator: char) -> String
knifer_rs::vstr::split = pub fn split<'src>(input: &'src str, separator: &str) -> Vec<&'src str>
knifer_rs::vstr::split_once = pub fn split_once<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)>
knifer_rs::vstr::split_once_last = pub fn split_once_last<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)>
knifer_rs::vstr::split_sentence_bound_indices = pub fn split_sentence_bound_indices(input: &str) -> Vec<(usize, &str)>
knifer_rs::vstr::split_sentence_bounds = pub fn split_sentence_bounds(input: &str) -> Vec<&str>
knifer_rs::vstr::split_trim = pub fn split_trim<'src>(input: &'src str, separator: &str) -> Vec<&'src str>
knifer_rs::vstr::split_word_bound_indices = pub fn split_word_bound_indices(input: &str) -> Vec<(usize, &str)>
knifer_rs::vstr::split_word_bounds = pub fn split_word_bounds(input: &str) -> Vec<&str>
knifer_rs::vstr::starts_with = pub fn starts_with(input: &str, prefix: &str) -> bool
knifer_rs::vstr::starts_with_ignore_case = pub fn starts_with_ignore_case(input: &str, prefix: &str) -> bool
knifer_rs::vstr::strip_prefix_ignore_case = pub fn strip_prefix_ignore_case<'src>(input: &'src str, prefix: &str) -> Option<&'src str>
knifer_rs::vstr::strip_suffix_ignore_case = pub fn strip_suffix_ignore_case<'src>(input: &'src str, suffix: &str) -> Option<&'src str>
knifer_rs::vstr::sub = pub fn sub(input: &str, from_index: isize, to_index: isize) -> String
knifer_rs::vstr::sub_after = pub fn sub_after<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str
knifer_rs::vstr::sub_before = pub fn sub_before<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str
knifer_rs::vstr::surround = pub fn surround(input: &str, left: &str, right: &str) -> String
knifer_rs::vstr::swap_case = pub fn swap_case(input: &str) -> String
knifer_rs::vstr::take_chars = pub fn take_chars(input: &str, count: usize) -> &str
knifer_rs::vstr::take_graphemes = pub fn take_graphemes(input: &str, count: usize) -> &str
knifer_rs::vstr::take_width = pub fn take_width(input: &str, max_width: usize) -> &str
knifer_rs::vstr::to_camel_case = pub fn to_camel_case(input: &str) -> String
knifer_rs::vstr::to_cobol_case = pub fn to_cobol_case(input: &str) -> String
knifer_rs::vstr::to_dot_case = pub fn to_dot_case(input: &str) -> String
knifer_rs::vstr::to_kebab_case = pub fn to_kebab_case(input: &str) -> String
knifer_rs::vstr::to_pascal_case = pub fn to_pascal_case(input: &str) -> String
knifer_rs::vstr::to_path_case = pub fn to_path_case(input: &str) -> String
knifer_rs::vstr::to_screaming_kebab_case = pub fn to_screaming_kebab_case(input: &str) -> String
knifer_rs::vstr::to_screaming_snake_case = pub fn to_screaming_snake_case(input: &str) -> String
knifer_rs::vstr::to_sentence_case = pub fn to_sentence_case(input: &str) -> String
knifer_rs::vstr::to_snake_case = pub fn to_snake_case(input: &str) -> String
knifer_rs::vstr::to_title_case = pub fn to_title_case(input: &str) -> String
knifer_rs::vstr::to_train_case = pub fn to_train_case(input: &str) -> String
knifer_rs::vstr::to_underline_case = pub fn to_underline_case(input: &str) -> String
knifer_rs::vstr::trim = pub fn trim(input: &str) -> &str
knifer_rs::vstr::trim_blank_lines = pub fn trim_blank_lines(input: &str) -> &str
knifer_rs::vstr::trim_end = pub fn trim_end(input: &str) -> &str
knifer_rs::vstr::trim_lines = pub fn trim_lines(input: &str) -> String
knifer_rs::vstr::trim_start = pub fn trim_start(input: &str) -> &str
knifer_rs::vstr::trim_to_empty = pub fn trim_to_empty(input: &str) -> &str
knifer_rs::vstr::trim_to_string = pub fn trim_to_string(input: &str) -> String
knifer_rs::vstr::truncate = pub fn truncate(input: &str, max_chars: usize) -> &str
knifer_rs::vstr::truncate_graphemes = pub fn truncate_graphemes(input: &str, max_graphemes: usize, suffix: &str) -> String
knifer_rs::vstr::truncate_width = pub fn truncate_width(input: &str, max_width: usize, suffix: &str) -> String
knifer_rs::vstr::truncate_with_suffix = pub fn truncate_with_suffix(input: &str, max_chars: usize, suffix: &str) -> String
knifer_rs::vstr::uncapitalize = pub fn uncapitalize(input: &str) -> String
knifer_rs::vstr::unescape_html = pub fn unescape_html(input: &str) -> String
knifer_rs::vstr::unescape_unicode = pub fn unescape_unicode(input: &str) -> String
knifer_rs::vstr::unicode_sentence_len = pub fn unicode_sentence_len(input: &str) -> usize
knifer_rs::vstr::unicode_sentences = pub fn unicode_sentences(input: &str) -> Vec<&str>
knifer_rs::vstr::unicode_word_indices = pub fn unicode_word_indices(input: &str) -> Vec<(usize, &str)>
knifer_rs::vstr::unicode_word_len = pub fn unicode_word_len(input: &str) -> usize
knifer_rs::vstr::unicode_words = pub fn unicode_words(input: &str) -> Vec<&str>
knifer_rs::vstr::unsurround = pub fn unsurround<'src>(input: &'src str, left: &str, right: &str) -> Option<&'src str>
knifer_rs::vstr::with_emoji_matcher = pub fn with_emoji_matcher<'src>(matcher: impl Fn(&str) -> bool + 'src) -> EmojiOptions<'src>
knifer_rs::vstr::with_emoji_replacer = pub fn with_emoji_replacer<'src>(replacer: impl Fn(&str) -> String + 'src) -> EmojiOptions<'src>
knifer_rs::vstr::word_count = pub fn word_count(input: &str) -> usize
knifer_rs::vstr::words = pub fn words(input: &str) -> Vec<&str>
knifer_rs::vstr::wrap = pub fn wrap(input: &str, width: usize) -> String
knifer_rs::vstr::wrap_width = pub fn wrap_width(input: &str, width: usize) -> String
knifer_rs::vstr::wrap_width_with_indent = pub fn wrap_width_with_indent(input: &str, width: usize, initial_indent: &str, subsequent_indent: &str) -> String
knifer_rs::vstr::wrap_width_with_options = pub fn wrap_width_with_options(input: &str, options: &WrapOptions<'_>) -> String
knifer_rs::vstr::wrap_with_indent = pub fn wrap_with_indent(input: &str, width: usize, initial_indent: &str, subsequent_indent: &str) -> String
knifer_rs::vstr::wrap_with_options = pub fn wrap_with_options(input: &str, options: &WrapOptions<'_>) -> String
<!-- public-api-signatures:end -->

## Optional Feature Inventory

- `pattern-regex`
- `unicode-segmentation`
- `unicode-width`

The default build does not expose the following signatures. These APIs appear
only when the matching optional feature is enabled, while the all-features
snapshot keeps their release signature stable.

<!-- public-api-optional-signatures:start -->
knifer_rs::vstr::PatternError = pub struct PatternError
knifer_rs::vstr::PatternError::message = pub fn message(&self) -> &str
knifer_rs::vstr::PatternError::pattern = pub fn pattern(&self) -> &str
knifer_rs::vstr::contains_pattern = pub fn contains_pattern(input: &str, pattern: &str) -> Result<bool, PatternError>
knifer_rs::vstr::display_width = pub fn display_width(input: &str) -> usize
knifer_rs::vstr::find_all_patterns = pub fn find_all_patterns(input: &str, pattern: &str) -> Result<Vec<(usize, usize)>, PatternError>
knifer_rs::vstr::find_pattern = pub fn find_pattern(input: &str, pattern: &str) -> Result<Option<(usize, usize)>, PatternError>
knifer_rs::vstr::grapheme_len = pub fn grapheme_len(input: &str) -> usize
knifer_rs::vstr::graphemes = pub fn graphemes(input: &str) -> Vec<&str>
knifer_rs::vstr::replace_pattern = pub fn replace_pattern(input: &str, pattern: &str, replacement: &str) -> Result<String, PatternError>
knifer_rs::vstr::split_sentence_bound_indices = pub fn split_sentence_bound_indices(input: &str) -> Vec<(usize, &str)>
knifer_rs::vstr::split_sentence_bounds = pub fn split_sentence_bounds(input: &str) -> Vec<&str>
knifer_rs::vstr::split_word_bound_indices = pub fn split_word_bound_indices(input: &str) -> Vec<(usize, &str)>
knifer_rs::vstr::split_word_bounds = pub fn split_word_bounds(input: &str) -> Vec<&str>
knifer_rs::vstr::take_graphemes = pub fn take_graphemes(input: &str, count: usize) -> &str
knifer_rs::vstr::take_width = pub fn take_width(input: &str, max_width: usize) -> &str
knifer_rs::vstr::truncate_graphemes = pub fn truncate_graphemes(input: &str, max_graphemes: usize, suffix: &str) -> String
knifer_rs::vstr::truncate_width = pub fn truncate_width(input: &str, max_width: usize, suffix: &str) -> String
knifer_rs::vstr::unicode_sentence_len = pub fn unicode_sentence_len(input: &str) -> usize
knifer_rs::vstr::unicode_sentences = pub fn unicode_sentences(input: &str) -> Vec<&str>
knifer_rs::vstr::unicode_word_indices = pub fn unicode_word_indices(input: &str) -> Vec<(usize, &str)>
knifer_rs::vstr::unicode_word_len = pub fn unicode_word_len(input: &str) -> usize
knifer_rs::vstr::unicode_words = pub fn unicode_words(input: &str) -> Vec<&str>
knifer_rs::vstr::wrap_width = pub fn wrap_width(input: &str, width: usize) -> String
knifer_rs::vstr::wrap_width_with_indent = pub fn wrap_width_with_indent(input: &str, width: usize, initial_indent: &str, subsequent_indent: &str) -> String
knifer_rs::vstr::wrap_width_with_options = pub fn wrap_width_with_options(input: &str, options: &WrapOptions<'_>) -> String
<!-- public-api-optional-signatures:end -->

## Facade Boundaries

Byte and encoding helpers stay separate from `knifer_rs::vstr`.

## `vbytes` API Shape

`knifer_rs::vbytes` is implemented as a byte-slice facade for data that may not
be valid UTF-8. All byte ranges are byte offsets. Byte-oriented helpers should
not reinterpret invalid UTF-8 as `&str`; use `to_str` only when explicit UTF-8
validation is desired.

## `vencoding` API Shape

`knifer_rs::vencoding` is implemented as an encoding-boundary facade for BOM
sniffing, UTF-8 validation, and lossy UTF-8 decoding. The MVP does not perform
general transcoding or add a default dependency on `encoding_rs`.

## Current Function Groups

Core names currently include `vbytes`, `vbytes::byte_len`, `vbytes::is_utf8`,
`vbytes::to_str`, `vbytes::sub`, `vbytes::trim_ascii`, `vbytes::find`,
`vbytes::find_all`, `vbytes::strip_prefix`, `vbytes::strip_suffix`,
`vbytes::replace_all`, `vencoding`, `vencoding::Bom`,
`vencoding::detect_bom`, `vencoding::strip_bom`, `vencoding::validate_utf8`,
`vencoding::decode_utf8_lossy`,
`EmojiOptions`, `EmojiOptions::with_matcher`,
`MatchKind`, `VStrMatch`, `VStrMatcher`, `find_overlapping`,
`PatternError`, `contains_pattern`, `find_pattern`, `find_all_patterns`,
`replace_pattern`, `graphemes`, `grapheme_len`, `take_graphemes`,
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
`ant_path_match_with_separator`, and `levenshtein_distance`.

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
