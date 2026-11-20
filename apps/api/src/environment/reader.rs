use std::fmt;
use std::str::FromStr;

use super::{
    EnvironmentError,
    Problem,
    Variable,
};

/// Reads variables from a lookup, remembering each failure so they can all be reported together.
pub(super) struct Reader<L> {
    lookup: L,
    problems: Vec<Problem>,
}

impl<L: Fn(&str) -> Option<String>> Reader<L> {
    pub(super) fn new(lookup: L) -> Self {
        Reader {
            lookup,
            problems: Vec::new(),
        }
    }

    /// The parsed value, or `None` when the variable is unset, blank, or invalid. An invalid value is recorded.
    pub(super) fn read_optional<T: FromStr>(
        &mut self,
        variable: Variable,
    ) -> Option<T>
    where
        T::Err: fmt::Display,
    {
        let raw = (self.lookup)(variable.as_str())?;
        let raw = raw.trim();
        if raw.is_empty() {
            return None;
        }

        match raw.parse::<T>() {
            Ok(value) => Some(value),
            Err(error) => {
                self.problems.push(Problem {
                    variable,
                    message: format!("{error} (got {raw:?})"),
                });
                None
            },
        }
    }

    /// Like [`Reader::read_optional`], and a missing value is recorded as a problem too.
    pub(super) fn read_required<T: FromStr>(
        &mut self,
        variable: Variable,
    ) -> Option<T>
    where
        T::Err: fmt::Display,
    {
        let before = self.problems.len();
        let value = self.read_optional(variable);
        if value.is_none() && self.problems.len() == before {
            self.problems.push(Problem {
                variable,
                message: "is required".to_owned(),
            });
        }

        value
    }

    /// Fails with every recorded problem, or succeeds when there were none.
    pub(super) fn finish(self) -> Result<(), EnvironmentError> {
        if self.problems.is_empty() {
            Ok(())
        } else {
            Err(EnvironmentError {
                problems: self.problems,
            })
        }
    }
}
