/// Replaces `{}` placeholders with display arguments in order.
///
/// Use `\{` to escape a literal opening brace. Extra arguments are ignored and
/// placeholders without arguments are kept as `{}`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
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
