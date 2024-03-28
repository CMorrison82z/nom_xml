use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_till1, take_until, take_while, take_while1},
    character::complete::{alpha1, char, one_of},
    combinator::{cut, map, opt, value},
    error::{context, ContextError, ParseError},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use std::str;
use std::{collections::HashMap, ops::Deref};

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
    value: String,
    attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Xml {
    Element(Tag, Option<Vec<Xml>>),
    Text(String),
}

fn whitespace<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while(move |c| " \t\r\n".contains(c))(i)
}

fn xml_key<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while1(|c: char| c.is_alphanumeric() || "_-".contains(c))(i)
}

fn xml_text<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    map(take_till1(|c: char| "&<>".contains(c)), |r: &'a str| {
        r.trim()
    })(i)
    // escaped(
    //     take_while1(|c: char| c.is_alphanumeric() || " \t\n\r,.!?;:'\"()[]{}/-_".contains(c)),
    //     '\\',
    //     one_of(r#""n\"#),
    // )(i)
}

fn quote_delim<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, char, E> {
    alt((char('\''), char('\"')))(i)
}

fn attribute_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "attribute_value",
        cut(delimited(
            quote_delim,
            // TODO:
            // Ideally, this should support `"` or `'` depending on the delimiter used...
            escaped(
                take_till1(|c: char| "\'\"".contains(c)),
                '\\',
                one_of(r#""n\"#),
            ),
            quote_delim,
        )),
    )(i)
}

fn attribute_key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, &'a str), E> {
    separated_pair(
        preceded(whitespace, xml_key),
        cut(preceded(whitespace, char('='))),
        attribute_value,
    )
    .parse(i)
}

fn xml_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Xml, E> {
    preceded(
        whitespace,
        alt((map(xml_text, |s| Xml::Text(s.into())), element)),
    )
    .parse(i)
}

fn attributes_hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, HashMap<String, String>, E> {
    context(
        "map",
        map(
            separated_list0(many1(one_of(" \t\r\n")), attribute_key_value),
            |tuple_vec| {
                tuple_vec
                    .into_iter()
                    .map(|(k, v)| (String::from(k), String::from(v)))
                    .collect()
            },
        ),
    )(i)
}

// NOTE:
// This will return the tag.
// We can use the Tag.value to determien the `closing_tag`
fn opening_tag<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (Tag, bool), E> {
    map(
        tuple((
            char('<'),
            map(
                separated_pair(xml_key, whitespace, attributes_hash),
                |(value, attributes)| {
                    let value: String = value.into();
                    Tag { value, attributes }
                },
            ),
            alt((
                value(true, tag("/>")),  // Detect self-closing tags
                value(false, char('>')), // Regular tags
            )),
        )),
        |(_, tag, is_self_closing)| (tag, is_self_closing),
    )(i)
}

// TODO:
// Remove all the `clone`
fn element<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Xml, E> {
    let (remaining, (my_tag, is_self_closing)) = opening_tag(i)?;

    if is_self_closing {
        Ok((remaining, Xml::Element(my_tag, None)))
    } else {
        map(
            terminated(
                many0(xml_value),
                preceded(
                    whitespace,
                    delimited(tag("</"), tag(my_tag.value.clone().deref()), char('>')),
                ),
            ),
            move |vs| {
                Xml::Element(
                    Tag {
                        value: my_tag.value.clone(),
                        attributes: my_tag.attributes.clone(),
                    },
                    Some(vs),
                )
            },
        )
        .parse(remaining)
    }
}

pub fn xml_meta<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    delimited(preceded(whitespace, tag("<?")), is_not(">"), char('>'))(i)
}

// TODO:
// Actually account for (use the) meta data
pub fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Xml, E> {
    cut(preceded(
        many0(xml_meta),
        delimited(opt(whitespace), element, opt(whitespace)),
    ))(i)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn parses_xml() {
        let data = "
<?xml version=\"1.0\"?>
<?xml-stylesheet href=\"catalog.xsl\" type=\"text/xsl\"?>
            <catalog>
   <product description=\"Cardigan Sweater\" product_image=\"cardigan.jpg\">
      <catalog_item gender=\"Mens\">
         <item_number>QWZ5671</item_number>
         <price>39.95</price>
            Nice sweater
      </catalog_item>
   </product>
</catalog>";

        assert_eq!(
            root::<(&str, ErrorKind)>(data).unwrap().1,
            Xml::Element(
                Tag {
                    value: "catalog".into(),
                    attributes: HashMap::new(),
                },
                Some(vec![Xml::Element(
                    Tag {
                        value: "product".into(),
                        attributes: HashMap::from([
                            (
                                String::from("description"),
                                String::from("Cardigan Sweater")
                            ),
                            (String::from("product_image"), String::from("cardigan.jpg"))
                        ]),
                    },
                    Some(vec![Xml::Element(
                        Tag {
                            value: "catalog_item".into(),
                            attributes: HashMap::from([(
                                String::from("gender"),
                                String::from("Mens")
                            ),]),
                        },
                        Some(vec![
                            Xml::Element(
                                Tag {
                                    value: "item_number".into(),
                                    attributes: HashMap::new(),
                                },
                                Some(vec![Xml::Text("QWZ5671".into(),),],),
                            ),
                            Xml::Element(
                                Tag {
                                    value: "price".into(),
                                    attributes: HashMap::new(),
                                },
                                Some(vec![Xml::Text("39.95".into(),),],),
                            ),
                            Xml::Text("Nice sweater".into(),),
                        ],),
                    ),],),
                ),],),
            ),
        );
    }
}
