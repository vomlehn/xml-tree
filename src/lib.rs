#![feature(try_trait_v2)]

pub mod xml_document;
pub mod xml_document_error;
pub mod xml_schema;

mod multiterator;
mod parser;
mod walkable;
mod walker_print;
mod xml_document_factory;
mod xsd_schema;

pub use crate::walkable::{Accumulator, ElemData, WalkData, WalkError, WalkResult, Walkable};
pub use crate::xml_document::{Element, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
pub use crate::xml_document_factory::XmlDocumentFactory;
pub use crate::xml_schema::{SchemaElement, XmlSchema};
