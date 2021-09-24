// ReExport Yaml, Toml value types. Use features here too

enum Format {
    // Maybe use features to allow people to not install all the sub parsers
    // #[cfg(feature = "yaml")]
    Yaml,
    // #[cfg(feature = "toml")]
    Toml,
}

// Detect frontmatter format
// For now could be as simple as `---` fence is Yaml and `+++` fence is Toml.
pub fn detect_format(input: &str) -> Option<Format> {
    unimplemented!();
}

// Gets just the file content, without caring about the front matter
pub fn get_content(input: &str) -> &str {
    unimplemented!();
}

// Not quite sure how the front matter should be sent back across different parsers
struct Frontmatter<T> {
    format: Format,
    raw: &str,
    value: T,
}

// May need to implement specific wrapping error for this library
pub fn parse<T: Format>(input: &str) -> Result<(Option<Frontmatter>, Option<&str>), Error> {
    unimplemented();
}

#[test]
fn test_example_yaml() {
    let test_string = "---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml";
    let matter = parse::<Format::Yaml>(test_string)?;

    // TODO: write some assertions
    assert!(false);
}

#[test]
fn test_example_toml() {
    let test_string = "+++\ntitle = \"Valid Toml Test\"\n+++\nsomething that's not toml";
    let matter = parse::<Format::Toml>(test_string)?;

    // TODO: write some assertions
    assert!(false);
}