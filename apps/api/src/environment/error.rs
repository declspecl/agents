use thiserror::Error;

use super::Variable;
use crate::problem_list::format_problems;

/// One variable that could not be used, and why.
#[derive(Debug, PartialEq, Eq, Error)]
#[error("{variable}: {message}")]
pub struct Problem {
    pub variable: Variable,
    pub message: String,
}

/// Every problem found while reading the environment. Never empty.
#[derive(Debug, PartialEq, Eq, Error)]
#[error("invalid environment:{}", format_problems(.problems))]
pub struct EnvironmentError {
    pub problems: Vec<Problem>,
}
