use askama::Template;

use crate::content::Directory;

/// One line of a listing.
pub struct Line {
    pub label: String,
    pub url_path: String,
    /// What the document covers. Directories and the parent link have none.
    pub summary: Option<String>,
}

/// A directory, as the markdown an agent reads when it lists a path ending in `/`.
#[derive(Template)]
#[template(path = "directory.md")]
pub struct DirectoryListing {
    pub url_path: String,
    pub lines: Vec<Line>,
}

impl DirectoryListing {
    /// The parent link first, then subdirectories, then documents.
    pub fn build(directory: &Directory<'_>) -> DirectoryListing {
        let parent = directory.path.to_name().map(|_| Line {
            label: "../".to_owned(),
            url_path: directory.path.as_str().rsplit_once('/').map_or("/".to_owned(), |(parent, _)| format!("/{parent}/")),
            summary: None,
        });
        let directories = directory.directories.iter().map(|path| Line {
            label: path.to_url_path(),
            url_path: path.to_url_path(),
            summary: None,
        });
        let documents = directory.documents.iter().map(|document| Line {
            label: document.title.clone(),
            url_path: document.path.to_url_path(),
            summary: Some(document.summary.clone()),
        });

        DirectoryListing {
            url_path: directory.path.to_url_path(),
            lines: parent.into_iter().chain(directories).chain(documents).collect(),
        }
    }
}
