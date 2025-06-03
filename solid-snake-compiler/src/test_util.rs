use regex::Regex;
use std::collections::HashMap;

use similar::{Algorithm, ChangeTag, TextDiff};

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

const ACTUAL_SIGN: &str = "-";
const EXPECTED_SIGN: &str = "+";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffMode {
    Word,
    Char,
}

pub fn compare_texts(actual: &str, expected: &str) -> bool {
    let expected = expected.replace('\r', "");
    let actual = actual.replace('\r', "");

    if expected == actual {
        return true;
    }

    let diff = TextDiff::configure()
        .algorithm(Algorithm::Patience)
        .diff_lines(&actual, &expected);

    println!("{:-^80}", "");
    println!(
        "{} / {}:\n",
        console::Style::new().color256(88).apply_to("Actual"),
        console::Style::new().green().apply_to("Expected")
    );

    let red = console::Style::new().color256(88);
    let green = console::Style::new().green();
    let dim = console::Style::new().dim();

    let red_fg_bg = console::Style::new().color256(52).on_color256(88);
    let green_fg_bg = console::Style::new()
        .color256(22)
        .on_color256(22)
        .on_green();

    let actual_max_digits = actual.lines().count().to_string().len();
    let expected_max_digits = expected.lines().count().to_string().len();
    let max_digits = std::cmp::max(actual_max_digits, expected_max_digits);

    for group in diff.grouped_ops(3) {
        for op in group {
            for change in diff.iter_inline_changes(&op) {
                let actual_index: Option<usize> = change.old_index();
                let expected_index: Option<usize> = change.new_index();

                let left_line_number = actual_index
                    .map(|i| format!("{:0>width$}", i + 1, width = max_digits))
                    .unwrap_or_else(|| " ".repeat(max_digits));
                let right_line_number = expected_index
                    .map(|i| format!("{:0>width$}", i + 1, width = max_digits))
                    .unwrap_or_else(|| " ".repeat(max_digits));

                let tag = change.tag();
                let (sign, style) = match tag {
                    ChangeTag::Delete => ("-", &red),
                    ChangeTag::Insert => ("+", &green),
                    ChangeTag::Equal => (" ", &dim),
                };
                let emphasized_style = match tag {
                    ChangeTag::Delete => &red_fg_bg,
                    ChangeTag::Insert => &green_fg_bg,
                    ChangeTag::Equal => style,
                };

                print!(
                    "{} {} {} │ ",
                    style.apply_to(sign),
                    style.apply_to(left_line_number),
                    style.apply_to(right_line_number)
                );

                // Handle inline content diffing

                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        eprint!("{}", emphasized_style.apply_to(value));
                    } else {
                        eprint!("{}", style.apply_to(value));
                    }
                }

                if change.missing_newline() {
                    eprintln!();
                }
            }
        }
    }

    false
}

