#[cfg(feature = "secure")]
use std::collections::HashMap;
#[cfg(feature = "fast")]
use foldhash::HashMap;

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
            Xml::Element(t, _) => t.value == name,
            _ => false,
        }
    }

    // TODO:
    // from_bytes, from_file, etc.
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagRef<'a> {
    pub value: &'a str,
    pub attributes: HashMap<&'a str, &'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XmlRef<'a> {
    Element(TagRef<'a>, Option<Vec<XmlRef<'a>>>),
    Text(&'a str),
}

impl<'a> XmlRef<'a> {
    pub fn from_input_str(i: &'a str) -> Result<Self, nom::Err<(&'a str, ErrorKind)>> {
        crate::parse::root_ref::<(&str, ErrorKind)>(i).map(|(_, x)| x)
    }

    pub fn from_tag(t: TagRef<'a>) -> Self {
        XmlRef::Element(t, None)
    }

    pub fn is_element(&self) -> bool {
        match self {
            XmlRef::Element(_, _) => true,
            XmlRef::Text(_) => false,
        }
    }

    pub fn tag_has_name(&self, name: &str) -> bool {
        match self {
            XmlRef::Element(t, _) => t.value == name,
            _ => false,
        }
    }
}

// TODO:
// Better name, and also review the idea.
// pub struct XmlRefHeld<'a> {
//     pub buffer: String,
//     pub xml: XmlRef<'a>
// }
