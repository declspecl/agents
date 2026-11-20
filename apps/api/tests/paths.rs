#![allow(clippy::unwrap_used, reason = "tests and their helpers may unwrap")]

use api::content::{
    DirectoryPath,
    DocumentPath,
};

#[test]
fn a_document_path_maps_to_its_url_and_directory() {
    let path = DocumentPath::parse("preferences/code-style.md").unwrap();

    assert_eq!(path.to_url_path(), "/preferences/code-style.md");
    assert_eq!(path.to_directory(), DirectoryPath::parse("preferences").unwrap());
}

#[test]
fn a_top_level_document_lives_in_the_root() {
    assert_eq!(DocumentPath::parse("about.md").unwrap().to_directory(), DirectoryPath::ROOT);
}

#[test]
fn a_directory_path_maps_to_its_url_with_a_trailing_slash() {
    assert_eq!(DirectoryPath::ROOT.to_url_path(), "/");
    assert_eq!(DirectoryPath::parse("setup/macos").unwrap().to_url_path(), "/setup/macos/");
}

#[test]
fn a_directory_names_its_children() {
    let child = DirectoryPath::ROOT.to_child("setup").to_child("macos");

    assert_eq!(child.as_str(), "setup/macos");
    assert_eq!(child.to_name(), Some("macos"));
    assert_eq!(DirectoryPath::ROOT.to_name(), None);
}

#[test]
fn rejects_document_paths_that_are_not_markdown_names() {
    for path in [
        "about",
        "about.txt",
        "About.md",
        "has space.md",
        "-leading.md",
        "trailing-.md",
        "double--hyphen.md",
        ".md",
        "/absolute.md",
        "a//b.md",
        "../escape.md",
        "dir/.md",
        "Upper/doc.md",
    ] {
        assert!(DocumentPath::parse(path).is_err(), "{path:?} should be rejected");
    }
}

#[test]
fn rejects_directory_paths_with_bad_names_or_slashes() {
    for path in ["/setup", "setup/", "Setup", "a//b", "..", "has space"] {
        assert!(DirectoryPath::parse(path).is_err(), "{path:?} should be rejected");
    }
}
