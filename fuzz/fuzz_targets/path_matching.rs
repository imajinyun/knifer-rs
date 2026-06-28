use knifer_rs::vstr;

const SEEDS: &str = include_str!("../corpus/path_matching.txt");

fn main() {
    let paths = [
        "",
        "/",
        "/api/v1/users",
        "/api/v1/admin/users/a",
        "a/b/c",
        "你好/世界",
        "emoji/🚀/path",
        "a.b.c.d",
    ];

    for path in paths.into_iter().chain(SEEDS.lines()) {
        assert!(vstr::ant_path_match(path, path));
        assert!(vstr::ant_path_match("/**", path));
        assert!(vstr::ant_path_match("**", path));

        if !path.is_empty() && !path.contains('*') && !path.contains('?') {
            let pattern = format!("/{path}/**");
            assert!(vstr::ant_path_match(&pattern, &format!("/{path}/child")));
        }
    }

    let cases = [
        ("/api/**/users/?", "/api/v1/admin/users/a", true),
        ("/api/*/users", "/api/v1/admin/users", false),
        ("/api/?/users", "/api/v/users", true),
        ("a.**.d", "a.b.c.d", true),
        ("a.*.d", "a.b.c.d", true),
        ("a.?.d", "a.b.d", true),
        ("你好/**", "你好/世界/🚀", true),
    ];

    for (pattern, path, expected) in cases {
        assert_eq!(
            vstr::ant_path_match(pattern, path),
            expected,
            "pattern={pattern:?} path={path:?}"
        );
    }

    assert!(vstr::ant_path_match_with_separator(
        "a.**.d", "a.b.c.d", "."
    ));
    assert!(!vstr::ant_path_match_with_separator(
        "a.*.d", "a.b.c.d", "."
    ));
    assert!(vstr::ant_path_match_with_separator("a/**", "a/b", ""));
}
