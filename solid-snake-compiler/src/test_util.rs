use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct PlaceholderBindings {
    map: HashMap<String, String>,
    pattern: Regex,
}

impl PlaceholderBindings {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            pattern: Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\}\}").unwrap(),
        }
    }

    fn apply_to_expected(&mut self, expected: &str, actual: &str) -> Result<String, String> {
        let mut result = String::new();
        let mut last_index = 0;

        for caps in self.pattern.captures_iter(expected) {
            let whole = caps.get(0).unwrap();
            let key = caps.get(1).unwrap().as_str();

            result.push_str(&expected[last_index..whole.start()]);

            let expected_prefix = &expected[..whole.start()];
            let remaining_actual = &actual[result.len()..];

            let actual_token = extract_token(remaining_actual).ok_or_else(|| {
                format!("Could not extract token for placeholder `{{{{{key}}}}}`")
            })?;

            if let Some(prev) = self.map.get(key) {
                if prev != actual_token {
                    return Err(format!(
                        "Placeholder `{{{{{key}}}}}` previously bound to `{prev}`, but now saw `{actual_token}`"
                    ));
                }
            } else {
                self.map.insert(key.to_string(), actual_token.to_string());
            }

            result.push_str(actual_token);
            last_index = whole.end();
        }

        result.push_str(&expected[last_index..]);
        Ok(result)
    }
}

fn extract_token(text: &str) -> Option<&str> {
    let trimmed = text.trim_start();
    let end = trimmed
        .find(|c: char| c.is_whitespace() || ":=()[]{}".contains(c))
        .unwrap_or(trimmed.len());
    Some(&trimmed[..end])
}

pub fn match_lines(expected: &str, actual: &str) -> Result<(), String> {
    let mut bindings = PlaceholderBindings::new();

    for (line_no, (exp, act)) in expected.lines().zip(actual.lines()).enumerate() {
        let resolved = bindings.apply_to_expected(exp, act)?;
        if resolved != act {
            return Err(format!(
                "Mismatch on line {}:\nExpected: {}\nActual:   {}",
                line_no + 1,
                resolved,
                act
            ));
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! assert_ir {
    ($expected:expr, $actual:expr) => {{
        match $crate::test_util::match_lines($expected, $actual) {
            Ok(()) => {}
            Err(err) => panic!("IR output mismatch:\n{}", err),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_identical_lines() {
        let expected = "::temp1:Bool = false\njump if_1: if ::temp1:Bool";
        let actual = "::temp1:Bool = false\njump if_1: if ::temp1:Bool";
        assert!(match_lines(expected, actual).is_ok());
    }

    #[test]
    fn matches_with_placeholders() {
        let expected = "::{{ a }}:Bool = false\njump {{ label }}: if ::{{ a }}:Bool";
        let actual = "::temp42:Bool = false\njump if_7: if ::temp42:Bool";
        assert!(match_lines(expected, actual).is_ok());
    }

    #[test]
    fn fails_on_placeholder_conflict() {
        let expected = "::{{ a }}:Bool = false\njump {{ a }}: if ::{{ a }}:Bool";
        let actual = "::temp42:Bool = false\njump temp99: if ::temp42:Bool";
        let result = match_lines(expected, actual);
        assert!(result.is_err());
        assert!(
            result.clone().unwrap_err().contains("previously bound"),
            "Got error: {result:?}"
        );
    }

    #[test]
    fn fails_on_non_matching_lines() {
        let expected = "::{{ a }}:Bool = false\njump {{ label }}: if ::{{ a }}:Bool";
        let actual = "::temp42:Bool = false\ninvalid jump temp42";
        let result = match_lines(expected, actual);
        assert!(result.is_err(), "Expected a failure but got success");

        let err_msg = result.unwrap_err();
        println!("Error msg: {err_msg}");
        // Make sure it fails in general — optional: check for a keyword
        assert!(
            err_msg.contains("Mismatch")
                || err_msg.contains("Could not extract")
                || err_msg.contains("previously bound"),
            "Unexpected error message: {err_msg}"
        );
    }

    #[test]
    fn allows_multiple_placeholders() {
        let expected = "::{{ temp }}:Num = 1\n:{{ var }}:Num = ::{{ temp }}:Num + :{{ var }}:Num";
        let actual = "::t99:Num = 1\n:x:Num = ::t99:Num + :x:Num";
        assert!(match_lines(expected, actual).is_ok());
    }
}
