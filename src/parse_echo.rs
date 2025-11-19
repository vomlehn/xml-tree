/**
 * Parse XML text input and produce an XML echo
 */

use std::fmt;
use std::io::{BufReader, Read, Write};
use std::marker::PhantomData;
use std::ops::{ControlFlow, FromResidual, Try};

use crate::{Element, ElementInfo};
use crate::misc::nl_indent;
use crate::ParseLoc;
pub use crate::xml_document_error::XmlDocumentError;
use crate::parse_xml::{Accumulator, LevelInfo, ParseXml};
use crate::document::DocumentInfo;

//const TREE_DEPTH: usize = 3;

pub struct ParseEchoParams {
}

pub struct ParseEcho<'a> {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
    pub output:         &'a mut dyn Write,
}

impl<'a> ParseEcho<'a> {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>,
        output: &'a mut dyn Write) -> Self {
        ParseEcho {
            document_info,
            root,
            output,
        }
    }

    pub fn parse_path<'r: 'a, 'b>(
        &'r mut self,
        path: &'b str,
        element_level_info: &<ParseEcho<'r> as ParseXml<'r>>::LI,
    ) -> Result<(DocumentInfo, <<<ParseEcho<'r> as ParseXml<'r>>::LI as LevelInfo<'r>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    {
        self.parse_path_base(path, element_level_info)
    }

    pub fn parse<'r: 'a, R>(
        &'r mut self,
        buf_reader: BufReader<R>,
        element_level_info: &<ParseEcho<'r> as ParseXml<'r>>::LI,
    ) -> Result<(DocumentInfo, <<<ParseEcho<'r> as ParseXml<'r>>::LI as LevelInfo<'r>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
        self.parse_base(buf_reader, element_level_info)
    }
}

impl<'a> ParseXml<'a> for ParseEcho<'a> {
    type LI = EchoLevelInfo<'a>;
    type AC = EchoAccumulator;
}

impl<'a> fmt::Display for ParseEcho<'a> {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl<'a> fmt::Debug for ParseEcho<'a> {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl<'a> Try for ParseEcho<'a>
{
    type Output = <<ParseEcho<'a> as ParseXml<'a>>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl<'a> FromResidual for ParseEcho<'a> {
    fn from_residual(_: <ParseEcho as Try>::Residual) -> Self
    { todo!() }
}

/// LevelInfo that tracks depth for indented output
#[derive(Debug, Clone)]
pub struct EchoLevelInfo<'a> {
    depth: usize,
    marker: PhantomData<&'a ()>,
}

impl<'a> EchoLevelInfo<'a> {
    pub fn new() -> Self {
        EchoLevelInfo {
            depth: 0,
            marker: PhantomData,
        }
    }
}

impl<'a> LevelInfo<'a> for EchoLevelInfo<'a> {
    type ParseXmlType = ParseEcho<'a>;
//    type ParseXmlType = <ParseEcho<'a> as ParseXml<'a>>::ParseXml;
//    type AccumulatorType<'c>: Accumulator<DocType<'c>> = Self::ParseXmlType;
//    type AccumulatorType: Accumulator<DocType<'a> = Self::ParseXmlType> = ParseAccumulator;
    type AccumulatorType = EchoAccumulator;
//    where
//        Self: 'c;

    fn next_level(&self, _element_info: &ElementInfo) -> Self {
        EchoLevelInfo {
            depth: self.depth + 1,
            marker: PhantomData,
        }
    }

    fn create_accumulator(&self, _parse_xml: &mut Self::ParseXmlType, element_info: ElementInfo) ->
        Result<EchoAccumulator, XmlDocumentError>
    {
        print!("XXX{}<{}>", nl_indent(self.depth), element_info.owned_name.local_name);
        Ok(EchoAccumulator::new(element_info, self.depth))
    }
}

/// Accumulator that just echoes structure (doesn't build elements)
pub struct EchoAccumulator {
    element_name: String,
    parse_loc: ParseLoc,
    depth: usize,
    current_subelement_name: Option<String>,
}

impl EchoAccumulator {
    pub fn new(element_info: ElementInfo, depth: usize) -> Self {
        EchoAccumulator {
            element_name: element_info.owned_name.local_name.clone(),
            parse_loc: element_info.parse_loc,
            depth: depth + 1,
            current_subelement_name: None,
        }
    }
}

impl Accumulator for EchoAccumulator {
    type Value = ();  // Echo doesn't return meaningful data
     type DocType<'a> = ParseEcho<'a>;

