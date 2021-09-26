#![allow(dead_code)]
extern crate toml;

pub use toml::Value;

fn find_toml_block(text: &str) -> Option<(usize, usize, usize)> {
    match text.starts_with("+++\n") {
        true => {
            let slice_after_marker = &text[4..];
            let fm_end = slice_after_marker.find("+++\n");
            if fm_end.is_none() {
                return None;
            };

            let fm_end = fm_end.unwrap();
            Some((4, fm_end + 4, fm_end + 2 * 4))
        }
        false => None,
    }
}

pub fn parse_and_find_content(text: &str) -> Result<(Option<toml::Value>, &str), toml::de::Error> {
    match find_toml_block(text) {
        Some((fm_start, fm_end, content_start)) => {
            let toml_str = &text[fm_start..fm_end];
            let frontmatter = toml_str.parse::<toml::Value>()?;

            let rest_of_text = &text[content_start..];

            Ok((Some(frontmatter), rest_of_text))
        }
        None => Ok((None, text)),
    }
}

pub fn parse(text: &str) -> Result<Option<toml::Value>, toml::de::Error> {
    let (matter, _) = parse_and_find_content(text)?;
    Ok(matter)
}

#[test]
fn test_valid() {
    let test_string = "+++\ntitle = \"Valid Toml Test\"\n+++\nsomething that's not toml";

    println!("Original String{:?}", test_string);
    let matter = parse(test_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_ok());
    let matter = matter.unwrap().unwrap();
    assert_eq!(&matter["title"].as_str().unwrap(), &"Valid Toml Test");
}

#[test]
fn test_valid_find_content() {
    let test_string = "+++\ntitle = \"Valid Toml Test\"\n+++\nsomething that's not toml";

    println!("Original String{:?}", test_string);
    let result = parse_and_find_content(test_string);
    assert!(result.is_ok());
    let (matter, stripped_string) = result.unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_some());
    assert!(stripped_string.to_string() == test_string[34..].to_string());
}

#[test]
fn test_none() {
    let test_string = "something that's not toml even if it has\n+++\nsome = \"toml\"\n--";

    println!("Original String{:?}", test_string);
    let matter = parse(test_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_ok());
    let matter = matter.unwrap();
    assert!(matter.is_none());
}

#[test]
fn test_none_find_content() {
    let test_string = "something that's not toml even if it has\n+++\nsome = \"toml\"\n--";

    println!("Original String{:?}", test_string);
    let result = parse_and_find_content(test_string);
    assert!(result.is_ok());
    let (matter, stripped_string) = result.unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_none());
    assert!(stripped_string.to_string() == test_string);
}

#[test]
fn test_empty() {
    let test_string = "+++\n+++\nsomething that's not toml";

    println!("Original String{:?}", test_string);
    let matter = parse(test_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_ok());
    let matter = matter.unwrap();
    assert!(matter.is_some());
}

#[test]
fn test_empty_find_content() {
    let test_string = "+++\n+++\nsomething that's not toml";

    println!("Original String{:?}", test_string);
    let result = parse_and_find_content(test_string);
    assert!(result.is_ok());
    let (matter, stripped_string) = result.unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_some());
    assert!(stripped_string.to_string() == test_string[8..]);
}

#[test]
fn test_tests() {
    assert_eq!(2 + 2, 4);
}
