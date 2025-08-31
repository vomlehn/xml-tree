/**
 * Parse XML text input and produce an XML echo
 */

use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};
use xml::name::OwnedName;

use crate::parser::LineNumber;
use crate::walk_and_print::nl_indent;
pub use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::{Accumulator, DirectElement, DocumentInfo, Element, ElementInfo, LevelInfo, XmlDocumentFactory};

pub struct XmlEcho {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl XmlEcho {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        XmlEcho {
            document_info,
            root,
        }
    }
}

impl XmlDocumentFactory for XmlEcho
{
    type LI = EchoLevelInfo;
    type AC = EchoAccumulator;

/*
    fn accumulator_new(name: OwnedName, element_info: ElementInfo) ->
        Box<dyn Accumulator<Value = <<Self as XmlDocumentFactory>::AC as Accumulator>::Value, Result = <<Self as XmlDocumentFactory>::AC as Accumulator>::Result>>
    {
        Box::new(Self::AC::new(name, element_info))
    }
*/
}

impl fmt::Display for XmlEcho {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl fmt::Debug for XmlEcho {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl Try for XmlEcho
{
    type Output = <<XmlEcho as XmlDocumentFactory>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for XmlEcho {
    fn from_residual(_: <XmlEcho as Try>::Residual) -> Self
    { todo!() }
}

type XmlEchoResult = Box<dyn Element>;

/**
 * Information for one element in an XML echo
 */
#[derive(Debug)]
pub struct EchoLevelInfo {
    depth:              usize,
}

impl EchoLevelInfo {
    pub fn new() -> Box<EchoLevelInfo> {
        Box::new(EchoLevelInfo {
            depth:              0,
        })
    }
}

impl LevelInfo for EchoLevelInfo
{
    type Factory = XmlEcho;

    fn next(&self) -> Self {
        EchoLevelInfo {
            depth:              self.depth + 1,
        }
    }

    fn accumulator(&self, name: OwnedName, element_info: ElementInfo) ->
        Box<dyn crate::xml_document_factory::Accumulator<Result = Result<Box<dyn Element + 'static>, XmlDocumentError>, Value = Box<dyn Element + 'static>> + 'static> {
        print!("{}{}", nl_indent(self.depth), name.local_name);
        Box::new(EchoAccumulator::new(name, element_info))
    }
}

/**
 * Information for one element in an XML echo
 * element:         A Boxed value for the Element that we're working on in parse_element().
 * open_subelement: Either None, if we don't have an unclosed Element for Some() if we do.
 */
#[derive(Debug)]
pub struct EchoAccumulator {
    element:            Box<dyn Element>,
    open_subelement:    Option<XmlEchoResult>,
}

impl EchoAccumulator {
    pub fn new(name: OwnedName, element_info: ElementInfo) -> Self {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));

        EchoAccumulator {
            element,
            open_subelement:    None,
        }
    }
}

impl Accumulator for EchoAccumulator
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
