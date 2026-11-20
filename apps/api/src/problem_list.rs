use std::fmt::Display;

/// Lays out problems one per line, indented, for an error message that reports every one of them.
pub fn format_problems<T: Display>(problems: &[T]) -> String {
    problems.iter().map(|problem| format!("\n  {problem}")).collect()
}
