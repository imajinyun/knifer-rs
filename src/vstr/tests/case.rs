use crate::vstr::*;

#[test]
fn case_helpers_normalize_common_word_boundaries() {
    assert_eq!(to_snake_case("helloWorld ID"), "hello_world_id");
    assert_eq!(to_underline_case("helloWorld ID"), "hello_world_id");
    assert_eq!(to_kebab_case("helloWorld ID"), "hello-world-id");
    assert_eq!(to_camel_case("hello_world-id"), "helloWorldId");
    assert_eq!(to_pascal_case("hello_world-id"), "HelloWorldId");
    assert_eq!(to_camel_case("HelloWorld"), "helloWorld");
    assert_eq!(to_pascal_case("helloWorld"), "HelloWorld");
    assert_eq!(to_dot_case("helloWorld ID"), "hello.world.id");
    assert_eq!(to_path_case("helloWorld ID"), "hello/world/id");
    assert_eq!(to_snake_case("HTTPServerID"), "http_server_id");
    assert_eq!(to_kebab_case("HTTPServerID"), "http-server-id");
    assert_eq!(to_screaming_snake_case("HTTPServerID"), "HTTP_SERVER_ID");
    assert_eq!(to_screaming_kebab_case("HTTPServerID"), "HTTP-SERVER-ID");
    assert_eq!(to_train_case("HTTPServerID"), "Http-Server-Id");
    assert_eq!(to_cobol_case("HTTPServerID"), "HTTP-SERVER-ID");
    assert_eq!(to_title_case("hello_world-id"), "Hello World Id");
    assert_eq!(to_sentence_case("hello_world-ID"), "Hello world id");
    assert_eq!(
        to_snake_case("  hello--rust_world  "),
        "__hello__rust_world__"
    );
}

#[test]
fn case_helpers_handle_empty_and_unicode_words() {
    assert_eq!(to_snake_case(""), "");
    assert_eq!(to_kebab_case(""), "");
    assert_eq!(to_camel_case(""), "");
    assert_eq!(to_pascal_case(""), "");
    assert_eq!(to_snake_case("你好 Rust"), "你好_rust");
    assert_eq!(to_pascal_case("你好 rust"), "你好Rust");
    assert_eq!(capitalize("rUST"), "Rust");
    assert_eq!(capitalize("你好"), "你好");
    assert_eq!(uncapitalize("Rust"), "rust");
    assert_eq!(uncapitalize("HTTPServer"), "hTTPServer");
    assert_eq!(swap_case("Rust 你好"), "rUST 你好");
    assert_eq!(swap_case("Straße"), "sTRASSE");
}

struct CaseFixture {
    name: &'static str,
    input: &'static str,
    snake: &'static str,
    kebab: &'static str,
    dot: &'static str,
    path: &'static str,
    screaming_snake: &'static str,
    train: &'static str,
    title: &'static str,
    sentence: &'static str,
    camel: &'static str,
    pascal: &'static str,
}

