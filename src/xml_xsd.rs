use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};
use xml::name::OwnedName;

use crate::parser::LineNumber;
use crate::walk_and_print::nl_indent;
pub use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::{Accumulator, DirectElement, DocumentInfo, Element, ElementInfo, LevelInfo, XmlDocumentFactory};

pub fn xxx(f: &mut fmt::Formatter<'_>, depth: usize, xml_doc: &XmlXsd) -> fmt::Result
{
    let mut indent_str = nl_indent(depth);
    write!(f, "{}XmlXsd::new(", indent_str)?;

    indent_str = nl_indent(depth + 1);
    let doc_info = &xml_doc.document_info;
    write!(f, "{}DocumentInfo::new(", indent_str)?;
    write!(f, "XmlVersion::Version10, ")?;
    write!(f, "\"{}\".to_string(), ", doc_info.encoding)?;
    write!(f, "{}", if doc_info.standalone.is_none() { "None" }
        else if doc_info.standalone.unwrap() {"true"} else {"false"})?;
    write!(f, "),")?;

/*
    let mut bl = PrintBaseLevel::new(f);
    let ed = PrintElemData::new(depth);
    walk::<PrintAccumulator, PrintBaseLevel, PrintElemData, PrintWalkData, PrintWalkResult>(&mut bl, xml_doc, &ed)?;
    write!(f, "{})", nl_indent(depth))
*/
    Ok(())
}

pub struct XmlXsd {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl XmlXsd {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        XmlXsd {
            document_info,
            root,
        }
    }
}

impl XmlDocumentFactory for XmlXsd
{
    type LI = XsdLevelInfo;
    type AC = XsdAccumulator;
}

impl fmt::Display for XmlXsd {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl fmt::Debug for XmlXsd {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl Try for XmlXsd
{
    type Output = <<XmlXsd as XmlDocumentFactory>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for XmlXsd {
    fn from_residual(_: <XmlXsd as Try>::Residual) -> Self
    { todo!() }
}

type XmlXsdResult = Box<dyn Element>;

/**
 * Information for one element in an XML echo
 */
#[derive(Debug)]
pub struct XsdLevelInfo {
    depth:              usize,
}

impl XsdLevelInfo {
    pub fn new() -> Box<XsdLevelInfo> {
        Box::new(XsdLevelInfo {
            depth:              0,
        })
    }
}

impl LevelInfo for XsdLevelInfo
{
    type Factory = XmlXsd;

    fn next(&self) -> Self {
        XsdLevelInfo {
            depth:              self.depth + 1,
        }
    }

    fn accumulator(&self, name: OwnedName, element_info: ElementInfo) ->
        Box<dyn crate::xml_document_factory::Accumulator<Result = Result<Box<dyn Element + 'static>, XmlDocumentError>, Value = Box<dyn Element + 'static>> + 'static> {
        print!("{}{}", nl_indent(self.depth), name.local_name);
        Box::new(XsdAccumulator::new(name, element_info))
    }
}

/**
 * Information for one element in an XML echo
 * element:         A Boxed value for the Element that we're working on in parse_element().
 * open_subelement: Either None, if we don't have an unclosed Element for Some() if we do.
 */
#[derive(Debug)]
pub struct XsdAccumulator {
    element:            Box<dyn Element>,
    open_subelement:    Option<XmlXsdResult>,
}

impl XsdAccumulator {
    pub fn new(name: OwnedName, element_info: ElementInfo) -> Self {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));

        XsdAccumulator {
            element,
            open_subelement:    None,
        }
    }
}

impl Accumulator for XsdAccumulator
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
