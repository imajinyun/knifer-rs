# `vstr` API Parity

`vstr` follows the practical shape of `knifer-go` while using Rust-native
borrowing, ownership, and UTF-8 guarantees.

## Golden Fixtures

Golden fixtures are captured as Rust unit tests so behavior remains executable:

- `knifer_go_vstr_golden_fixtures_cover_case_conversion`
- `case_conversion_cross_crate_fixtures_lock_acronym_number_separator_unicode`
- `knifer_go_vstr_golden_fixtures_cover_unicode_escape`
- `knifer_go_vstr_golden_fixtures_cover_ant_path_matching`
- `knifer_go_vstr_golden_fixtures_cover_similarity`
- `commons_string_utils_classics_lock_cross_crate_shape`

The case conversion matrix locks acronym boundaries, number boundaries,
repeated separators, Unicode lowercase expansion, and CJK prefix behavior
across snake, kebab, dot, path, screaming, train, title, sentence, camel, and
Pascal shapes.

The StringUtils classics fixture tracks Apache Commons Lang `StringUtils`
behavior for `common_prefix`, `difference`, `rotate`, and `wrap_if_missing`
(all `covered`). `common_suffix` and the ignore-case
`add_prefix_if_not_ignore_case` / `add_suffix_if_not_ignore_case` helpers are
`covered-rust-shape` because Commons exposes only the case-sensitive prefix
side.

## Compatibility Notes

- `trim_to_empty` is an alias for `trim`; Rust `&str` cannot be null.
- `sub`, `take_chars`, and `drop_chars` are scalar-safe and never split UTF-8
  byte sequences.
- Case conversion intentionally follows a daily-business utility shape rather
  than trying to expose every behavior from `heck` or `convert-case`.
- Ant path helpers are route/path utilities, not filesystem canonicalization.

## Open Compatibility Work

1. Keep comparing edge cases with `knifer-go/vstr` before stabilizing 1.0.
2. Mark differences as `covered`, `covered-rust-shape`, or
   `intentional-diff`.
3. Expand fixtures whenever a public helper is added.