    fn start_subelement(&mut self, _parse_xml: &mut ParseEcho, _element_info: &ElementInfo) {
        // Nothing special needed
    }
    
    fn add_subelement(&mut self, _parse_xml: &mut ParseEcho, _subelement: ()) {
        // For echo, subelements have already been printed
        // We don't need to do anything with the () value
    }
    
    fn end_subelement(&mut self, _parse_xml: &mut ParseEcho) {
        if let Some(name) = &self.current_subelement_name {
            print!("XXX{}</{}>", nl_indent(self.depth + 1), name);
        }
        self.current_subelement_name = None;
    }
    
    fn finish(self, _parse_xml: &mut ParseEcho) -> () {
        print!("XXX{}</{}>", nl_indent(self.depth), self.element_name);
        ()
    }
    
    fn has_open_subelement(&self) -> bool {
        self.current_subelement_name.is_some()
    }
    
    fn current_subelement_name(&self) -> &str {
        self.current_subelement_name.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("")
    }
    
    fn element_name(&self) -> &str {
        &self.element_name
    }
    
    fn parse_loc(&self) -> ParseLoc {
        self.parse_loc.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use stdext::function_name;
    use std::io::{BufReader, Cursor, Write};
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::{DocumentInfo, Element, ElementInfo,
        nl_indent, display_owned_name, ParseLoc/*, XmlDisplay*/};
    use crate::misc::write_vec;
    use crate::element::display_element_info;

    use super::{EchoLevelInfo, ParseEcho};

    const TREE_DEPTH: usize = 3;

//    const TREE_DEPTH: usize = 3;

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

        let owned_name = OwnedName {
            local_name: "schema".to_string(),
            namespace:  None,
            prefix:     None,
        };
        let namespace = Namespace::empty();
        let element_info = ElementInfo::new(owned_name, ParseLoc::new("TBD".to_string(), 0), vec!(), namespace);
        let element = EchoElement::new(element_info, 0, vec!(), vec!(), vec!(), vec!());
        let cursor = Cursor::new((&input_str).as_bytes());
        let reader = BufReader::new(cursor);
        let root: Box<dyn Element> = Box::new(element);

        let document_info = DocumentInfo::new(XmlVersion::Version10, "encoding".to_string(), None);

        let echo_level_info = EchoLevelInfo::new();

        let mut output = Vec::<u8>::new();
        let mut parse_echo = ParseEcho::new(document_info, root, &mut output);
        // FIXME: Handle returned error
        let _ = parse_echo.parse(reader, &echo_level_info);
        println!();
    }

#[derive(Clone)]
pub struct EchoElement {
    pub element_info:   ElementInfo,
    pub depth:          usize,
    pub before_element: Vec<XmlEvent>,
    pub content:        Vec<XmlEvent>,
    pub after_element:  Vec<XmlEvent>,
    pub subelements:    Vec<Box<dyn Element>>,
}

impl EchoElement {
    pub fn new(element_info: ElementInfo,
        depth:          usize,
        before_element: Vec::<XmlEvent>,
        content: Vec::<XmlEvent>,
        after_element: Vec::<XmlEvent>,
        subelements: Vec<Box<dyn Element>>) -> EchoElement {
        EchoElement {
            element_info,
            depth,
            subelements,
            before_element,
            content,
            after_element,
        }
    }

