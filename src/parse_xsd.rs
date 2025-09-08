/**
 * Parse XML text input and produce an XML echo
 */

use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};

use crate::element::{Element, ElementInfo};
use crate::parse_item::LineNumber;
pub use crate::xml_document_error::XmlDocumentError;
use crate::parse_doc::{Accumulator, LevelInfo, ParseDoc};
use crate::document::DocumentInfo;

pub struct ParseXsd {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl ParseXsd {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        ParseXsd {
            document_info,
            root,
        }
    }
}

impl ParseDoc for ParseXsd {
    type LI = XsdLevelInfo;
    type AC = XsdAccumulator;
}

impl fmt::Display for ParseXsd {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl fmt::Debug for ParseXsd {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl Try for ParseXsd
{
    type Output = <<ParseXsd as ParseDoc>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for ParseXsd {
    fn from_residual(_: <ParseXsd as Try>::Residual) -> Self
    { todo!() }
}

/// LevelInfo that tracks depth for indented output
#[derive(Debug, Clone)]
pub struct XsdLevelInfo {
    depth:      usize,
    element:    Box<dyn Element>,
}

impl XsdLevelInfo {
    pub fn new(root: &Box<dyn Element>) -> Self {
        XsdLevelInfo {
            depth:      0,
            element:    root.clone(),
        }
    }
}

impl LevelInfo for XsdLevelInfo {
    type AccumulatorType = XsdAccumulator;

    fn next_level(&self) -> Self {
        XsdLevelInfo {
            depth:      self.depth + 1,
            element:    self.element.subelements()[0].clone(),
        }
    }

    fn create_accumulator(&self, element_info: ElementInfo) ->
        Result<XsdAccumulator, XmlDocumentError>
    {
        println!("{}<{}>", "  ".repeat(self.depth), element_info.owned_name.local_name);
        Ok(XsdAccumulator::new(element_info, self.depth))
    }
}

/// Accumulator that just echoes structure (doesn't build elements)
pub struct XsdAccumulator {
    element_name: String,
    element_lineno: LineNumber,
    depth: usize,
    current_subelement_name: Option<String>,
}

impl XsdAccumulator {
    pub fn new(element_info: ElementInfo, depth: usize) -> Self {
        XsdAccumulator {
            element_name: element_info.owned_name.local_name.clone(),
            element_lineno: element_info.lineno,
            depth,
            current_subelement_name: None,
        }
    }
}

impl Accumulator for XsdAccumulator {
    type Value = ();  // Xsd doesn't return meaningful data

    fn start_subelement(&mut self, _element_info: &ElementInfo) {
        // Nothing special needed
    }
    
    fn add_subelement(&mut self, _subelement: ()) {
        // For echo, subelements have already been printed
        // We don't need to do anything with the () value
    }
    
    fn end_subelement(&mut self) {
        if let Some(name) = &self.current_subelement_name {
            println!("{}</{}>", "  ".repeat(self.depth + 1), name);
        }
        self.current_subelement_name = None;
    }
    
    fn has_open_subelement(&self) -> bool {
        self.current_subelement_name.is_some()
    }
    
    fn current_subelement_name(&self) -> &str {
        self.current_subelement_name.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("")
    }
    
    fn finish(self) -> () {
        println!("{}</{}>", "  ".repeat(self.depth), self.element_name);
        ()
    }
    
    fn element_name(&self) -> &str {
        &self.element_name
    }
    
    fn element_lineno(&self) -> LineNumber {
        self.element_lineno
    }
}
