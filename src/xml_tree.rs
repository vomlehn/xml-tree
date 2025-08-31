/**
 * Parse XML text input and produce an XML tree
 */

use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};
use xml::name::OwnedName;

use crate::parser::LineNumber;
use crate::walk_and_print::print_walk;
pub use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::{Accumulator, DirectElement, DocumentInfo, Element, ElementInfo, LevelInfo, XmlDocumentFactory};

pub struct XmlTree {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl XmlTree {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        XmlTree {
            document_info,
            root,
        }
    }
}

impl XmlDocumentFactory for XmlTree
{
    type LI = TreeLevelInfo;
    type AC = TreeAccumulator;

/*
    fn accumulator_new(name: OwnedName, element_info: ElementInfo) ->
        Box<dyn Accumulator<Value = <<Self as XmlDocumentFactory>::AC as Accumulator>::Value, Result = <<Self as XmlDocumentFactory>::AC as Accumulator>::Result>>
    {
        Box::new(Self::AC::new(name, element_info))
    }
*/
}

impl fmt::Display for XmlTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        print_walk(f, 0, self)
    }
}

impl fmt::Debug for XmlTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_walk(f, 0, self)
    }
}

impl Try for XmlTree
{
    type Output = <<XmlTree as XmlDocumentFactory>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for XmlTree {
    fn from_residual(_: <XmlTree as Try>::Residual) -> Self
    { todo!() }
}

type XmlTreeResult = Box<dyn Element>;

/**
 * Information for one element in an XML tree
 */
#[derive(Debug)]
pub struct TreeLevelInfo {
}

impl TreeLevelInfo {
    pub fn new() -> Box<TreeLevelInfo> {
        Box::new(TreeLevelInfo {
        })
    }
}

impl LevelInfo for TreeLevelInfo
{
    type Factory = XmlTree;

    fn next(&self) -> Self {
        TreeLevelInfo {
        }
    }

    fn accumulator(&self, name: OwnedName, element_info: ElementInfo) ->
        Box<dyn crate::xml_document_factory::Accumulator<Result = Result<Box<dyn Element + 'static>, XmlDocumentError>, Value = Box<dyn Element + 'static>> + 'static> {
        Box::new(TreeAccumulator::new(name, element_info))
    }
}

/**
 * Information for one element in an XML tree
 * element:         A Boxed value for the Element that we're working on in parse_element().
 * open_subelement: Either None, if we don't have an unclosed Element for Some() if we do.
 */
#[derive(Debug)]
pub struct TreeAccumulator {
    element:            Box<dyn Element>,
    open_subelement:    Option<XmlTreeResult>,
}

impl TreeAccumulator {
    pub fn new(name: OwnedName, element_info: ElementInfo) -> Self {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));

        TreeAccumulator {
            element,
            open_subelement:    None,
        }
    }
}

impl Accumulator for TreeAccumulator
{
    type Value = Box<dyn Element>;
    type Result = Result<Self::Value, XmlDocumentError>;

    fn end(&self) -> Result<Self::Value, XmlDocumentError> {
        Ok(self.element.clone())
    }

    fn in_element(&self) -> bool {
        self.open_subelement.is_some()
    }

    fn start_subelement(&mut self, subelement: Box<dyn Element>) {
        self.open_subelement = Some(subelement);
    }

    fn end_subelement(&mut self) {
        let open_subelement = self.open_subelement().unwrap();
        self.element.subelements_mut().push(open_subelement);
        self.open_subelement = None;
    }

    fn open_subelement(&self) -> Option<Self::Value> {
        self.open_subelement.clone()
    }

    fn name(&self) -> &str {
        self.element.name()
    }

    fn lineno(&self) -> LineNumber {
        self.element.lineno()
    }
}
