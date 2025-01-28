pub mod xml_definition;
pub mod xml_document;
pub mod xml_document_error;

mod parser;
mod xml_document_factory;
mod xsd_schema;

pub use crate::xml_definition::{XmlDefinition, ElementDefinition};
pub use crate::xml_document::{Element, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
