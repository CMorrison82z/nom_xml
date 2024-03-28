pub mod parse;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::parse::*;

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
            </catalog>
        ";

        assert_eq!(
            Xml::from_input_str(data).unwrap(),
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
