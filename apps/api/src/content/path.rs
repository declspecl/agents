use std::fmt;

use thiserror::Error;

/// Why a path is not a valid content path.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum PathError {
    #[error("{0:?} must be lowercase letters, digits and single hyphens")]
    InvalidName(String),
    #[error("{0:?} must end in .md")]
    NotMarkdown(String),
}

const DOCUMENT_EXTENSION: &str = ".md";

/// A lowercase, hyphenated name: letters and digits, with single hyphens only between them.
fn check_name(name: &str) -> Result<(), PathError> {
    let valid = !name.is_empty()
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--")
        && name.chars().all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-');

    if valid {
        Ok(())
    } else {
        Err(PathError::InvalidName(name.to_owned()))
    }
}

/// The location of a document, relative to the content root, such as `preferences/code-style.md`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocumentPath {
    relative: String,
}

impl DocumentPath {
    /// Parses a relative path whose every directory is a valid name and whose last segment is a valid name plus `.md`.
    pub fn parse(relative: &str) -> Result<DocumentPath, PathError> {
        let (directories, file) = match relative.rsplit_once('/') {
            Some((directories, file)) => (Some(directories), file),
            None => (None, relative),
        };

        for directory in directories.into_iter().flat_map(|directories| directories.split('/')) {
            check_name(directory)?;
        }
        let stem = file.strip_suffix(DOCUMENT_EXTENSION).ok_or_else(|| PathError::NotMarkdown(file.to_owned()))?;
        check_name(stem)?;

        Ok(DocumentPath {
            relative: relative.to_owned(),
        })
    }

    /// The relative path, such as `preferences/code-style.md`.
    pub fn as_str(&self) -> &str {
        &self.relative
    }

    /// The path a client requests, such as `/preferences/code-style.md`.
    pub fn to_url_path(&self) -> String {
        format!("/{}", self.relative)
    }

    /// The directory that holds this document.
    pub fn to_directory(&self) -> DirectoryPath {
        match self.relative.rsplit_once('/') {
            Some((directory, _)) => DirectoryPath {
                relative: directory.to_owned(),
            },
            None => DirectoryPath::ROOT,
        }
    }
}

impl fmt::Display for DocumentPath {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.relative)
    }
}

/// The location of a directory, relative to the content root. The root is the empty path.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectoryPath {
    relative: String,
}

impl DirectoryPath {
    pub const ROOT: DirectoryPath = DirectoryPath {
        relative: String::new(),
    };

    /// Parses a relative path with no trailing slash, such as `preferences`. The empty string is the root.
    pub fn parse(relative: &str) -> Result<DirectoryPath, PathError> {
        for name in relative.split('/').filter(|_| !relative.is_empty()) {
            check_name(name)?;
        }

        Ok(DirectoryPath {
            relative: relative.to_owned(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.relative
    }

    /// The path a client requests: `/` for the root, otherwise `/preferences/`.
    pub fn to_url_path(&self) -> String {
        if self.relative.is_empty() {
            "/".to_owned()
        } else {
            format!("/{}/", self.relative)
        }
    }

    /// The directory named `name` inside this one.
    pub fn to_child(
        &self,
        name: &str,
    ) -> DirectoryPath {
        if self.relative.is_empty() {
            DirectoryPath {
                relative: name.to_owned(),
            }
        } else {
            DirectoryPath {
                relative: format!("{}/{name}", self.relative),
            }
        }
    }

    /// The last segment, or `None` for the root.
    pub fn to_name(&self) -> Option<&str> {
        match self.relative.rsplit_once('/') {
            Some((_, name)) => Some(name),
            None if self.relative.is_empty() => None,
            None => Some(&self.relative),
        }
    }
}

impl fmt::Display for DirectoryPath {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.to_url_path())
    }
}
