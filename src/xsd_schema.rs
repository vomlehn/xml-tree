/*
 * This is the parsed scheme for XSD files.
 */
use lazy_static::lazy_static;
//use std::cell::RefCell;
use std::collections::BTreeMap;
use xml::namespace::Namespace;
use xml::common::XmlVersion;
use xml::name::OwnedName;

use crate::xml_schema::XmlSchema;
use crate::xml_document::XmlDocument;
use crate::xml_document_factory::{DirectElement, DocumentInfo, ElementInfo};
//use crate::walk_and_print::PrintBaseLevel;

lazy_static! {
    pub static ref XSD_SCHEMA: XmlSchema<'static> = XmlSchema::new(
        "XSD_SCHEMA",
        "XsdSchema",
        XmlDocument::new(
            DocumentInfo::new(XmlVersion::Version10, "xxx".to_string(),
                 None
            ),
            Box::new(
                DirectElement::new(OwnedName {
                        local_name: "XsdSchema".to_string(),
                        namespace:  None,
                        prefix:     None,
                    }, ElementInfo::new(0, Vec::new(), Namespace(
                            BTreeMap::<String, String>::new()
                        )
                    ),
                    vec!(Box::new(DirectElement::new(OwnedName {
                                local_name: "level2element1".to_string(),
                                namespace:  None,
                                prefix:     None,
                            }, ElementInfo::new(0, Vec::new(), Namespace(
                                    BTreeMap::<String, String>::new()
                                )
                            ),
                            vec!(),
                        )),
                        Box::new(DirectElement::new(OwnedName {
                                local_name: "level2element2".to_string(),
                                namespace:  None,
                                prefix:     None,
                            }, ElementInfo::new(0, Vec::new(), Namespace(
                                    BTreeMap::<String, String>::new()
                                )
                            ),
                            vec!(
                                Box::new(DirectElement::new(OwnedName {
                                        local_name: "level3element1".to_string(),
                                        namespace:  None,
                                        prefix:     None,
                                    }, ElementInfo::new(0, Vec::new(), Namespace(
                                            BTreeMap::<String, String>::new()
                                        )
                                    ),
                                    vec!(),
                                )),
                            ),
                        )),
                        Box::new(DirectElement::new(OwnedName {
                                local_name: "level2element3".to_string(),
                                namespace:  None,
                                prefix:     None,
                            }, ElementInfo::new(0, Vec::new(), Namespace(
                                    BTreeMap::<String, String>::new()
                                )
                            ),
                            vec!(),
                        )),
                    ),
                ),
            ),
        ),
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
