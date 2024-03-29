use std::collections::HashMap;

use nom::error::ErrorKind;

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
    pub value: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Xml {
    Element(Tag, Option<Vec<Xml>>),
    Text(String),
}

impl Xml {
    pub fn from_input_str<'a>(i: &'a str) -> Result<Self, nom::Err<(&str, ErrorKind)>> {
        crate::parse::root::<(&str, ErrorKind)>(i).map(|(_, x)| x)
    }
    // TODO:
    // from_bytes, from_file, etc.
}
