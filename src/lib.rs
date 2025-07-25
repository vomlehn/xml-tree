#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

pub mod xml_document;
pub mod xml_document_error;
pub mod xml_schema;

mod t;
mod banner;
mod multiterator; // FIXME: toss this
mod parser;
mod walkable;
mod walk_and_print;
mod xml_document_factory;
mod xml_tree_element;
mod xsd_schema;

pub use crate::banner::set_banner_file_name;
pub use crate::walkable::{Accumulator, ElemData, WalkData/*, Walkable*/};
pub use crate::xml_document::{Element, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
pub use crate::xml_document_factory::XmlDocumentFactory;
pub use crate::xml_tree_element::XmlTreeElement;
pub use crate::xml_schema::XmlSchema;
pub use crate::xsd_schema::XSD_SCHEMA;
