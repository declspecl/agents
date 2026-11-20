use std::fmt;

use super::Variable;

#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    pub variable: Variable,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnvironmentError {
    pub problems: Vec<Problem>,
}

impl fmt::Display for EnvironmentError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "invalid environment:")?;
        for problem in &self.problems {
            write!(f, "\n  {}: {}", problem.variable, problem.message)?;
        }

        Ok(())
    }
}

impl std::error::Error for EnvironmentError {}