    fn display_element_start(&self, output: &mut dyn Write, depth: usize) ->
        fmt::Result {
        let depth0 = 3 * depth;
        let depth1 = depth0 + 1;

        // FIXME: return error code
        let _ = write!(output, "{}vec!(Box::new(EchoElement::new(",
            nl_indent(depth0));

        let owned_name = OwnedName {
            local_name: self.name().to_string(),
            namespace:  None,
            prefix:     None,
        };
        let _ = display_owned_name(output, depth1, &owned_name);

        let element_info = ElementInfo {
            parse_loc:  ParseLoc::new("TBD".to_string(), 0),
            owned_name: owned_name,
            attributes: vec!(),
        };
        let _ = display_element_info(output, depth1, &element_info);
        let _ = write!(output, "{}", nl_indent(depth1));
        let _ = write_vec::<XmlEvent>(output, depth1, &self.before_element);
        let _ = write!(output, ", ");
        let _ = write_vec::<XmlEvent>(output, depth1, &self.content);
        let _ = write!(output, ", ");
        let _ = write_vec::<XmlEvent>(output, depth1, &self.after_element);
        let _ = write!(output, ",");
        let _ = write!(output, "{}vec!(", nl_indent(depth1 + 1));
        Ok(())
    }

    fn display_element_end(&self, output: &mut dyn Write, depth: usize) -> fmt::Result {
        let depth0 = TREE_DEPTH + 3 * depth;
        let depth1 = depth0 + 1;
        let depth2 = depth1 + 2;

        let _ = write!(output, "{})", nl_indent(depth2));
        let _ = write!(output, "{})", nl_indent(depth1));
        let _ = write!(output, "{})", nl_indent(depth0));
            // FIXME: return error
        Ok(())
    }
}

/*
impl Default for EchoElement {
    fn default() -> EchoElement {
        EchoElement {
            element_info: ElementInfo {
                owned_name: OwnedName {
                    local_name: "".to_string(),
                    namespace:  None,
                    prefix:     None
                },
                parse_loc:     ParseLoc::new("TBD".to_string(), 0),
                attributes:     vec!(),
            },
            depth: 0,
            subelements: vec!(),
            before_element: vec!(),
            content: vec!(),
            after_element: vec!(),
        }
    }
}
*/

/*
impl fmt::Display for EchoElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, self.depth)
    }
}

impl fmt::Debug for EchoElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, self.depth)
    }
}
*/

impl Element for EchoElement {
/*
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display_start(f, depth)
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }
*/

    /**
     * Find a subelement (one level deeper) with the given name
     */
    fn get(&self, name: &str) -> Option<&dyn Element> {
/*
println!("get: looking for {} in {}", name, self.name());
println!("...");
*/
for x in self.subelements() {
    println!(" {}", x.name());
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

    fn parse_loc(&self) -> ParseLoc {
        self.element_info.parse_loc.clone()
    }

    /**
     * Return a vector of all subelements.
     */
//    fn subelements<'b>(&'b self) -> &'b Vec<Box<dyn Element + 'b>> {
    fn subelements(&self) -> &Vec<Box<dyn Element>> {
        &self.subelements
    }

    /**
     * Return a mutable vector of all subelements.
     */
//    fn subelements_mut<'b>(&'b mut self) -> &'b mut Vec<Box<dyn Element + '_>> {
    fn subelements_mut(&mut self) -> &mut Vec<Box<dyn Element>> {
        &mut self.subelements
    }
}

/*
impl XmlDisplay for EchoElement {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}Box::new(EchoElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let element_info = ElementInfo {
            parse_loc: ParseLoc::new("TBD".to_string(), 0),
            owned_name: OwnedName {
                        local_name: self.name().to_string(),
                        namespace:  None,
                        prefix:     None,
            },
        };

        display_owned_name(f, depth + 1, &element_info.owned_name)?;
        display_element_info(f, depth + 1, &element_info)?;
        write!(f, "{}vec!(), vec!(), vec!(),", nl_indent(depth + 1))?;

        write!(f, "{}vec!(", nl_indent(depth + 1))
    }
}
*/
}
