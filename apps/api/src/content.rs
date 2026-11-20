//! The content the API serves: markdown documents arranged in a tree.
//!
//! Documents live in `content/` and are embedded into the binary at build time. They are parsed once into a
//! [`Catalog`] at startup, so a malformed document fails startup (and the tests) and never a request. Directories are
//! not stored: they are views the catalog derives from document paths.

mod catalog;
mod directory;
mod document;
mod error;
mod path;

pub use catalog::{
    Catalog,
    Source,
};
pub use directory::Directory;
pub use document::{
    Document,
    DocumentError,
};
pub use error::{
    ContentError,
    Problem,
};
pub use path::{
    DirectoryPath,
    DocumentPath,
    PathError,
};
