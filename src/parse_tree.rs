/**
 * Parse XML text input and produce an XML tree
 */

use std::fmt;
use std::io::{BufReader, Read};
use std::ops::{ControlFlow, FromResidual, Try};
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::element::{element_info_display, Element, ElementInfo};
use crate::misc::{nl_indent, owned_name_display, vec_display, XmlDisplay};
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

    pub fn parse_path<'b>(
        path: &'b str,
        element_level_info: &<ParseTree as ParseDoc>::LI,
    ) -> Result<(DocumentInfo, <<<ParseTree as ParseDoc>::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    {
        Self::parse_path_base(path, element_level_info)
    }

    pub fn parse<R>(
        buf_reader: BufReader<R>,
        element_level_info: &<ParseTree as ParseDoc>::LI,
    ) -> Result<(DocumentInfo, <<<ParseTree as ParseDoc>::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
        Self::parse_base(buf_reader, element_level_info)
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
    element: TreeElement,
    current_subelement_name: Option<String>,
}

impl TreeAccumulator {
    pub fn new(element_info: ElementInfo) -> Self {
        let element = TreeElement::new(element_info, vec![], vec![], vec![], vec![]);
        TreeAccumulator {
            element,
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

#[derive(Clone)]
pub struct TreeElement {
    pub element_info: ElementInfo,
    pub before_element: Vec<XmlEvent>,
    pub content: Vec<XmlEvent>,
    pub after_element: Vec<XmlEvent>,
    pub subelements: Vec<Box<dyn Element>>,
}

impl TreeElement {
    pub fn new(element_info: ElementInfo,
        before_element: Vec::<XmlEvent>,
        content: Vec::<XmlEvent>,
        after_element: Vec::<XmlEvent>,
        subelements: Vec<Box<dyn Element>>) -> TreeElement {
        TreeElement {
            element_info,
            subelements,
            before_element,
            content,
            after_element,
        }
    }
}

impl Default for TreeElement {
    fn default() -> TreeElement {
        TreeElement {
            element_info: ElementInfo {
                owned_name: OwnedName {
                    local_name: "".to_string(),
                    namespace:  None,
                    prefix:     None
                },
                lineno:     0,
            },
            subelements: vec!(),
            before_element: vec!(),
            content: vec!(),
            after_element: vec!(),
        }
    }
}

impl fmt::Display for TreeElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl fmt::Debug for TreeElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

impl Element for TreeElement {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}vec!(Box::new(TreeElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let owned_name = OwnedName {
            local_name: self.name().to_string(),
            namespace:  None,
            prefix:     None,
        };
        owned_name_display(f, depth + 1, &owned_name)?;

        let element_info = ElementInfo {
            lineno:     0,
            owned_name: owned_name,
        };
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}", nl_indent(depth + 1))?;
        vec_display::<XmlEvent>(f, depth, &self.before_element)?;
        write!(f, ", ")?;
        vec_display::<XmlEvent>(f, depth, &self.content)?;
        write!(f, ", ")?;
        vec_display::<XmlEvent>(f, depth, &self.after_element)?;
        write!(f, ",")?;
        write!(f, "{}vec!(", nl_indent(depth + 1))
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }

    /**
     * Find a subelement (one level deeper) with the given name
     */
    fn get(&self, name: &str) -> Option<&dyn Element> {
println!("get: looking for {} in {}", name, self.name());
println!("...");
for x in self.subelements() {
    println!(" {}", x);
}
        self.subelements()
            .iter()
            .find(|&x| {
                println!("get: is {} == {}", x.name(), name);
                x.name() == name
            })
            .map(|v| &**v)
    }

    /*
     * Return the element name
     */
    // FIXME: maybe remove this from Element
    fn name(&self) -> &str {
        &self.element_info.owned_name.local_name
    }

    fn lineno(&self) -> LineNumber {
        self.element_info.lineno
    }

    /**
     * Return a vector of all subelements.
     */
    fn subelements<'b>(&'b self) -> &'b Vec<Box<dyn Element + 'static>> {
        &self.subelements
    }

    /**
     * Return a mutable vector of all subelements.
     */
    fn subelements_mut<'b>(&'b mut self) -> &'b mut Vec<Box<dyn Element + 'static>> {
        &mut self.subelements
    }
}

impl XmlDisplay for TreeElement {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}Box::new(TreeElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let element_info = ElementInfo {
            lineno: 0,
            owned_name: OwnedName {
                        local_name: self.name().to_string(),
                        namespace:  None,
                        prefix:     None,
            },
        };

        owned_name_display(f, depth + 1, &element_info.owned_name)?;
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}vec!(), vec!(), vec!(),", nl_indent(depth + 1))?;

        write!(f, "{}vec!(", nl_indent(depth + 1))
    }
}
