use crate::{
    ast::Span,
    error_reporting::{CompileError, CompileErrorList},
};

pub const INDENT: &str = "<<INDENT>>";
pub const DEDENT: &str = "<<DEDENT>>";

#[derive(Debug)]
pub struct PreprocessResult {
    pub original: String,
    pub transformed: String,
    pub rev_offset_map: Vec<Option<usize>>, // transformed → original
}
impl PreprocessResult {
    /// Map a span in the transformed code back to a span in the original source.
    pub fn map_span_back(
        &self,
        transformed_start: usize,
        transformed_end: usize,
    ) -> Option<(usize, usize)> {
        let orig_start = self
            .rev_offset_map
            .get(transformed_start)
            .copied()
            .flatten();
        let orig_end = self.rev_offset_map.get(transformed_end).copied().flatten();
        orig_start.zip(orig_end)
    }
}

fn push_char_with_map(
    transformed: &mut String,
    rev_offset_map: &mut Vec<Option<usize>>,
    c: char,
    orig_byte: usize,
) {
    let mut buf = [0; 4];
    let encoded = c.encode_utf8(&mut buf);
    transformed.push_str(encoded);
    for _ in 0..encoded.len() {
        rev_offset_map.push(Some(orig_byte));
    }
}

pub fn preprocess_indentation(source: &str) -> Result<PreprocessResult, CompileErrorList> {
    let mut transformed = String::with_capacity(source.len());
    let mut rev_offset_map = Vec::with_capacity(source.len());

    let mut orig_index = 0;
    let mut indent_stack = vec![0];
    let mut error_list = CompileErrorList::new();
    let mut indent_char: Option<char> = None;

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;
        let line_start = orig_index;
        let line_len = line.len();
        let trimmed = line.trim_start();
        let current_indent = line.len() - trimmed.len();
        let raw_indent = &line[..current_indent]; // slice only

        if trimmed.is_empty() || trimmed.starts_with('#') {
            for (i, c) in line.char_indices() {
                let orig_byte = line_start + i;
                push_char_with_map(&mut transformed, &mut rev_offset_map, c, orig_byte);
            }

            transformed.push('\n');
            rev_offset_map.push(Some(line_start + line.len()));
            orig_index = line_start + line.len() + 1;
            continue;
        }

        // Detect indent type
        if indent_char.is_none() {
            if raw_indent.contains(' ') {
                indent_char = Some(' ');
            } else if raw_indent.contains('\t') {
                indent_char = Some('\t');
            }
        }

        // Check for consistency
        if let Some(expected_char) = indent_char {
            if raw_indent.chars().any(|c| c != expected_char) {
                let span = Span {
                    line: line_num,
                    column: 0,
                    start: line_start,
                    end: line_start,
                };
                error_list.push_error(CompileError::mixed_indentation(line_num, span));
                orig_index += line_len + 1; // skip entire line
                continue;
            }
        }

        let last_indent = *indent_stack.last().unwrap();

        match current_indent.cmp(&last_indent) {
            std::cmp::Ordering::Greater => {
                indent_stack.push(current_indent);
                transformed.push_str(INDENT);
                rev_offset_map.extend(std::iter::repeat_n(None, INDENT.len())); // synthetic
            }
            std::cmp::Ordering::Less => {
                while let Some(&prev) = indent_stack.last() {
                    if prev > current_indent {
                        indent_stack.pop();
                        transformed.push_str(DEDENT);
                        rev_offset_map.extend(std::iter::repeat_n(None, DEDENT.len())); // synthetic
                    } else {
                        break;
                    }
                }
            }
            std::cmp::Ordering::Equal => {}
        }

        // Write trimmed part of line, map each character
        for (j_byte, c) in trimmed.char_indices() {
            let orig_byte = line_start + current_indent + j_byte;
            push_char_with_map(&mut transformed, &mut rev_offset_map, c, orig_byte);
        }

        transformed.push('\n');
        rev_offset_map.push(Some(line_start + line_len));

        orig_index = line_start + line_len + 1;
    }

    // Final dedents
    while indent_stack.len() > 1 {
        debug_assert!(!indent_stack.is_empty(), "Indentation stack underflow");
        indent_stack.pop();
        transformed.push_str(DEDENT);
        rev_offset_map.extend(std::iter::repeat_n(None, DEDENT.len())); // synthetic
    }

    if error_list.is_empty() {
        Ok(PreprocessResult {
            original: source.to_string(),
            transformed,
            rev_offset_map,
        })
    } else {
        Err(error_list)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_text_eq, error_reporting::CompileErrorKind};

    use super::*;

    fn clean(input: &str) -> String {
        input.trim_matches('\n').to_string()
    }

    #[test]
    fn test_no_indentation() {
        let source = "a = 1\nb = 2\n";
        let result = preprocess_indentation(source).unwrap();
        assert!(!result.transformed.contains(INDENT));
        assert!(!result.transformed.contains(DEDENT));
        assert!(result.transformed.contains("a = 1"));
        assert!(result.transformed.contains("b = 2"));
    }

    #[test]
    fn test_multiple_dedents_in_row() {
        let source = "\
if x:
  if y:
    a = 1
b = 2\n";

        let result = preprocess_indentation(source).unwrap();
        eprintln!("{}", result.transformed);
        eprintln!("Rev map length: {}", result.rev_offset_map.len());
        assert_text_eq!(result.transformed.matches(DEDENT).count(), 2);
    }

    #[test]
    fn test_blank_line_inside_block() {
        let source = "\
if x:
  a = 1
  
  b = 2";

        let result = preprocess_indentation(source).unwrap();
        eprintln!("{}", result.transformed);
        eprintln!("Rev map length: {}", result.rev_offset_map.len());
        assert!(result.transformed.contains("a = 1"));
        assert!(result.transformed.contains("b = 2"));
        // Ensure dedents are not inserted after blank lines
        assert_text_eq!(result.transformed.matches(DEDENT).count(), 1);
    }

    #[test]
    fn test_empty_blank_line_inside_block() {
        let source = "\
if x:
  a = 1

  b = 2";

        let result = preprocess_indentation(source).unwrap();
        eprintln!("{}", result.transformed);
        eprintln!("Rev map length: {}", result.rev_offset_map.len());
        assert!(result.transformed.contains("a = 1"));
        assert!(result.transformed.contains("b = 2"));
        // Ensure dedents are not inserted after blank lines
        assert_text_eq!(result.transformed.matches(DEDENT).count(), 1);
    }

    #[test]
    fn test_eof_inside_block() {
        let source = "
if x:
  a = 1";

        let result = preprocess_indentation(source).unwrap();
        eprintln!("{}", result.transformed);
        eprintln!("Rev map length: {}", result.rev_offset_map.len());
        assert!(result.transformed.ends_with(DEDENT));
    }

    #[test]
    fn test_unicode_indent_edge_case() {
        let source = "if x:\n🚀🚀a = 1\n";
        let result = preprocess_indentation(source).unwrap();
        assert!(result.transformed.contains("a = 1"));
    }

    #[test]
    fn test_simple_indent_dedent() {
        let source = clean(
            r#"
if x:
    y = 1
    z = 2
"#,
        );

        let result = preprocess_indentation(&source).unwrap();

        assert!(result.transformed.contains(INDENT));
        assert!(result.transformed.contains(DEDENT));

        let expected = clean(
            r#"
if x:
<<INDENT>>y = 1
z = 2
<<DEDENT>>
"#,
        );

        assert_text_eq!(clean(&result.transformed), expected);
    }

    #[test]
    fn test_offset_mapping() {
        let source = clean(
            r#"
if x:
    y = 42
"#,
        );

        let result = preprocess_indentation(&source).unwrap();

        // Find span of "42" in transformed code
        let transformed = &result.transformed;
        let start = transformed.find("42").unwrap();
        let end = start + 2;

        let (orig_start, orig_end) = result.map_span_back(start, end).unwrap();

        assert_text_eq!(&source[orig_start..orig_end], "42");
    }

    #[test]
    fn test_mixed_indentation_error() {
        let source = "if x:\n \ty = 1\n";

        let result = preprocess_indentation(source);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .into_iter()
                .any(|e| matches!(e.kind, CompileErrorKind::MixedIndentation { .. }))
        );
    }

    #[test]
    fn test_nested_indents() {
        let source = clean(
            r#"
if x:
    if y:
        z = 1
"#,
        );

        let result = preprocess_indentation(&source).unwrap();

        let expected = clean(
            r#"
if x:
<<INDENT>>if y:
<<INDENT>>z = 1
<<DEDENT>><<DEDENT>>
"#,
        );

        assert_text_eq!(clean(&result.transformed), expected);
    }

    #[test]
    fn test_simple_indent_and_dedent() {
        let source = "if x:\n  y = 1\nz = 2\n";
        let result = preprocess_indentation(source).unwrap();
        assert!(result.transformed.contains(INDENT));
        assert!(result.transformed.contains(DEDENT));
        assert!(result.transformed.contains("y = 1"));
        assert!(result.transformed.contains("z = 2"));
    }

    #[test]
    fn test_mixed_indentation_error2() {
        let source = "if x:\n\t y = 1\n";
        let result = preprocess_indentation(source);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.into_iter()
                .any(|e| matches!(e.kind, CompileErrorKind::MixedIndentation { .. }))
        );
    }

    #[test]
    fn test_nested_blocks() {
        let source = "if x:\n  if y:\n    z = 1\n  w = 2\nu = 3\n";
        let result = preprocess_indentation(source).unwrap();
        let transformed = &result.transformed;
        assert_text_eq!(transformed.matches(INDENT).count(), 2);
        assert_text_eq!(transformed.matches(DEDENT).count(), 2);
    }

    #[test]
    fn test_dedent_with_trailing_blank_lines() {
        let source = "if x:\n  y = 1\n\n\n";
        let result = preprocess_indentation(source).unwrap();
        assert!(result.transformed.ends_with(DEDENT));
    }

    #[test]
    fn test_artificial_token_not_mapped() {
        let source = "if x:\n  y = 1\n";
        let result = preprocess_indentation(source).unwrap();
        let idx = result.transformed.find(INDENT).unwrap();
        let actual_opt = result.rev_offset_map.iter().position(|&v| v == Some(idx));
        if let Some(actual) = &actual_opt {
            eprintln!("Expected None, got Some({})", actual);
            assert!(actual_opt.is_none());
        }
    }

    #[test]
    fn test_span_mapping_roundtrip() {
        let source = "if x:\n  y = 1\nz = 2\n";
        let result = preprocess_indentation(source).unwrap();
        let start = result.transformed.find("y = 1").unwrap();
        let end = start + "y = 1".len();
        let (orig_start, orig_end) = result.map_span_back(start, end).unwrap();
        assert_text_eq!(&result.original[orig_start..orig_end], "y = 1");
    }

    #[test]
    fn test_deep_indentation() {
        let source = "if x:\n  if y:\n    if z:\n      a = 1\n";
        let result = preprocess_indentation(source).unwrap();
        assert_text_eq!(result.transformed.matches(INDENT).count(), 3);
        assert_text_eq!(result.transformed.matches(DEDENT).count(), 3);
    }

    #[test]
    fn test_comment_only_lines() {
        let source = "# comment\nif x:\n  # inner comment\n  y = 1\n";
        let result = preprocess_indentation(source).unwrap();
        assert!(result.transformed.contains(INDENT));
        assert!(!result.transformed.contains("<<DEDENT>>\n#"));
    }

    #[test]
    fn test_tabs_only_indent() {
        let source = "if x:\n\tprint = 1\n";
        let result = preprocess_indentation(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_spaces_only_indent() {
        let source = "if x:\n    print = 1\n";
        let result = preprocess_indentation(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unicode_comment_span_handling() {
        let source = "if x:\n  y = 1   # → should not panic\n";
        let result = preprocess_indentation(source).unwrap();

        // Check that the transformed version still contains the original line
        assert!(
            result.transformed.contains("y = 1"),
            "Transformed output should contain the line with code"
        );

        // Confirm no panic and span map still points back correctly
        let start = result.transformed.find("1").unwrap();
        let end = start + 1;
        let (orig_start, orig_end) = result.map_span_back(start, end).unwrap();
        assert_text_eq!(&result.original[orig_start..orig_end], "1");
    }

    #[test]
    fn test_unicode_comment_does_not_break_span_mapping() {
        let source = "let a = x * 3   # → should stay as x * 3\n";
        let result = preprocess_indentation(source).unwrap();

        // Ensure transformed contains the expected number
        let start = result.transformed.find("3").expect("should find '3'");
        let end = start + "3".len();

        // Map back to original
        let (orig_start, orig_end) = result
            .map_span_back(start, end)
            .expect("span mapping should succeed");

        assert_text_eq!(&result.original[orig_start..orig_end], "3");
    }

    #[test]
    fn test_unicode_comment_span_map_consistency() {
        let source = "let a = x * 3   # → stay 🚀 ثابت \n";
        let result = preprocess_indentation(source).unwrap();

        // Ensure the transformed code still contains the literal we care about
        let transformed = &result.transformed;
        let span_start = transformed.find("3").expect("should find '3'");
        let span_end = span_start + "3".len();

        // Map span back to original
        let (orig_start, orig_end) = result
            .map_span_back(span_start, span_end)
            .expect("span mapping should succeed");

        assert_text_eq!(&result.original[orig_start..orig_end], "3");

        // 🔍 Extra check: no None mappings in the neighborhood of '3'
        for offset in span_start.saturating_sub(3)..=span_end + 3 {
            if let Some(Some(orig)) = result.rev_offset_map.get(offset) {
                // valid mapping – great
                assert!(
                    *orig <= result.original.len(),
                    "mapped offset out of bounds"
                );
            } else {
                // Only allow `None` if it's a known synthetic token
                let synthetic_slice =
                    &transformed[offset..offset.saturating_add(10).min(transformed.len())];
                assert!(
                    synthetic_slice.starts_with(INDENT) || synthetic_slice.starts_with(DEDENT),
                    "Unexpected None in offset map at {}: '{}'",
                    offset,
                    synthetic_slice
                );
            }
        }
    }

    #[test]
    fn test_unicode_comment_does_not_break_span_afterward() {
        let source = "let a = x * 3   # → Unicode comment\nlet b = x + 1\n";
        let result = preprocess_indentation(source).unwrap();

        let transformed = &result.transformed;

        let x_index = transformed.rfind("x").expect("should find second 'x'");
        let (orig_start, _) = result
            .map_span_back(x_index, x_index + 1)
            .expect("span mapping should succeed");

        eprintln!("Transformed:\n{}", result.transformed);
        eprintln!(
            "Mapped slice: '{}'",
            &result.original[orig_start..orig_start + 1]
        );
        eprintln!("Expected slice: 'x'");

        assert_text_eq!(&result.original[orig_start..orig_start + 1], "x");
    }

    #[test]
    fn test_unicode_span_alignment_for_diagnostics() {
        let source = "\
let 🚀 = 1
let a = 🧠 + x + ☃️  # x is undefined here
";

        let result = crate::preprocessor::preprocess_indentation(source).unwrap();

        // Find the 'x' in the transformed code
        let transformed = &result.transformed;
        let x_index = transformed.find('x').expect("should find 'x'");
        let (orig_start, orig_end) = result
            .map_span_back(x_index, x_index + 1)
            .expect("span mapping should succeed");

        // Confirm the span maps to 'x' in the original source
        assert_text_eq!(&result.original[orig_start..orig_end], "x");

        // Confirm that visual column count places caret correctly
        let line_start = result.original[..orig_start]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let visual_col = result.original[line_start..orig_start].chars().count();

        // We expect the line to contain the comment (line 2)
        let line = result.original.lines().nth(1).unwrap();
        let mut caret_line = String::new();
        for (i, _c) in line.chars().enumerate() {
            if i == visual_col {
                caret_line.push('┬');
                break;
            }
            caret_line.push(' ');
        }

        println!("source line: {}", line);
        println!("caret line : {}", caret_line);

        // Just ensure caret is under the 'x'
        assert_text_eq!(line.chars().nth(visual_col).unwrap(), 'x');
    }
}
