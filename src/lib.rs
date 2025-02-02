pub mod xml_schema;
pub mod xml_document;
pub mod xml_document_error;

mod parser;
mod xml_document_factory;
mod xsd_schema;

pub use crate::xml_schema::{XmlSchema, SchemaElement};
pub use crate::xml_document::{Element, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
