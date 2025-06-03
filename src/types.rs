use std::collections::HashMap;

use nom::error::ErrorKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag {
    pub value: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Xml {
    Element(Tag, Option<Vec<Xml>>),
    Text(String),
}

impl Xml {
    pub fn from_tag(t: Tag) -> Self {
        Xml::Element(t, None)
    }

    pub fn from_input_str<'a>(i: &'a str) -> Result<Self, nom::Err<(&'a str, ErrorKind)>> {
        crate::parse::root::<(&str, ErrorKind)>(i).map(|(_, x)| x)
    }

    pub fn is_element(&self) -> bool {
        match self {
            Xml::Element(_, _) => true,
            Xml::Text(_) => false,
        }
    }

    pub fn tag_has_name(&self, name: &str) -> bool {
        match self {
            Xml::Element(t, _) if t.value == name => true,
            _ => false,
        }
    }

    // TODO:
    // from_bytes, from_file, etc.
}
