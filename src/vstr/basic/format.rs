/// Replaces `{}` placeholders with display arguments in order.
///
/// Use `\{` to escape a literal opening brace. Extra arguments are ignored and
/// placeholders without arguments are kept as `{}`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(
///     vstr::format("name={}, age={}", &[&"tom", &12]),
///     "name=tom, age=12"
/// );
/// assert_eq!(vstr::format(r"\{} {}", &[&"x"]), "{} x");
/// ```
#[must_use]
pub fn format(template: &str, args: &[&dyn std::fmt::Display]) -> String {
    if template.is_empty() || args.is_empty() {
        return template.to_owned();
    }

    let mut output = String::with_capacity(template.len());
    let mut arg_index = 0;
    let mut chars = template.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.peek() == Some(&'{') {
            output.push('{');
            chars.next();
            continue;
        }

        if ch == '{' && chars.peek() == Some(&'}') {
            chars.next();
            if let Some(arg) = args.get(arg_index) {
                output.push_str(&arg.to_string());
                arg_index += 1;
            } else {
                output.push_str("{}");
            }
            continue;
        }

        output.push(ch);
    }

    output
}

/// Renders `{name}` placeholders from a set of key/value pairs.
///
/// Placeholder syntax is intentionally small and dependency-free:
///
/// - `{name}` is replaced by the value whose key equals `name` exactly (no
///   surrounding whitespace is trimmed). The first matching pair wins.
/// - `{{` emits a literal `{` and `}}` emits a literal `}`.
/// - An unknown `{name}` is left verbatim (missing-key policy: keep the
///   placeholder), never panicking and never inserting an empty string.
/// - A `{` with no closing `}` is emitted verbatim along with the rest of the
///   template.
///
/// Values are inserted as-is, so escape untrusted values (for example with
/// [`escape_html`](crate::vstr::escape_html)) before rendering into markup.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(
///     vstr::render_template("Hi {name}, {count} new", [("name", "Tom"), ("count", "3")]),
///     "Hi Tom, 3 new"
/// );
/// // Escaped braces and a missing key are preserved.
/// assert_eq!(
///     vstr::render_template("{{literal}} {missing}", [("name", "Tom")]),
///     "{literal} {missing}"
/// );
/// ```
#[must_use]
pub fn render_template<'a, I>(template: &str, vars: I) -> String
where
    I: IntoIterator<Item = (&'a str, &'a str)>,
{
    let vars: Vec<(&str, &str)> = vars.into_iter().collect();
    let bytes = template.as_bytes();
    let mut output = String::with_capacity(template.len());
    let mut index = 0;

    while index < template.len() {
        match bytes[index] {
            b'{' => {
                if bytes.get(index + 1) == Some(&b'{') {
                    output.push('{');
                    index += 2;
                    continue;
                }

                // `{` is ASCII, so `index + 1` is a char boundary.
                if let Some(offset) = template[index + 1..].find('}') {
                    let name = &template[index + 1..index + 1 + offset];
                    let end = index + 1 + offset + 1;
                    match vars.iter().find(|(key, _)| *key == name) {
                        Some((_, value)) => output.push_str(value),
                        None => output.push_str(&template[index..end]),
                    }
                    index = end;
                    continue;
                }

                output.push_str(&template[index..]);
                break;
            }
            b'}' => {
                if bytes.get(index + 1) == Some(&b'}') {
                    output.push('}');
                    index += 2;
                } else {
                    output.push('}');
                    index += 1;
                }
            }
            _ => {
                let Some(ch) = template[index..].chars().next() else {
                    break;
                };
                output.push(ch);
                index += ch.len_utf8();
            }
        }
    }

    output
}
