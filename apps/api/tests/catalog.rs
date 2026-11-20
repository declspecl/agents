#![allow(clippy::unwrap_used, reason = "tests and their helpers may unwrap")]

mod support;

use api::content::{
    Catalog,
    DirectoryPath,
    Document,
    DocumentPath,
    Problem,
    Source,
};
use support::{
    document,
    fixture_catalog,
    load,
};

fn path(relative: &str) -> DocumentPath {
    DocumentPath::parse(relative).unwrap()
}

fn directory(relative: &str) -> DirectoryPath {
    DirectoryPath::parse(relative).unwrap()
}

fn problems_of(sources: &[(&str, &str)]) -> Vec<Problem> {
    let sources = sources
        .iter()
        .map(|(path, text)| Source {
            path: (*path).to_owned(),
            text,
        })
        .collect();

    Catalog::load_from(sources, Vec::new()).unwrap_err().problems
}

#[test]
fn finds_a_document_by_path() {
    let catalog = fixture_catalog();
    let found: &Document = catalog.find_document(&path("preferences/editor.md")).unwrap();

    assert_eq!(found.title, "Editor");
    assert_eq!(found.summary, "Editor choices.");
    assert!(found.body.starts_with("# Editor\n"));
    assert!(!found.body.contains("+++"));
    assert!(catalog.find_document(&path("preferences/missing.md")).is_none());
}

#[test]
fn lists_documents_in_path_order() {
    let catalog = fixture_catalog();
    let listed: Vec<_> = catalog.list_documents().map(|document| document.path.as_str()).collect();

    assert_eq!(listed, ["about.md", "preferences/editor.md", "preferences/shell.md", "setup/macos/tools.md"]);
}

#[test]
fn the_root_lists_its_documents_and_top_level_directories() {
    let catalog = fixture_catalog();
    let root = catalog.find_directory(&DirectoryPath::ROOT).unwrap();

    assert_eq!(root.documents.iter().map(|document| document.path.as_str()).collect::<Vec<_>>(), ["about.md"]);
    assert_eq!(root.directories, [directory("preferences"), directory("setup")]);
}

#[test]
fn a_directory_lists_only_what_is_directly_inside_it() {
    let catalog = fixture_catalog();
    let setup = catalog.find_directory(&directory("setup")).unwrap();

    assert!(setup.documents.is_empty());
    assert_eq!(setup.directories, [directory("setup/macos")]);

    let preferences = catalog.find_directory(&directory("preferences")).unwrap();
    assert_eq!(preferences.documents.len(), 2);
    assert!(preferences.directories.is_empty());
}

#[test]
fn a_directory_with_no_documents_does_not_exist() {
    assert!(fixture_catalog().find_directory(&directory("nothing")).is_none());
    // A prefix of a real name is not the directory.
    assert!(fixture_catalog().find_directory(&directory("pref")).is_none());
}

#[test]
fn the_root_exists_even_when_empty() {
    let catalog = load(&[]);

    assert!(catalog.find_directory(&DirectoryPath::ROOT).is_some());
    assert_eq!(catalog.list_directories(), [DirectoryPath::ROOT]);
}

#[test]
fn lists_every_directory_with_the_root_first() {
    let listed: Vec<_> = fixture_catalog().list_directories().iter().map(|directory| directory.to_url_path()).collect();

    assert_eq!(listed, ["/", "/preferences/", "/setup/", "/setup/macos/"]);
}

#[test]
fn reports_every_bad_source_together() {
    let good = document("Good", "Fine.");
    let problems = problems_of(&[
        ("good.md", &good),
        ("Bad Name.md", &good),
        ("no-front-matter.md", "# Just a heading\n"),
        ("unclosed.md", "+++\ntitle = \"x\"\nsummary = \"y\"\n\n# X\n"),
        ("missing-summary.md", "+++\ntitle = \"x\"\n+++\n\n# X\n"),
        ("unknown-field.md", "+++\ntitle = \"x\"\nsummary = \"y\"\nauthor = \"z\"\n+++\n\n# X\n"),
        ("blank-title.md", "+++\ntitle = \"  \"\nsummary = \"y\"\n+++\n\n# X\n"),
        ("no-heading.md", "+++\ntitle = \"x\"\nsummary = \"y\"\n+++\n\nNo heading here.\n"),
    ]);

    let sources: Vec<_> = problems.iter().map(|problem| problem.file.as_str()).collect();
    assert_eq!(
        sources,
        ["Bad Name.md", "no-front-matter.md", "unclosed.md", "missing-summary.md", "unknown-field.md", "blank-title.md", "no-heading.md"]
    );
}

#[test]
fn names_the_reason_for_each_failure() {
    let reasons = |text: &str| problems_of(&[("doc.md", text)])[0].message.clone();

    assert!(reasons("# Heading\n").contains("front matter fence"));
    assert!(reasons("+++\ntitle = \"x\"\nsummary = \"y\"\n\n# X\n").contains("must end with"));
    assert!(reasons("+++\ntitle = \"x\"\n+++\n\n# X\n").contains("summary"));
    assert!(reasons("+++\ntitle = \"x\"\nsummary = \"y\"\n+++\n\ntext\n").contains("level-one heading"));
}

#[test]
fn includes_problems_found_before_parsing() {
    let early = Problem {
        file: "binary.md".to_owned(),
        message: "is not valid UTF-8".to_owned(),
    };
    let error = Catalog::load_from(Vec::new(), vec![early]).unwrap_err();

    assert_eq!(error.problems.len(), 1);
    assert!(error.to_string().contains("binary.md: is not valid UTF-8"));
}

#[test]
fn the_embedded_content_is_valid() {
    let catalog = Catalog::load_embedded().expect("every document under content/ must parse");

    assert!(catalog.list_documents().count() > 0);
    assert!(catalog.find_document(&path("about.md")).is_some());
}
