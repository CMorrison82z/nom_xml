use std::collections::HashMap;

use crate::{serialize::*, types::*};

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
                        attributes: HashMap::from(
                            [(String::from("gender"), String::from("Mens")),]
                        ),
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

#[test]
fn duplicate_tags_ok() {
    let data = "
            <?xml version=\"1.0\"?>
            <?xml-stylesheet href=\"prices.xsl\" type=\"text/xsl\"?>
            <prices>
                <price
                    val=\"19.95\"
                    val1=\"9.95\"
                />
                <price val=\"29.95\"/>
                <price val=\"39.95\"/>
                <price val=\"49.95\"/>
            </prices>
        ";

    assert_eq!(
        Xml::from_input_str(data).unwrap(),
        Xml::Element(
            Tag {
                value: "prices".into(),
                attributes: HashMap::new(),
            },
            Some(vec![
                Xml::Element(
                    Tag {
                        value: "price".into(),
                        attributes: HashMap::from([
                            ("val".into(), "19.95".into()),
                            ("val1".into(), "9.95".into())
                        ]),
                    },
                    None
                ),
                Xml::Element(
                    Tag {
                        value: "price".into(),
                        attributes: HashMap::from([("val".into(), "29.95".into())]),
                    },
                    None
                ),
                Xml::Element(
                    Tag {
                        value: "price".into(),
                        attributes: HashMap::from([("val".into(), "39.95".into())]),
                    },
                    None
                ),
                Xml::Element(
                    Tag {
                        value: "price".into(),
                        attributes: HashMap::from([("val".into(), "49.95".into())]),
                    },
                    None
                ),
            ],),
        ),
    );
}

#[test]
fn multi_line_attributes() {
    let data = "
            <?xml version=\"1.0\"?>
            <?xml-stylesheetew href=\"prices.xsl\" type=\"text/xsl\"?>
            <prices>
                <price
                    val = \"9.95\"
                    val1 = \"19.95\"
                />
                <price val=\"29.95\"/>
            </prices>
        ";

    assert_eq!(
        Xml::from_input_str(data).unwrap(),
        Xml::Element(
            Tag {
                value: "prices".into(),
                attributes: HashMap::new(),
            },
            Some(vec![
                Xml::Element(
                    Tag {
                        value: "price".into(),
                        attributes: HashMap::from([
                            ("val".into(), "9.95".into()),
                            ("val1".into(), "19.95".into())
                        ]),
                    },
                    None
                ),
                Xml::Element(
                    Tag {
                        value: "price".into(),
                        attributes: HashMap::from([("val".into(), "29.95".into())]),
                    },
                    None
                ),
            ],),
        ),
    );
}

// FIXME:
// This test only passes half the time, due to the lack of strict
// ordering on the HashMap in `attributes`
#[test]
fn serialize_xml() {
    let data = "<catalog>
    <product description=\"Cardigan Sweater\" product_image=\"cardigan.jpg\">
        <catalog_item gender=\"Mens\">
            <item_number>
                QWZ5671
            </item_number>
            <price>
                39.95
            </price>
            Nice sweater
        </catalog_item>
    </product>
</catalog>";

    assert_eq!(
        String::from(data),
        to_string(Xml::Element(
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
                        attributes: HashMap::from(
                            [(String::from("gender"), String::from("Mens")),]
                        ),
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
        )),
    );
}
