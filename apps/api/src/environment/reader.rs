use std::fmt;
use std::str::FromStr;

use super::{
    EnvironmentError,
    Problem,
    Variable,
};

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
