use serde::Deserialize;
use thiserror::Error;

use super::DocumentPath;

const FRONT_MATTER_FENCE: &str = "+++";

/// Why a source file is not a valid document.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum DocumentError {
    #[error("must start with a +++ front matter fence")]
    MissingOpeningFence,
    #[error("front matter must end with a +++ fence on its own line")]
    MissingClosingFence,
    #[error("invalid front matter: {0}")]
    InvalidFrontMatter(#[from] toml::de::Error),
    #[error("front matter {0} must not be blank")]
    BlankField(&'static str),
    #[error("body must begin with a level-one heading")]
    MissingHeading,
}

/// The metadata at the top of a source file, between `+++` fences, as TOML.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrontMatter {
    title: String,
    summary: String,
}

/// One markdown document. The indexes use its title and summary as link text and description; `body` is what is served.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    pub path: DocumentPath,
    /// The name an index shows for this document.
    pub title: String,
    /// One sentence on what the document covers, for an agent deciding whether to read it.
    pub summary: String,
    /// Markdown. It begins with a level-one heading, since an agent reads it with no other context.
    pub body: String,
}

impl Document {
    /// Splits front matter from the body and checks both.
    pub fn parse(
        path: DocumentPath,
        source: &str,
    ) -> Result<Document, DocumentError> {
        let after_open = source.strip_prefix(FRONT_MATTER_FENCE).and_then(|rest| rest.strip_prefix('\n')).ok_or(DocumentError::MissingOpeningFence)?;
        let (front_matter, body) = after_open.split_once(&format!("\n{FRONT_MATTER_FENCE}\n")).ok_or(DocumentError::MissingClosingFence)?;

        let FrontMatter {
            title,
            summary,
        } = toml::from_str(front_matter)?;
        for (field, value) in [("title", &title), ("summary", &summary)] {
            if value.trim().is_empty() {
                return Err(DocumentError::BlankField(field));
            }
        }

        let body = body.trim_start_matches('\n').to_owned();
        if !body.starts_with("# ") {
            return Err(DocumentError::MissingHeading);
        }

        Ok(Document {
            path,
            title,
            summary,
            body,
        })
    }
}
