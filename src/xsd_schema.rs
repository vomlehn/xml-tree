/*
 * This is the parsed scheme for XSD files.
 */
use lazy_static::lazy_static;
//use std::sync::Arc;

use crate::xml_schema::{DirectElement, XmlSchema};

lazy_static! {
    pub static ref XSD_SCHEMA: XmlSchema<'static> = XmlSchema::new(
        "XsdSchema",
        Box::new(DirectElement::new(
            "schema",
            vec!()
        ))
    );
}

/*
lazy_static! {
    pub static ref XSD_SCHEMA: XmlSchema<'static> = XmlSchema::new(
        "XsdSchema",
        Arc::new(DirectElement::new(
            "schema",
            vec!(
                Arc::new(DirectElement::new("import", vec!())),
                Arc::new(DirectElement::new(
                    "annotation",
                    vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                )),
                Arc::new(DirectElement::new(
                    "element",
                    vec!(
                        Arc::new(DirectElement::new(
                            "annotation",
                            vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                        )),
                        Arc::new(DirectElement::new(
                            "key",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                                )),
                                Arc::new(DirectElement::new("selector", vec!())),
                                Arc::new(DirectElement::new("field", vec!())),
                            )
                        )),
                    )
                )),
                Arc::new(DirectElement::new(
                    "complexType",
                    vec!(
                        Arc::new(DirectElement::new(
                            "annotation",
                            vec!(
                                Arc::new(DirectElement::new("documentation", vec!())),
                                Arc::new(DirectElement::new("appinfo", vec!())),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "attribute",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(
                                        Arc::new(DirectElement::new("documentation", vec!())),
                                        Arc::new(DirectElement::new("appinfo", vec!())),
                                    )
                                )),
                                Arc::new(DirectElement::new(
                                    "simpleType",
                                    vec!(Arc::new(DirectElement::new(
                                        "restriction",
                                        vec!(Arc::new(DirectElement::new(
                                            "enumeration",
                                            vec!(Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(Arc::new(DirectElement::new(
                                                    "documentation",
                                                    vec!()
                                                )),)
                                            )),)
                                        )),)
                                    )),)
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "choice",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                                )),
                                Arc::new(DirectElement::new(
                                    "element",
                                    vec!(
                                        Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(Arc::new(DirectElement::new(
                                                "documentation",
                                                vec!()
                                            )),)
                                        )),
                                        Arc::new(DirectElement::new(
                                            "key",
                                            vec!(
                                                Arc::new(DirectElement::new(
                                                    "annotation",
                                                    vec!(Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),)
                                                )),
                                                Arc::new(DirectElement::new("selector", vec!())),
                                                Arc::new(DirectElement::new("field", vec!())),
                                            )
                                        )),
                                    )
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "sequence",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "annotation",
                                    vec!(
                                        Arc::new(DirectElement::new("documentation", vec!())),
                                        Arc::new(DirectElement::new("appinfo", vec!())),
                                    )
                                )),
                                Arc::new(DirectElement::new(
                                    "element",
                                    vec!(Arc::new(DirectElement::new(
                                        "annotation",
                                        vec!(
                                            Arc::new(DirectElement::new("documentation", vec!())),
                                            Arc::new(DirectElement::new("appinfo", vec!())),
                                        )
                                    )),)
                                )),
                                Arc::new(DirectElement::new(
                                    "choice",
                                    vec!(
                                        Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(Arc::new(DirectElement::new(
                                                "documentation",
                                                vec!()
                                            )),)
                                        )),
                                        Arc::new(DirectElement::new(
                                            "element",
                                            vec!(Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),
                                                    Arc::new(DirectElement::new("appinfo", vec!())),
                                                )
                                            )),)
                                        )),
                                    )
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "simpleContent",
                            vec!(Arc::new(DirectElement::new(
                                "extension",
                                vec!(Arc::new(DirectElement::new("attribute", vec!())),)
                            )),)
                        )),
                        Arc::new(DirectElement::new(
                            "sequence",
                            vec!(
                                Arc::new(DirectElement::new(
                                    "element",
                                    vec!(Arc::new(DirectElement::new(
                                        "annotation",
                                        vec!(
                                            Arc::new(DirectElement::new("documentation", vec!())),
                                            Arc::new(DirectElement::new("appinfo", vec!())),
                                        )
                                    )),)
                                )),
                                Arc::new(DirectElement::new(
                                    "choice",
                                    vec!(
                                        Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(Arc::new(DirectElement::new(
                                                "documentation",
                                                vec!()
                                            )),)
                                        )),
                                        Arc::new(DirectElement::new(
                                            "element",
                                            vec!(Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),
                                                    Arc::new(DirectElement::new("appinfo", vec!())),
                                                )
                                            )),)
                                        )),
                                    )
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new(
                            "complexContent",
                            vec!(Arc::new(DirectElement::new(
                                "extension",
                                vec!(
                                    Arc::new(DirectElement::new(
                                        "attribute",
                                        vec!(Arc::new(DirectElement::new(
                                            "annotation",
                                            vec!(
                                                Arc::new(DirectElement::new(
                                                    "documentation",
                                                    vec!()
                                                )),
                                                Arc::new(DirectElement::new("appinfo", vec!())),
                                            )
                                        )),)
                                    )),
                                    Arc::new(DirectElement::new(
                                        "choice",
                                        vec!(
                                            Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(Arc::new(DirectElement::new(
                                                    "documentation",
                                                    vec!()
                                                )),)
                                            )),
                                            Arc::new(DirectElement::new(
                                                "element",
                                                vec!(Arc::new(DirectElement::new(
                                                    "annotation",
                                                    vec!(Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),)
                                                )),)
                                            )),
                                        )
                                    )),
                                    Arc::new(DirectElement::new(
                                        "sequence",
                                        vec!(
                                            Arc::new(DirectElement::new(
                                                "annotation",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "documentation",
                                                        vec!()
                                                    )),
                                                    Arc::new(DirectElement::new("appinfo", vec!())),
                                                )
                                            )),
                                            Arc::new(DirectElement::new(
                                                "choice",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "annotation",
                                                        vec!(Arc::new(DirectElement::new(
                                                            "documentation",
                                                            vec!()
                                                        )),)
                                                    )),
                                                    Arc::new(DirectElement::new(
                                                        "choice",
                                                        vec!(
                                                            Arc::new(DirectElement::new(
                                                                "annotation",
                                                                vec!(Arc::new(DirectElement::new(
                                                                    "documentation",
                                                                    vec!()
                                                                )),)
                                                            )),
                                                            Arc::new(DirectElement::new(
                                                                "element",
                                                                vec!(Arc::new(DirectElement::new(
                                                                    "annotation",
                                                                    vec!(Arc::new(
                                                                        DirectElement::new(
                                                                            "documentation",
                                                                            vec!()
                                                                        )
                                                                    ),)
                                                                )),)
                                                            )),
                                                        )
                                                    )),
                                                    Arc::new(DirectElement::new(
                                                        "element",
                                                        vec!(Arc::new(DirectElement::new(
                                                            "annotation",
                                                            vec!(Arc::new(DirectElement::new(
                                                                "documentation",
                                                                vec!()
                                                            )),)
                                                        )),)
                                                    )),
                                                )
                                            )),
                                            Arc::new(DirectElement::new(
                                                "element",
                                                vec!(
                                                    Arc::new(DirectElement::new(
                                                        "annotation",
                                                        vec!(
                                                            Arc::new(DirectElement::new(
                                                                "documentation",
                                                                vec!()
                                                            )),
                                                            Arc::new(DirectElement::new(
                                                                "appinfo",
                                                                vec!()
                                                            )),
                                                        )
                                                    )),
                                                    Arc::new(DirectElement::new(
                                                        "complexType",
                                                        vec!(Arc::new(DirectElement::new(
                                                            "complexContent",
                                                            vec!()
                                                        )),)
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),)
                        )),
                    )
                )),
                Arc::new(DirectElement::new(
                    "simpleType",
                    vec!(
                        Arc::new(DirectElement::new(
                            "annotation",
                            vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                        )),
                        Arc::new(DirectElement::new(
                            "enumeration",
                            vec!(Arc::new(DirectElement::new(
                                "annotation",
                                vec!(Arc::new(DirectElement::new("documentation", vec!())),)
                            )),)
                        )),
                        Arc::new(DirectElement::new(
                            "restriction",
                            vec!(
                                Arc::new(DirectElement::new("maxInclusive", vec!())),
                                Arc::new(DirectElement::new("minInclusive", vec!())),
                                Arc::new(DirectElement::new("pattern", vec!())),
                                Arc::new(DirectElement::new(
                                    "enumeration",
                                    vec!(Arc::new(DirectElement::new(
                                        "annotation",
                                        vec!(
                                            Arc::new(DirectElement::new("documentation", vec!())),
                                            Arc::new(DirectElement::new("appinfo", vec!())),
                                        )
                                    )),)
                                )),
                            )
                        )),
                        Arc::new(DirectElement::new("union", vec!())),
                    )
                )),
            )
        )),
    );
}
*/
