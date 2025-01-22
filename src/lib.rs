pub mod define;
pub mod parser;
//pub mod xml_tree;
pub mod xml_factory_tree;
pub mod xml_tree_error;

use crate::define::{XmlDescTree, XmlDesc};
pub use crate::xml_factory_tree::XmlFactory;
pub use crate::xml_tree_error::XmlTreeError;

// FIXME: for testing only, remove me
static TEST_XML_DESC_TREE: XmlDescTree = XmlDescTree {
    root:       "a1",
    xml_descs:  & [
        XmlDesc {
            name:   "a1",
            allowable_subelements: & ["a2"],
        },
        XmlDesc {
            name:   "a2",
            allowable_subelements: & ["a1"],
        }
    ]
};
