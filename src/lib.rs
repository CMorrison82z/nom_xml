use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while, take_while1},
    character::complete::{alpha1, char, one_of},
    combinator::{cut, map, opt, value},
    error::{context, ContextError, ParseError},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use std::str;
use std::{collections::HashMap, ops::Deref};

#[derive(Debug)]
pub struct Tag {
    value: String,
    attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Xml {
    Element(Tag, Option<Vec<Xml>>),
    Text(String),
}

fn whitespace<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while(move |c| " \t\r\n".contains(c))(i)
}

fn xml_text<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(
        take_while1(|c: char| c.is_alphanumeric() || " \t\n\r,.!?;:'\"()[]{}/-".contains(c)),
        '\\',
        one_of(r#""n\"#),
    )(i)
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
                take_while(|c: char| c.is_alphanumeric() || " ,.!?;:()[]{}<>/-".contains(c)),
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
        preceded(whitespace, alpha1),
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
                separated_pair(alpha1, whitespace, attributes_hash),
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
                preceded(whitespace, delimited(tag("</"), tag(my_tag.value.clone().deref()), char('>'))),
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
    delimited(preceded(whitespace, char('<')), is_not(">"), char('>'))(i)
}

// TODO:
// Actually account for (use the) meta data
pub fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Xml, E> {
    cut(preceded(xml_meta, delimited(opt(whitespace), element, opt(whitespace))))(i)
}
