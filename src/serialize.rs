use crate::types::*;

const INDENTATION: usize = 4;

// TODO: Pretty options
pub fn to_string(x: Xml) -> String {
    match x {
        Xml::Element(ref t @ Tag { ref value, .. }, Some(children)) => format!(
            "{open_tag}\n{serialized_children}\n</{value}>",
            open_tag = tag_to_string(t.clone(), false),
            serialized_children = children
                .into_iter()
                .map(|x| to_string(x)
                    .lines()
                    .map(|l| " ".repeat(INDENTATION) + l)
                    .collect::<Vec<_>>()
                    .join("\n"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        Xml::Element(t, None) => tag_to_string(t, true),
        Xml::Text(s) => s,
    }
}

fn tag_to_string(Tag { value, attributes }: Tag, is_self_closed: bool) -> String {
    let attributes_str = attributes
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "<{}{closing_delim}",
        format!("{value} {attributes_str}").trim(),
        closing_delim = if is_self_closed { "/>" } else { ">" }
    )
}
