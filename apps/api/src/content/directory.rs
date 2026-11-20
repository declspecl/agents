use super::{
    DirectoryPath,
    Document,
};

/// A view of one directory: the documents directly inside it and the directories directly inside it.
#[derive(Debug)]
pub struct Directory<'catalog> {
    pub path: DirectoryPath,
    pub documents: Vec<&'catalog Document>,
    pub directories: Vec<DirectoryPath>,
}
