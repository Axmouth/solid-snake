use std::ops::Range;

use crate::{ast::Span, preprocessor::PreprocessResult};

use ariadne::{Color, Label, Report, ReportKind, Source};
use backtrace::Backtrace;
use std::backtrace::Backtrace as BacktraceStd;

use std::path::Path;

#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: CompileErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum CompileErrorKind {
    SyntaxError(String),
    UnexpectedError(String),
    InternalCompilerError { error: String, callers: Vec<String> },
    InvalidOperand(String),
    InvalidNumeric(String),
    MixedIndentation { line: usize },
    ReadUndefinedVariable { name: String },
    AssignUndefinedVariable { name: String },
    TypeMismatch { expected: String, actual: String },
    UntypedVariable { name: String },
    UnexpectedEndOfInput,
}

impl CompileError {
    pub fn new(kind: CompileErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn at(kind: CompileErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    #[inline(never)]
    pub fn internal_compiler_error(txt: impl Into<String>) -> Self {
        let callers = print_callers();
        Self {
            kind: CompileErrorKind::InternalCompilerError {
                error: txt.into(),
                callers,
            },
            span: Span {
                line: 0,
                column: 0,
                start: 0,
                end: 0,
            },
        }
    }

    pub fn syntax_error(op: impl Into<String>, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::SyntaxError(op.into()),
            span,
        }
    }

    pub fn unexpected_error(op: impl Into<String>, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedError(op.into()),
            span,
        }
    }

    pub fn invalid_operand(op: impl Into<String>, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::InvalidOperand(op.into()),
            span,
        }
    }

    pub fn invalid_numeric(literal: impl Into<String>, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::InvalidNumeric(literal.into()),
            span,
        }
    }

    pub fn mixed_indentation(line: usize, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::MixedIndentation { line },
            span,
        }
    }

    pub fn read_undefined_variable(name: impl Into<String>, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::ReadUndefinedVariable { name: name.into() },
            span,
        }
    }

    pub fn assign_undefined_variable(name: impl Into<String>, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::AssignUndefinedVariable { name: name.into() },
            span,
        }
    }

    pub fn untyped_variable(name: impl Into<String>, span: Span) -> Self {
        Self {
            kind: CompileErrorKind::UntypedVariable { name: name.into() },
            span,
        }
    }

    pub fn type_mismatch(
        expected: impl Into<String>,
        actual: impl Into<String>,
        span: Span,
    ) -> Self {
        Self {
            kind: CompileErrorKind::TypeMismatch {
                expected: expected.into(),
                actual: actual.into(),
            },
            span,
        }
    }

    pub fn unexpected_eof(span: Span) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedEndOfInput,
            span,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompileErrorList {
    errors: Vec<CompileError>,
}

impl CompileErrorList {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.len() < 1
    }

    pub fn is_fatal(&self) -> bool {
        !self.errors.is_empty()
    }

    // Cause we might have warnings later too
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn push_error(&mut self, error: CompileError) {
        self.errors.push(error)
    }

    pub fn extend(&mut self, errors: impl IntoIterator<Item = CompileError>) {
        self.errors.extend(errors)
    }

    pub fn with_error(mut self, error: CompileError) -> Self {
        self.errors.push(error);
        self
    }

    pub fn with_errors<I: IntoIterator<Item = CompileError>>(mut self, errors: I) -> Self {
        self.errors.extend(errors);
        self
    }

    pub fn err_iter(&self) -> impl Iterator<Item = &CompileError> {
        self.errors.iter()
    }
}

impl IntoIterator for CompileErrorList {
    type Item = CompileError;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.errors.into_iter())
    }
}

impl From<CompileError> for CompileErrorList {
    fn from(error: CompileError) -> Self {
        CompileErrorList::new().with_error(error)
    }
}

#[inline(never)]
pub fn print_callers() -> Vec<String> {
    let bt = Backtrace::new();
    let mut output = Vec::new();
    let mut collecting = false;

    #[cfg(not(debug_assertions))]
    {
        let fbt = BacktraceStd::force_capture();

        for line in fbt.to_string().lines() {
            let line = line.trim();

            // Skip numeric frame index lines
            if line.starts_with(|c: char| c.is_ascii_digit()) {
                // continue;
            }

            // Keep only your crate's functions or paths
            if line.contains("solid_snake_compiler") && !line.contains("print_callers") {
                output.push(line.to_string());
            }
        }

        output
    }

    #[cfg(debug_assertions)]
    {
        for frame in bt.frames() {
            for symbol in frame.symbols() {
                let Some(file) = symbol.filename() else {
                    continue;
                };
                let Some(line) = symbol.lineno() else {
                    continue;
                };
                let func_name = symbol.name().map(|n| n.to_string()).unwrap_or_default();

                let file_str = file.to_string_lossy();

                // Skip internal or boilerplate frames
                let is_internal = file_str.contains(".cargo")
                // || file_str.contains("/backtrace")
                || file_str.starts_with("/rustc/")
                || file_str.contains(".rustup/toolchains")
                || file_str.contains(".rustup\\toolchains")
                || func_name.contains("print_callers")
                || func_name.contains("__scrt")
                || func_name.contains("lang_start");

                if is_internal {
                    continue;
                }

                // Start collecting after we get past internal/helper frames
                collecting = true;

                if collecting {
                    let file_name = Path::new(&file).to_str().unwrap_or("<file>");
                    let display_func = if func_name.is_empty() {
                        "<unknown>".to_string()
                    } else {
                        func_name
                    };

                    output.push(format!(
                        "{}:{} (function: {})",
                        file_name, line, display_func
                    ));
                }
            }
        }

        output
    }
}

