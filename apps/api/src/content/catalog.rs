use std::collections::{
    BTreeMap,
    BTreeSet,
};

use include_dir::{
    Dir,
    include_dir,
};

use super::{
    ContentError,
    Directory,
    DirectoryPath,
    Document,
    DocumentPath,
    Problem,
};

static EMBEDDED: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/content");

/// A source file before it is parsed: its path relative to the content root, and its text.
pub struct Source<'text> {
    pub path: String,
    pub text: &'text str,
}

/// Every document, by path.
#[derive(Debug)]
pub struct Catalog {
    documents: BTreeMap<DocumentPath, Document>,
}

impl Catalog {
    /// Parses the content embedded in the binary.
    pub fn load_embedded() -> Result<Catalog, ContentError> {
        let mut sources = Vec::new();
        let mut problems = Vec::new();
        collect_sources(&EMBEDDED, &mut sources, &mut problems);

        Catalog::load_from(sources, problems)
    }

    /// Parses `sources`, reporting every bad one together. `problems` carries failures found before parsing.
    pub fn load_from(
        sources: Vec<Source<'_>>,
        mut problems: Vec<Problem>,
    ) -> Result<Catalog, ContentError> {
        let mut documents = BTreeMap::new();

        for source in sources {
            let parsed =
                DocumentPath::parse(&source.path).map_err(|error| error.to_string()).and_then(|path| Document::parse(path, source.text).map_err(|error| error.to_string()));

            match parsed {
                Ok(document) => {
                    documents.insert(document.path.clone(), document);
                },
                Err(message) => problems.push(Problem {
                    file: source.path,
                    message,
                }),
            }
        }

        if problems.is_empty() {
            Ok(Catalog {
                documents,
            })
        } else {
            Err(ContentError {
                problems,
            })
        }
    }

    pub fn find_document(
        &self,
        path: &DocumentPath,
    ) -> Option<&Document> {
        self.documents.get(path)
    }

    /// Every document, in path order.
    pub fn list_documents(&self) -> impl Iterator<Item = &Document> {
        self.documents.values()
    }

    /// The directory at `path`, or `None` when no document is inside it. The root always exists, even when empty.
    pub fn find_directory(
        &self,
        path: &DirectoryPath,
    ) -> Option<Directory<'_>> {
        let prefix = if path.as_str().is_empty() {
            String::new()
        } else {
            format!("{}/", path.as_str())
        };

        let mut documents = Vec::new();
        let mut names = BTreeSet::new();
        for document in self.documents.values() {
            let Some(rest) = document.path.as_str().strip_prefix(&prefix) else {
                continue;
            };
            match rest.split_once('/') {
                Some((name, _)) => {
                    names.insert(name);
                },
                None => documents.push(document),
            }
        }

        if documents.is_empty() && names.is_empty() && *path != DirectoryPath::ROOT {
            return None;
        }

        Some(Directory {
            path: path.clone(),
            documents,
            directories: names.into_iter().map(|name| path.to_child(name)).collect(),
        })
    }

    /// Every directory that holds at least one document, directly or below, with the root first.
    pub fn list_directories(&self) -> Vec<DirectoryPath> {
        let mut directories = BTreeSet::from([DirectoryPath::ROOT]);
        for document in self.documents.values() {
            let mut directory = document.path.to_directory();
            while directories.insert(directory.clone()) {
                match directory.as_str().rsplit_once('/') {
                    Some((parent, _)) => directory = DirectoryPath::parse(parent).unwrap_or(DirectoryPath::ROOT),
                    None => break,
                }
            }
        }

        directories.into_iter().collect()
    }
}

fn collect_sources(
    directory: &'static Dir<'static>,
    sources: &mut Vec<Source<'static>>,
    problems: &mut Vec<Problem>,
) {
    for file in directory.files() {
        let path = file.path().to_string_lossy().replace('\\', "/");
        match file.contents_utf8() {
            Some(text) => sources.push(Source {
                path,
                text,
            }),
            None => problems.push(Problem {
                file: path,
                message: "is not valid UTF-8".to_owned(),
            }),
        }
    }
    for child in directory.dirs() {
        collect_sources(child, sources, problems);
    }
}
