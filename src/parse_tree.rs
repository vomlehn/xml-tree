/**
 * Parse XML text input and produce an XML tree
 */

use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};

use crate::element::{DirectElement, Element, ElementInfo};
use crate::parse_item::LineNumber;
pub use crate::xml_document_error::XmlDocumentError;
use crate::parse_doc::{Accumulator, LevelInfo, ParseDoc};
use crate::document::DocumentInfo;

pub struct ParseTree {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl ParseTree {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        ParseTree {
            document_info,
            root,
        }
    }
}

impl ParseDoc for ParseTree {
    type LI = TreeLevelInfo;
    type AC = TreeAccumulator;
    // No AC type needed anymore
}

impl LevelInfo for TreeLevelInfo {
    type AccumulatorType = TreeAccumulator;

    fn next_level(&self) -> Self {
        TreeLevelInfo
    }

    fn create_accumulator(&self, element_info: ElementInfo) ->
        Result<TreeAccumulator, XmlDocumentError>
    {
        Ok(TreeAccumulator::new(element_info))
    }
}

impl fmt::Display for ParseTree {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
//        print_walk(f, 0, self)
    }
}

impl fmt::Debug for ParseTree {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
//        print_walk(f, 0, self)
    }
}

impl Try for ParseTree
{
    type Output = <<ParseTree as ParseDoc>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for ParseTree {
    fn from_residual(_: <ParseTree as Try>::Residual) -> Self
    { todo!() }
}

/// LevelInfo that doesn't track depth - just creates tree nodes
#[derive(Debug, Clone)]
pub struct TreeLevelInfo;

impl TreeLevelInfo {
    pub fn new() -> Self {
        TreeLevelInfo
    }
}

/// Accumulator that builds actual element tree
pub struct TreeAccumulator {
    element: DirectElement,
    current_subelement_name: Option<String>,
}

impl TreeAccumulator {
    pub fn new(element_info: ElementInfo) -> Self {
        TreeAccumulator {
            element: DirectElement::new(element_info, vec![], vec![], vec![], vec![]),
            current_subelement_name: None,
        }
    }
}

impl Accumulator for TreeAccumulator {
    type Value = Box<dyn Element>;

    fn start_subelement(&mut self, element_info: &ElementInfo) {
        // We'll set the name when we get the actual subelement
        self.current_subelement_name = Some(element_info.owned_name.local_name.clone());

    }
    
    fn add_subelement(&mut self, subelement: Box<dyn Element>) {
        self.current_subelement_name = Some(subelement.name().to_string());
        self.element.subelements_mut().push(subelement);
    }
    
    fn end_subelement(&mut self) {
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
    
    fn finish(self) -> Box<dyn Element> {
        Box::new(self.element)
    }
    
    fn element_name(&self) -> &str {
        self.element.name()
    }
    
    fn element_lineno(&self) -> LineNumber {
        self.element.lineno()
    }
}
