/**
 * Parse XML text input and produce an XML echo
 */

use std::fmt;
use std::io::{BufReader, Read};
use std::ops::{ControlFlow, FromResidual, Try};

use crate::element::{Element, ElementInfo};
use crate::misc::nl_indent;
use crate::parse_item::LineNumber;
pub use crate::xml_document_error::XmlDocumentError;
use crate::parse_doc::{Accumulator, LevelInfo, ParseDoc};
use crate::document::DocumentInfo;

pub struct ParseEcho {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl ParseEcho {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        ParseEcho {
            document_info,
            root,
        }
    }

    pub fn parse_path<'b>(
        path: &'b str,
        element_level_info: &<ParseEcho as ParseDoc>::LI,
    ) -> Result<(DocumentInfo, <<<ParseEcho as ParseDoc>::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    {
        Self::parse_path_base(path, element_level_info)
    }

    pub fn parse<R>(
        buf_reader: BufReader<R>,
        element_level_info: &<ParseEcho as ParseDoc>::LI,
    ) -> Result<(DocumentInfo, <<<ParseEcho as ParseDoc>::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
        Self::parse_base(buf_reader, element_level_info)
    }
}

impl ParseDoc for ParseEcho {
    type LI = EchoLevelInfo;
    type AC = EchoAccumulator;
}

impl fmt::Display for ParseEcho {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl fmt::Debug for ParseEcho {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl Try for ParseEcho
{
    type Output = <<ParseEcho as ParseDoc>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for ParseEcho {
    fn from_residual(_: <ParseEcho as Try>::Residual) -> Self
    { todo!() }
}

/// LevelInfo that tracks depth for indented output
#[derive(Debug, Clone)]
pub struct EchoLevelInfo {
    depth: usize,
}

impl EchoLevelInfo {
    pub fn new() -> Self {
        EchoLevelInfo { depth: 0 }
    }
}

impl LevelInfo for EchoLevelInfo {
    type AccumulatorType = EchoAccumulator;

    fn next_level(&self) -> Self {
        EchoLevelInfo { depth: self.depth + 1 }
    }

    fn create_accumulator(&self, element_info: ElementInfo) ->
        Result<EchoAccumulator, XmlDocumentError>
    {
        print!("{}<{}>", nl_indent(self.depth), element_info.owned_name.local_name);
        Ok(EchoAccumulator::new(element_info, self.depth))
    }
}

/// Accumulator that just echoes structure (doesn't build elements)
pub struct EchoAccumulator {
    element_name: String,
    element_lineno: LineNumber,
    depth: usize,
    current_subelement_name: Option<String>,
}

impl EchoAccumulator {
    pub fn new(element_info: ElementInfo, depth: usize) -> Self {
        EchoAccumulator {
            element_name: element_info.owned_name.local_name.clone(),
            element_lineno: element_info.lineno,
            depth: depth + 1,
            current_subelement_name: None,
        }
    }
}

impl Accumulator for EchoAccumulator {
    type Value = ();  // Echo doesn't return meaningful data

    fn start_subelement(&mut self, _element_info: &ElementInfo) {
        // Nothing special needed
    }
    
    fn add_subelement(&mut self, _subelement: ()) {
        // For echo, subelements have already been printed
        // We don't need to do anything with the () value
    }
    
    fn end_subelement(&mut self) {
        if let Some(name) = &self.current_subelement_name {
            print!("{}</{}>", nl_indent(self.depth + 1), name);
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
        print!("{}</{}>", nl_indent(self.depth), self.element_name);
        ()
    }
    
    fn element_name(&self) -> &str {
        &self.element_name
    }
    
    fn element_lineno(&self) -> LineNumber {
        self.element_lineno
    }
}

#[cfg(test)]
mod tests {
    use stdext::function_name;
    use std::io::{BufReader, Cursor};

    use crate::parse_doc::ParseDoc;

    use super::{EchoLevelInfo, ParseEcho};

    #[test]
    fn testit() {
        println!("Running test {}", function_name!());

        let input_str = 
            "<!--  \n".to_owned() +
            "\n" +
            "Just supply a few elements. This will only work for non-checking code.\n" +
            " -->\n" +
            "<schema xmlns:xtce=\"http://www.omg.org/spec/XTCE/20180204\" xmlns=\"http://www.w3.org/2001/XMLSchema\" targetNamespace=\"http://www.omg.org/spec/XTCE/20180204\" elementFormDefault=\"qualified\" attributeFormDefault=\"unqualified\" version=\"1.2\">\n" +
            "    <one>\n" +
            "       <two>\n" +
            "          <three>\n" +
            "          </three>\n" +
            "       </two>\n" +
            "    </one>\n" +
            "    <four>\n" +
            "    </four>\n" +
            "</schema>\n";
        for (lineno, line) in input_str.split('\n').enumerate() {
            println!("{} {}", lineno, line);
        }

        let cursor = Cursor::new((&input_str).as_bytes());
        let reader = BufReader::new(cursor);

        let echo_level_info = EchoLevelInfo::new();

        // FIXME: Handle returned error
        let _ = ParseEcho::parse(reader, &echo_level_info);
        println!();
    }
}
