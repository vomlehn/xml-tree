/*
 * I don't know what this is supposed to implement
 */

// FIXME: insert banner
// Auto-generated file
use lazy_static::lazy_static;
//use std::collections::BTreeMap;

//use xml::common::XmlVersion;
//use xml::name::OwnedName;
//use xml::namespace::Namespace;

//use crate::parse_tree::{XmlElement, DocumentInfo, ElementInfo};
//use crate::xml_tree::XmlTree;
use crate::xsd_data::XsdSchema;

lazy_static! {
    pub static ref XSD_SCHEMA: XsdSchema<'static> = XsdSchema::new(
/*
        "XSD_SCHEMA",
        "XsdSchema",
        "XsdSchema",
        XmlTree::new(
            DocumentInfo::new(XmlVersion::Version10, "encoding".to_string(), None),
            Box::new(XmlElement::new(
//ElementInfo::new(0, vec!(),
//    Namespace(BTreeMap::<String, String>::new())),
                ElementInfo::new(
                    OwnedName{local_name: "XsdSchema".to_string(),
                        namespace: None, prefix: None},
                    0, vec!(),
                    Namespace(BTreeMap::<String, String>::new())),
                vec!(), vec!(), vec!(),
                vec!(
                    Box::new(XmlElement::new(
                        ElementInfo::new(
                            OwnedName{local_name: "import".to_string(),
                                namespace: None, prefix: None},
                            0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                        )
                    )),
                    Box::new(XmlElement::new(
                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                            namespace: None, prefix: None},
                        0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                )
                            )),
                        )
                    )),
                    Box::new(XmlElement::new(
                        ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                            namespace: None, prefix: None},
                        0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "key".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "selector".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "field".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                        )
                    )),
                    Box::new(XmlElement::new(
                        ElementInfo::new(
                        OwnedName{local_name: "complexType".to_string(),
                            namespace: None, prefix: None},
                        0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "attribute".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "simpleType".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "restriction".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "enumeration".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "choice".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "key".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "selector".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "field".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "sequence".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "choice".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "simpleContent".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "extension".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "attribute".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "sequence".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "choice".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "complexContent".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "extension".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "attribute".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "choice".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "sequence".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "choice".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "choice".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                            Box::new(XmlElement::new(
                                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                                    namespace: None, prefix: None},
                                                                                0, vec!(),
                                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                                vec!(), vec!(), vec!(),
                                                                                vec!(
                                                                                )
                                                                            )),
                                                                        )
                                                                    )),
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                            Box::new(XmlElement::new(
                                                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                                                    namespace: None, prefix: None},
                                                                                0, vec!(),
                                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                                vec!(), vec!(), vec!(),
                                                                                vec!(
                                                                                    Box::new(XmlElement::new(
                                                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                                            namespace: None, prefix: None},
                                                                                        0, vec!(),
                                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                                        vec!(), vec!(), vec!(),
                                                                                        vec!(
                                                                                        )
                                                                                    )),
                                                                                )
                                                                            )),
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                            Box::new(XmlElement::new(
                                                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                                    namespace: None, prefix: None},
                                                                                0, vec!(),
                                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                                vec!(), vec!(), vec!(),
                                                                                vec!(
                                                                                )
                                                                            )),
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                        OwnedName{local_name: "element".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "appinfo".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                            Box::new(XmlElement::new(
                                                                ElementInfo::new(
                        OwnedName{local_name: "complexType".to_string(),
                                                                    namespace: None, prefix: None},
                                                                0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(XmlElement::new(
                                                                        ElementInfo::new(
                        OwnedName{local_name: "complexContent".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                        )
                    )),
                    Box::new(XmlElement::new(
                        ElementInfo::new(
                        OwnedName{local_name: "simpleType".to_string(),
                            namespace: None, prefix: None},
                        0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "enumeration".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                        OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                        OwnedName{local_name: "restriction".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "maxInclusive".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "minInclusive".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                        OwnedName{local_name: "pattern".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(XmlElement::new(
                                        ElementInfo::new(
                                        OwnedName{local_name: "enumeration".to_string(),
                                            namespace: None, prefix: None},
                                        0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(XmlElement::new(
                                                ElementInfo::new(
                                                    OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                    0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(XmlElement::new(
                                                        ElementInfo::new(
                                                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(XmlElement::new(
                                                            ElementInfo::new(
                                                            OwnedName{local_name: "appinfo".to_string(),
                                                            namespace: None, prefix: None},
                                                        0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(XmlElement::new(
                                ElementInfo::new(
                                OwnedName{local_name: "union".to_string(),
                                    namespace: None, prefix: None},
                                0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                )
                            )),
                        )
                    )),
                )
            )),
        )
*/
    );
}