fn case_conversion_fixtures() -> [CaseFixture; 6] {
    [
        CaseFixture {
            name: "acronym boundary",
            input: "XMLHttpRequest2",
            snake: "xml_http_request2",
            kebab: "xml-http-request2",
            dot: "xml.http.request2",
            path: "xml/http/request2",
            screaming_snake: "XML_HTTP_REQUEST2",
            train: "Xml-Http-Request2",
            title: "Xml Http Request2",
            sentence: "Xml http request2",
            camel: "xMLHttpRequest2",
            pascal: "XMLHttpRequest2",
        },
        CaseFixture {
            name: "numeric acronym suffix",
            input: "HTTPRequest2XX",
            snake: "http_request2_xx",
            kebab: "http-request2-xx",
            dot: "http.request2.xx",
            path: "http/request2/xx",
            screaming_snake: "HTTP_REQUEST2_XX",
            train: "Http-Request2-Xx",
            title: "Http Request2 Xx",
            sentence: "Http request2 xx",
            camel: "hTTPRequest2XX",
            pascal: "HTTPRequest2XX",
        },
        CaseFixture {
            name: "mixed separators and number",
            input: "http_server-id 42",
            snake: "http_server_id_42",
            kebab: "http-server-id-42",
            dot: "http.server.id.42",
            path: "http/server/id/42",
            screaming_snake: "HTTP_SERVER_ID_42",
            train: "Http-Server-Id-42",
            title: "Http Server Id 42",
            sentence: "Http server id 42",
            camel: "httpServerId42",
            pascal: "HttpServerId42",
        },
        CaseFixture {
            name: "repeated separators",
            input: "already__split--case  ",
            snake: "already__split__case__",
            kebab: "already--split--case--",
            dot: "already..split..case..",
            path: "already//split//case//",
            screaming_snake: "ALREADY__SPLIT__CASE__",
            train: "Already--Split--Case--",
            title: "Already Split Case",
            sentence: "Already  split  case  ",
            camel: "alreadySplitCase",
            pascal: "AlreadySplitCase",
        },
        CaseFixture {
            name: "unicode lowercase expansion",
            input: "StraßeHTTP",
            snake: "straße_http",
            kebab: "straße-http",
            dot: "straße.http",
            path: "straße/http",
            screaming_snake: "STRASSE_HTTP",
            train: "Straße-Http",
            title: "Straße Http",
            sentence: "Straße http",
            camel: "straßeHTTP",
            pascal: "StraßeHTTP",
        },
        CaseFixture {
            name: "cjk prefix",
            input: "你好_rust-world",
            snake: "你好_rust_world",
            kebab: "你好-rust-world",
            dot: "你好.rust.world",
            path: "你好/rust/world",
            screaming_snake: "你好_RUST_WORLD",
            train: "你好-Rust-World",
            title: "你好 Rust World",
            sentence: "你好 rust world",
            camel: "你好RustWorld",
            pascal: "你好RustWorld",
        },
    ]
}

fn assert_separated_case_fixture(fixture: &CaseFixture) {
    assert_eq!(
        to_snake_case(fixture.input),
        fixture.snake,
        "{}",
        fixture.name
    );
    assert_eq!(
        to_underline_case(fixture.input),
        fixture.snake,
        "{}",
        fixture.name
    );
    assert_eq!(
        to_kebab_case(fixture.input),
        fixture.kebab,
        "{}",
        fixture.name
    );
    assert_eq!(to_dot_case(fixture.input), fixture.dot, "{}", fixture.name);
    assert_eq!(
        to_path_case(fixture.input),
        fixture.path,
        "{}",
        fixture.name
    );
}

fn assert_styled_case_fixture(fixture: &CaseFixture) {
    assert_eq!(
        to_screaming_snake_case(fixture.input),
        fixture.screaming_snake,
        "{}",
        fixture.name
    );
    assert_eq!(
        to_screaming_kebab_case(fixture.input),
        fixture.screaming_snake.replace('_', "-"),
        "{}",
        fixture.name
    );
    assert_eq!(
        to_cobol_case(fixture.input),
        fixture.screaming_snake.replace('_', "-"),
        "{}",
        fixture.name
    );
    assert_eq!(
        to_train_case(fixture.input),
        fixture.train,
        "{}",
        fixture.name
    );
    assert_eq!(
        to_title_case(fixture.input),
        fixture.title,
        "{}",
        fixture.name
    );
    assert_eq!(
        to_sentence_case(fixture.input),
        fixture.sentence,
        "{}",
        fixture.name
    );
    assert_eq!(
        to_camel_case(fixture.input),
        fixture.camel,
        "{}",
        fixture.name
    );
    assert_eq!(
        to_pascal_case(fixture.input),
        fixture.pascal,
        "{}",
        fixture.name
    );
}

#[test]
fn case_conversion_cross_crate_fixtures_lock_acronym_number_separator_unicode() {
    for fixture in case_conversion_fixtures() {
        assert_separated_case_fixture(&fixture);
        assert_styled_case_fixture(&fixture);
    }

    assert_eq!(to_train_case("userID2FA"), "User-Id2-Fa");
    assert_eq!(to_screaming_snake_case("http2ServerID"), "HTTP2_SERVER_ID");
}

#[test]
fn knifer_go_vstr_golden_fixtures_cover_case_conversion() {
    assert_eq!(to_camel_case("hello_world"), "helloWorld");
    assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    assert_eq!(to_underline_case("HelloWorld"), "hello_world");
    assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
}
