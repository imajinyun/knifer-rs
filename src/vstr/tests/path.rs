use crate::vstr::*;

#[test]
fn knifer_go_vstr_golden_fixtures_cover_ant_path_matching() {
    assert!(ant_path_match("/api/**/users/?", "/api/v1/admin/users/a"));
    assert!(!ant_path_match("/api/*/users", "/api/v1/admin/users"));
    assert!(ant_path_match_with_separator(
        "foo.**.bar",
        "foo.a.b.bar",
        "."
    ));
}

#[test]
fn ant_path_match_supports_segment_wildcards() {
    assert!(ant_path_match("/api/**", "/api/v1/users"));
    assert!(ant_path_match("/api/?/users", "/api/v/users"));
    assert!(ant_path_match("/api/*", "/api/v1"));
    assert!(!ant_path_match("/api/*", "/api/v1/users"));
    assert!(ant_path_match_with_separator("a.**.d", "a.b.c.d", "."));
    assert!(!ant_path_match_with_separator("a.*.d", "a.b.c.d", "."));
    assert!(ant_path_match_with_separator("a.?.d", "a.b.d", "."));
    assert!(ant_path_match_with_separator("a/**", "a/b", ""));
}
