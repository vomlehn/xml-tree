#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

pub mod xml_document;
pub mod xml_document_error;
pub mod xml_schema;

mod banner;
mod document;
mod element;
mod multiterator; // FIXME: toss this
mod parse_doc;
mod parse_echo;
mod parse_item;
mod parse_tree;
mod walk_print;
mod walk_tree;
mod xsd_data;
mod xsd_schema;

pub use crate::banner::set_banner_file_name;
pub use crate::document::DocumentInfo;
pub use crate::element::Element;
pub use crate::parse_doc::{Accumulator, ParseDoc};
pub use crate::parse_echo::{EchoAccumulator, XmlEcho, EchoLevelInfo};
pub use crate::parse_tree::{ParseTree, TreeAccumulator, TreeLevelInfo};
pub use crate::xml_document_error::XmlDocumentError;
pub use crate::xsd_schema::XSD_SCHEMA;