/// Helper to map a span to byte range in the original source
fn map_span(span: Span, name_len: usize, preprocessed: &PreprocessResult) -> Option<Range<usize>> {
    preprocessed
        .map_span_back(span.start, span.start + name_len)
        // Fix for unicode, visual offset is not same as byte offset
        .map(|(start, end)| {
            preprocessed.original[..start].chars().count()
                ..preprocessed.original[..end].chars().count()
        })
}

/// Generic error emitter
#[allow(clippy::too_many_arguments)]
fn emit_report(
    kind: ReportKind,
    message: &str,
    span_range: Range<usize>,
    label: &str,
    color: Color,
    help: Option<&str>,
    source_file: &str,
    source: &str,
) {
    let mut report = Report::build(kind, (source_file, span_range.clone()))
        .with_message(message)
        .with_label(
            Label::new((source_file, (span_range)))
                .with_message(label)
                .with_color(color),
        );

    if let Some(help) = help {
        report = report.with_help(help);
    }

    report
        .finish()
        .eprint((source_file, Source::from(source)))
        .unwrap();
}

// TODO infra for reporting warnings too
pub fn report_error(error: &CompileError, source_file: &str, preprocessed: &PreprocessResult) {
    let span = error.span;
    let span_range = map_span(span, span.end - span.start, preprocessed).unwrap(); // TODO ..
    match &error.kind {
        CompileErrorKind::ReadUndefinedVariable { name }
        | CompileErrorKind::AssignUndefinedVariable { name } => {
            let is_assign = matches!(error.kind, CompileErrorKind::AssignUndefinedVariable { .. });
            let label = if is_assign {
                "Variable assigned before being declared."
            } else {
                "Variable used before being declared."
            };
            let Some(span_range) = map_span(span, name.len(), preprocessed) else {
                eprintln!("Could not map span for variable: {}", name);
                return;
            };

            emit_report(
                ReportKind::Error,
                &format!("Undefined variable '{}'", name),
                span_range,
                label,
                Color::Red,
                Some("Declare the variable first using 'let'."),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::UnexpectedEndOfInput => {
            let pos = preprocessed.original.len();
            emit_report(
                ReportKind::Error,
                "Unexpected end of input",
                pos..pos,
                "Parsing stopped here",
                Color::Red,
                Some("The parser expected more tokens."),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::InvalidNumeric(literal) => {
            emit_report(
                ReportKind::Error,
                &format!("Invalid numeric literal: {literal}"),
                span_range,
                "Not a valid number",
                Color::Yellow,
                Some("Check for typos or invalid characters."),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::InvalidOperand(op) => {
            emit_report(
                ReportKind::Error,
                &format!("Invalid operand: {op}"),
                span_range,
                "This operand is not valid in this context",
                Color::Yellow,
                Some("Make sure the operation supports this operand."),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::MixedIndentation { line } => {
            emit_report(
                ReportKind::Error,
                &format!("Inconsistent indentation on line {}", line),
                span_range,
                "Indentation conflict here",
                Color::Blue,
                Some("Mixing tabs and spaces is discouraged."),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::TypeMismatch { expected, actual } => {
            emit_report(
                ReportKind::Error,
                "Type mismatch",
                span_range,
                &format!("Expected '{expected}', but got '{actual}'"),
                Color::Cyan,
                Some("Ensure both sides of the expression use compatible types."),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::UntypedVariable { name } => {
            emit_report(
                ReportKind::Error,
                "Type mismatch",
                span_range,
                &format!("Failed to resolve type for variable '{name}'"),
                Color::Cyan,
                Some(&format!(
                    "Add a type hint for the variable: let {}: <type>.",
                    name
                )),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::UnexpectedError(error) => {
            emit_report(
                ReportKind::Error,
                "Unexpected Error",
                span_range,
                &format!("Unexpected Error: {}.", error),
                Color::Cyan,
                Some(&format!("Unexpected Error: {}.", error)),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::SyntaxError(error) => {
            emit_report(
                ReportKind::Error,
                "Syntax Error",
                span_range,
                &format!("Syntax Error: {}.", error),
                Color::Cyan,
                Some(&format!("Syntax Error: {}.", error)),
                source_file,
                &preprocessed.original,
            );
        }
        CompileErrorKind::InternalCompilerError { error, callers } => {
            emit_report(
                ReportKind::Error,
                "Internal Compiler Error",
                span_range,
                &format!("Internal Compiler Error: {}.", error),
                Color::Cyan,
                Some(&format!("Internal Compiler Error: {}.", error)),
                source_file,
                &preprocessed.original,
            );
            for caller in callers {
                eprintln!("{}", caller);
            }
        }
    }
}
