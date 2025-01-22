pub mod xml_definition;
pub mod xml_document;
pub mod xml_document_error;

mod parser;
mod xml_document_factory;

/*
use crate::xml_definition::{XmlDefinition, ElementDefinition};
pub use crate::xml_document_factory::XmlDocumentFactory;
pub use crate::xml_document_error::XmlDocumentError;
*/

/*
// FIXME: for testing only, remove me
static TEST_XML_DESC_TREE: XmlDefinition = XmlDefinition {
    root:       "a1",
    element_definitions:  & [
        ElementDefinition {
            name:   "a1",
            allowable_subelements: & ["a2"],
        },
        ElementDefinition {
            name:   "a2",
            allowable_subelements: & ["a1"],
        }
    ]
};
*/
