// ReExport Yaml, Toml value types. Use features here too

// TODO: real wrapping error
#[derive(Debug)]
struct Error {}

pub enum Format {
    // Maybe use features to allow people to not install all the sub parsers
    // #[cfg(feature = "yaml")]
    Yaml,
    // #[cfg(feature = "toml")]
    Toml,
}

pub struct Document<'a, T> {
    format: Format,
    raw: &'a str,
    frontmatter: T,
    body: &'a str,
}

trait ParseFrontmatter<T> {
    fn parse(input: &str) -> Result<Document<T>, Error>;
}

impl<'a> ParseFrontmatter<yaml_rust::Yaml> for Document<'a, yaml_rust::Yaml> {
    fn parse(_input: &str) -> Result<Document<yaml_rust::Yaml>, Error> {
        unimplemented!();
    }
}

impl<'a> ParseFrontmatter<toml::Value> for Document<'a, toml::Value> {
    fn parse(_input: &str) -> Result<Document<toml::Value>, Error> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use crate::new_lib::{Document, Error, ParseFrontmatter};

    #[test]
    fn test_example_yaml() -> Result<(), Error> {
        let test_string: &str = "---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml";

        let frontmatter: Document<yaml_rust::Yaml> = ParseFrontmatter::<yaml_rust::Yaml>::parse(test_string)?;

        // TODO: write some assertions
        assert!(false);

        Ok(())
    }

    #[test]
    fn test_example_toml() -> Result<(), Error> {
        let test_string = "+++\ntitle = \"Valid Toml Test\"\n+++\nsomething that's not toml";

        let frontmatter: Document<toml::Value> = ParseFrontmatter::<toml::Value>::parse(test_string)?;

        // TODO: write some assertions
        assert!(false);

        Ok(())
    }
}
