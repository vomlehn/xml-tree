/*
 * I don't know what this is supposed to implement
 */

// FIXME: insert banner
// Auto-generated file
use lazy_static::lazy_static;
use std::collections::BTreeMap;

use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;

use crate::xml_document::DirectElement;
use crate::xml_document_factory::{DocumentInfo, ElementInfo};
use crate::xml_schema::XmlSchema;
use crate::XmlDocument;

lazy_static! {
    pub static ref XSD_SCHEMA: XmlSchema<'static> = XmlSchema::new(
        "XSD_SCHEMA",
        "XmlSchema",
        "XsdSchema",
        XmlDocument::new(
            DocumentInfo::new(XmlVersion::Version10, "encoding".to_string(), None),
            Box::new(DirectElement::new(
                OwnedName{local_name: "XsdSchema".to_string(),
                    namespace: None, prefix: None},
                ElementInfo::new(0, vec!(),
                    Namespace(BTreeMap::<String, String>::new())),
                vec!(), vec!(), vec!(),
                vec!(
                    Box::new(DirectElement::new(
                        OwnedName{local_name: "import".to_string(),
                            namespace: None, prefix: None},
                        ElementInfo::new(0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                        )
                    )),
                    Box::new(DirectElement::new(
                        OwnedName{local_name: "annotation".to_string(),
                            namespace: None, prefix: None},
                        ElementInfo::new(0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "documentation".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                )
                            )),
                        )
                    )),
                    Box::new(DirectElement::new(
                        OwnedName{local_name: "element".to_string(),
                            namespace: None, prefix: None},
                        ElementInfo::new(0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "annotation".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "documentation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "key".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "selector".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "field".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                        )
                    )),
                    Box::new(DirectElement::new(
                        OwnedName{local_name: "complexType".to_string(),
                            namespace: None, prefix: None},
                        ElementInfo::new(0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "annotation".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "documentation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "appinfo".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "attribute".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "appinfo".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "simpleType".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "restriction".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "enumeration".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
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
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "choice".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "element".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "key".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "selector".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "field".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
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
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "sequence".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "appinfo".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "element".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "appinfo".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "choice".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "element".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
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
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "simpleContent".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "extension".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "attribute".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "sequence".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "element".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "appinfo".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "choice".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "element".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
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
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "complexContent".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "extension".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "attribute".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                )
                                            )),
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "choice".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "element".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
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
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "sequence".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "annotation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "documentation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "appinfo".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                )
                                                            )),
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "choice".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "choice".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "annotation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                            Box::new(DirectElement::new(
                                                                                OwnedName{local_name: "documentation".to_string(),
                                                                                    namespace: None, prefix: None},
                                                                                ElementInfo::new(0, vec!(),
                                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                                vec!(), vec!(), vec!(),
                                                                                vec!(
                                                                                )
                                                                            )),
                                                                        )
                                                                    )),
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "element".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                            Box::new(DirectElement::new(
                                                                                OwnedName{local_name: "annotation".to_string(),
                                                                                    namespace: None, prefix: None},
                                                                                ElementInfo::new(0, vec!(),
                                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                                vec!(), vec!(), vec!(),
                                                                                vec!(
                                                                                    Box::new(DirectElement::new(
                                                                                        OwnedName{local_name: "documentation".to_string(),
                                                                                            namespace: None, prefix: None},
                                                                                        ElementInfo::new(0, vec!(),
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
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "element".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "annotation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                            Box::new(DirectElement::new(
                                                                                OwnedName{local_name: "documentation".to_string(),
                                                                                    namespace: None, prefix: None},
                                                                                ElementInfo::new(0, vec!(),
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
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "element".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "annotation".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "documentation".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "appinfo".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
                                                                            Namespace(BTreeMap::<String, String>::new())),
                                                                        vec!(), vec!(), vec!(),
                                                                        vec!(
                                                                        )
                                                                    )),
                                                                )
                                                            )),
                                                            Box::new(DirectElement::new(
                                                                OwnedName{local_name: "complexType".to_string(),
                                                                    namespace: None, prefix: None},
                                                                ElementInfo::new(0, vec!(),
                                                                    Namespace(BTreeMap::<String, String>::new())),
                                                                vec!(), vec!(), vec!(),
                                                                vec!(
                                                                    Box::new(DirectElement::new(
                                                                        OwnedName{local_name: "complexContent".to_string(),
                                                                            namespace: None, prefix: None},
                                                                        ElementInfo::new(0, vec!(),
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
                    Box::new(DirectElement::new(
                        OwnedName{local_name: "simpleType".to_string(),
                            namespace: None, prefix: None},
                        ElementInfo::new(0, vec!(),
                            Namespace(BTreeMap::<String, String>::new())),
                        vec!(), vec!(), vec!(),
                        vec!(
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "annotation".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "documentation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                )
                            )),
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "enumeration".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "annotation".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "documentation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                )
                                            )),
                                        )
                                    )),
                                )
                            )),
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "restriction".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
                                    Namespace(BTreeMap::<String, String>::new())),
                                vec!(), vec!(), vec!(),
                                vec!(
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "maxInclusive".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "minInclusive".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "pattern".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                        )
                                    )),
                                    Box::new(DirectElement::new(
                                        OwnedName{local_name: "enumeration".to_string(),
                                            namespace: None, prefix: None},
                                        ElementInfo::new(0, vec!(),
                                            Namespace(BTreeMap::<String, String>::new())),
                                        vec!(), vec!(), vec!(),
                                        vec!(
                                            Box::new(DirectElement::new(
                                                OwnedName{local_name: "annotation".to_string(),
                                                    namespace: None, prefix: None},
                                                ElementInfo::new(0, vec!(),
                                                    Namespace(BTreeMap::<String, String>::new())),
                                                vec!(), vec!(), vec!(),
                                                vec!(
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "documentation".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
                                                            Namespace(BTreeMap::<String, String>::new())),
                                                        vec!(), vec!(), vec!(),
                                                        vec!(
                                                        )
                                                    )),
                                                    Box::new(DirectElement::new(
                                                        OwnedName{local_name: "appinfo".to_string(),
                                                            namespace: None, prefix: None},
                                                        ElementInfo::new(0, vec!(),
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
                            Box::new(DirectElement::new(
                                OwnedName{local_name: "union".to_string(),
                                    namespace: None, prefix: None},
                                ElementInfo::new(0, vec!(),
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
    );
}
