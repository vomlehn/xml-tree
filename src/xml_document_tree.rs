/**
 * Parse XML text input and produce an XML tree
 */

//use std::convert::Infallible;
use std::fmt;
use std::io::{BufReader, Read};
//use std::marker::PhantomData;
use std::ops::{ControlFlow, FromResidual, Try};
use xml::name::OwnedName;

use crate::parser::LineNumber;
use crate::walk_and_print::print_walk;
//use crate::walk_and_print::nl_indent;
//use crate::walk_and_print::vec_display;
//use crate::walk_and_print::XmlDisplay;
pub use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::{Accumulator, DirectElement, DocumentInfo, Element, ElementInfo, LevelInfo, XmlDocumentFactory};
//use crate::parser::Parser;

pub type XmlDocument = XmlTreeFactory;

#[derive(Debug)]
pub struct XmlDocumentTree {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl XmlDocumentTree {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        XmlDocumentTree {
            document_info,
            root,
        }
    }
}

pub struct XmlTreeFactory {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl XmlTreeFactory {
//    pub fn new<R>(buf_reader: BufReader<R>, root: Box<dyn Element>) -> XmlTreeFactory
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> XmlTreeFactory
//    where
//        R:  Read,
    {
        XmlTreeFactory {
            document_info,
            root,
        }
/*
        let tree_level_info = TreeLevelInfo::new();
        match <Self as XmlDocumentFactory>::parse::<R>(buf_reader, &tree_level_info) {
            Err(e) => panic!("FIXME: parse failed: {}", e),
            Ok((document_info, root)) => XmlTreeFactory {
                document_info,
                root,
            },
        }
*/
    }
}

impl XmlDocumentFactory for XmlTreeFactory
{
    type LI = TreeLevelInfo;
    type AC = TreeAccumulator;

    fn accumulator_new(name: OwnedName, element_info: ElementInfo) ->
        Box<dyn Accumulator<Value = <<Self as XmlDocumentFactory>::AC as Accumulator>::Value, Result = <<Self as XmlDocumentFactory>::AC as Accumulator>::Result>>
    {
        Box::new(Self::AC::new(name, element_info))
    }

/*
    /**
     * Return an error value
     */
    fn err(e: XmlDocumentError) -> Self::RES {
        Err(e)
    }

    /**
     * Return a success value
     */
    fn ok(document_info: DocumentInfo, top_element: <<Self as XmlDocumentFactory>::AC as Accumulator>::Value) -> Self::RES {
        Ok(XmlTreeFactory::new())
    }
*/
}

impl fmt::Display for XmlTreeFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        print_walk(f, 0, self)
    }
}

impl fmt::Debug for XmlTreeFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_walk(f, 0, self)
    }
}

impl Try for XmlTreeFactory
{
    type Output = <<XmlTreeFactory as XmlDocumentFactory>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for XmlTreeFactory {
    fn from_residual(_: <XmlTreeFactory as Try>::Residual) -> Self
    { todo!() }
}

type XmlTreeResult = Box<dyn Element>;

/**
 * Information for one element in an XML tree
 */
#[derive(Debug)]
pub struct TreeLevelInfo {
/*
    depth:              usize,
*/
}

impl TreeLevelInfo {
    pub fn new() -> Box<TreeLevelInfo> {
        Box::new(TreeLevelInfo {
/*
            depth:              0,
*/
        })
    }
}

impl LevelInfo for TreeLevelInfo
{
/*
    type Value = Box<dyn Element>;
    type Result = Result<Self::Value, XmlDocumentError>;
*/

    fn next(&self) -> Self {
        TreeLevelInfo {
/*
            depth:              self.depth + 1,
*/
        }
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
    fn new(name: OwnedName, element_info: ElementInfo) -> Self {
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

/*
    fn start(&self, document_info: DocumentInfo) -> Self {}
    fn start(&self, name: OwnedName, element_info: ElementInfo) -> Self {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));
        TreeAccumulator {
            open_subelement:    None,
        }
    }
*/

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
