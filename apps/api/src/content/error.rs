use thiserror::Error;

use crate::problem_list::format_problems;

/// One source file that could not become a document, and why.
#[derive(Debug, PartialEq, Eq, Error)]
#[error("{file}: {message}")]
pub struct Problem {
    pub file: String,
    pub message: String,
}

/// Every problem found while loading content. Never empty.
#[derive(Debug, PartialEq, Eq, Error)]
#[error("invalid content:{}", format_problems(.problems))]
pub struct ContentError {
    pub problems: Vec<Problem>,
}