pub fn compare_texts2(actual: &str, expected: &str, diff_mode: DiffMode) -> bool {
    let expected = expected.replace('\r', "");
    let actual = actual.replace('\r', "");

    if expected == actual {
        return true;
    }

    let diff = TextDiff::configure()
        .algorithm(Algorithm::Patience)
        .diff_lines(&actual, &expected);

    println!("{:->80}", "");
    println!(
        "{} / {}:\n",
        console::Style::new().color256(88).apply_to("Actual"),
        console::Style::new().green().apply_to("Expected")
    );

    let red = console::Style::new().color256(88);
    let green = console::Style::new().green();
    let dim = console::Style::new().dim();
    let red_fg_bg = console::Style::new().color256(52).on_color256(88);
    let green_fg_bg = console::Style::new()
        .color256(22)
        .on_color256(22)
        .on_green();

    let actual_max_digits = actual.lines().count().to_string().len();
    let expected_max_digits = expected.lines().count().to_string().len();
    let max_digits = std::cmp::max(actual_max_digits, expected_max_digits);

    for group in diff.grouped_ops(3) {
        let mut deletions = Vec::new();
        let mut insertions = Vec::new();

        for op in group {
            for change in diff.iter_changes(&op) {
                match change.tag() {
                    ChangeTag::Delete => deletions.push(change),
                    ChangeTag::Insert => insertions.push(change),
                    ChangeTag::Equal => {
                        // Process accumulated deletions and insertions before the Equal
                        let max = std::cmp::max(deletions.len(), insertions.len());
                        for i in 0..max {
                            let del = deletions.get(i);
                            let ins = insertions.get(i);

                            match (del, ins) {
                                (Some(d), Some(i)) => {
                                    let left = d.value();
                                    let right = i.value();

                                    let inline_diff = TextDiff::configure()
                                        .algorithm(Algorithm::Myers)
                                        .diff_chars(left, right);

                                    let left_ln = d
                                        .old_index()
                                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                                        .unwrap_or_else(|| " ".repeat(max_digits));
                                    let right_ln = i
                                        .new_index()
                                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                                        .unwrap_or_else(|| " ".repeat(max_digits));

                                    print!(
                                        "- {} {} │ ",
                                        red.apply_to(&left_ln),
                                        red.apply_to(&right_ln)
                                    );
                                    for c in inline_diff.iter_all_changes() {
                                        let style = match c.tag() {
                                            ChangeTag::Delete => &red_fg_bg,
                                            ChangeTag::Insert => &green_fg_bg,
                                            ChangeTag::Equal => &dim,
                                        };
                                        if c.value() == "\n" {
                                            eprintln!();
                                        } else {
                                            eprint!("{}", style.apply_to(c.value()));
                                        }
                                    }
                                    eprintln!();
                                }
                                (Some(d), None) => {
                                    let ln = d
                                        .old_index()
                                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                                        .unwrap_or_else(|| " ".repeat(max_digits));
                                    print!("- {} {} │ ", red.apply_to(&ln), red.apply_to(""));
                                    for c in d.value().chars() {
                                        if c == '\n' {
                                            eprintln!();
                                        } else {
                                            eprint!("{}", red_fg_bg.apply_to(c));
                                        }
                                    }
                                    eprintln!();
                                }
                                (None, Some(i)) => {
                                    let ln = i
                                        .new_index()
                                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                                        .unwrap_or_else(|| " ".repeat(max_digits));
                                    print!("+ {} {} │ ", green.apply_to(""), green.apply_to(&ln));
                                    for c in i.value().chars() {
                                        if c == '\n' {
                                            eprintln!();
                                        } else {
                                            eprint!("{}", green_fg_bg.apply_to(c));
                                        }
                                    }
                                    eprintln!();
                                }
                                _ => {}
                            }
                        }
                        deletions.clear();
                        insertions.clear();

                        // Print the equal line
                        let eq_ln = change
                            .old_index()
                            .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                            .unwrap_or_else(|| " ".repeat(max_digits));
                        print!("  {} {} │ ", dim.apply_to(&eq_ln), dim.apply_to(&eq_ln));
                        eprint!("{}", dim.apply_to(change.value()));
                        eprintln!();
                    }
                }
            }
        }

        // Handle any leftovers after the last Equal
        let max = std::cmp::max(deletions.len(), insertions.len());
        for i in 0..max {
            let del = deletions.get(i);
            let ins = insertions.get(i);

            match (del, ins) {
                (Some(d), Some(i)) => {
                    let left = d.value();
                    let right = i.value();

                    let inline_diff = TextDiff::configure()
                        .algorithm(Algorithm::Myers)
                        .diff_chars(left, right);

                    let left_ln = d
                        .old_index()
                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                        .unwrap_or_else(|| " ".repeat(max_digits));
                    let right_ln = i
                        .new_index()
                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                        .unwrap_or_else(|| " ".repeat(max_digits));

                    print!(
                        "- {} {} │ ",
                        red.apply_to(&left_ln),
                        red.apply_to(&right_ln)
                    );
                    for c in inline_diff.iter_all_changes() {
                        let style = match c.tag() {
                            ChangeTag::Delete => &red_fg_bg,
                            ChangeTag::Insert => &green_fg_bg,
                            ChangeTag::Equal => &dim,
                        };
                        if c.value() == "\n" {
                            eprintln!();
                        } else {
                            eprint!("{}", style.apply_to(c.value()));
                        }
                    }
                    eprintln!();
                }
                (Some(d), None) => {
                    let ln = d
                        .old_index()
                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                        .unwrap_or_else(|| " ".repeat(max_digits));
                    print!("- {} {} │ ", red.apply_to(&ln), red.apply_to(""));
                    for c in d.value().chars() {
                        if c == '\n' {
                            eprintln!();
                        } else {
                            eprint!("{}", red_fg_bg.apply_to(c));
                        }
                    }
                    eprintln!();
                }
                (None, Some(i)) => {
                    let ln = i
                        .new_index()
                        .map(|x| format!("{:0>width$}", x + 1, width = max_digits))
                        .unwrap_or_else(|| " ".repeat(max_digits));
                    print!("+ {} {} │ ", green.apply_to(""), green.apply_to(&ln));
                    for c in i.value().chars() {
                        if c == '\n' {
                            eprintln!();
                        } else {
                            eprint!("{}", green_fg_bg.apply_to(c));
                        }
                    }
                    eprintln!();
                }
                _ => {}
            }
        }
    }

    false
}

#[macro_export]
macro_rules! assert_text_eq {
    ($actual:expr, $expected:expr) => {
        // TODO make more efficient
        // TODO add optional message
        if !$crate::test_util::compare_texts(&$actual.to_string(), &$expected.to_string()) {
            panic!("assert_text_eq! failed");
        }
    };
}

#[macro_export]
macro_rules! assert_ir {
    ($actual:expr, $expected:expr) => {{
        match $crate::test_util::match_lines($actual, $expected) {
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
