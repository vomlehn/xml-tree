#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

pub mod xml_document;
pub mod xml_document_error;
pub mod xml_schema;

pub mod banner;
pub mod document;
pub mod element;
mod misc;
pub mod multiterator; // FIXME: toss this
pub mod parse_doc;
pub mod parse_echo;
pub mod parse_item;
pub mod parse_schema;
pub mod parse_tree;
pub mod parse_xsd;
pub mod walk_tree;
pub mod xsd_data;
pub mod xsd_schema;

pub use crate::banner::set_banner_file_name;
pub use crate::document::DocumentInfo;
pub use crate::element::{Element, ElementInfo};
pub use crate::parse_doc::{Accumulator, ParseDoc};
pub use crate::parse_echo::{EchoAccumulator, EchoLevelInfo, ParseEcho};
pub use crate::parse_schema::{ParseSchema, ParseSchemaParams, SchemaElement, SchemaAccumulator, SchemaLevelInfo};
pub use crate::parse_tree::{ParseTree, TreeElement, TreeAccumulator, TreeLevelInfo};
pub use crate::parse_xsd::{ParseXsd, XsdAccumulator, XsdLevelInfo};
pub use crate::xml_document_error::XmlDocumentError;
pub use crate::xsd_schema::XSD_SCHEMA;
