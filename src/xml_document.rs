/*
 * Take an XML Definition tree and generate an XmlDocument
 */

//use std::error::Error;
use std::ops::Deref;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Arc;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

use crate::parser::LineNumber;
//use crate::xml_document::DirectElement;
use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::XmlDocumentFactory;
use crate::xml_schema::XmlSchema;
use crate::walk_and_print::XmlPrint;

/*
 * Parsed XML document
 *
 * document_info    Information about the document
 * elements         The oarsed document
 */
//#[derive(Debug)]
pub struct XmlDocument<'a> {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element<'a>>,
}

impl<'a> XmlDocument<'a> {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element<'a>>) -> XmlDocument<'a> {
        XmlDocument {
            document_info:  document_info,
            root:           root,
        }
    }

    pub fn new_from_path(
        path: &str,
        xml_schema: &'a XmlSchema<'a>,
    ) -> Result<XmlDocument<'a>, XmlDocumentError>
    {
        let file = match File::open(path) {
            Err(e) => return Err(XmlDocumentError::Error(Arc::new(e))),
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        XmlDocument::new_from_reader(reader, xml_schema)
    }

    pub fn new_from_reader<'b, R: Read + 'b>(
        buf_reader: BufReader<R>,
        xml_schema: &'b XmlSchema<'b>,
    ) -> Result<XmlDocument<'b>, XmlDocumentError> {
        // Create the factory using the reader and XML definition
        let xml_document = XmlDocumentFactory::<R>::new_from_reader(buf_reader, xml_schema)?;
        Ok(xml_document)
    }

    fn _display_piece(&self, f: &mut fmt::Formatter<'_>, pieces: &Vec<XmlEvent>) -> fmt::Result {
        let result = for piece in pieces {
            match piece {
                XmlEvent::Comment(cmnt) => write!(f, "<!-- {} -->", cmnt)?,
                XmlEvent::Whitespace(ws) => write!(f, "{}", ws)?,
                XmlEvent::Characters(characters) => write!(f, "{}", characters)?,
                XmlEvent::CData(cdata) => write!(f, "{}", cdata)?,
                _ => return Err(fmt::Error),
            }
        };

        Ok(result)
    }
}

impl fmt::Display for XmlDocument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut xml_print = XmlPrint::new(f);
        xml_print.walk(self)
    }
}

impl<'a> fmt::Debug for XmlDocument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut xml_print = XmlPrint::new(f);
        xml_print.walk(self)
    }
}

#[derive(Clone, Debug)]
pub struct ElementInfo {
    pub lineno: LineNumber,
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
}

impl ElementInfo {
    pub fn new(
        lineno: LineNumber,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> ElementInfo {
        ElementInfo {
            lineno: lineno,
            attributes: attributes,
            namespace: namespace,
        }
    }
}

/*
 * trait making DirectElement and IndirectElement work well together
 * name:    Function that returns the name of the element
 * get:     Search for an element by name
 */
pub trait Element<'a> {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn get<'b>(&self, name: &str) -> Option<Box<dyn Element<'b>>>;
    fn name<'b>(&'b self) -> &'b str;
    fn subelements(&'a self) -> &'a Vec<Box<dyn Element<'a>>>;
}

/* Check all Display impls to ensure status is passed back properly */
// FIXME: why do I need two dyn Element? Maybe eliminate everything
// with Sync or everything without Sync.
impl fmt::Display for Box<dyn Element<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
//        write!(f, "{}", *self)
    }
}

impl<'a> fmt::Debug for Box<dyn Element<'a>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
//        write!(f, "{}", *self)
    }
}

/*
impl<'a> fmt::Display for dyn Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "schema element {}\n", self.name())?;
        write!(f, "...{} subelements\n", self.subelements().len());
        for element in &*self.subelements() {
            write!(f, "{}", self)?;
        }
        Ok(())
    }
}

impl fmt::Debug for dyn Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
*/

/*
impl<'a> fmt::Display for dyn Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "schema element {}\n", self.name())?;
        write!(f, "...{} subelements\n", self.subelements().len());
        for element in &*self.subelements() {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}

impl fmt::Debug for dyn Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f)
    }
}
 */

/*
 * Define the structure used to construct the tree for the parsed document.
 */
pub struct DirectElement<'a> {
    pub name: OwnedName,
    pub element_info: ElementInfo,
    // Always empty
    pub subelements: Vec<Box<dyn Element<'a>>>,
    pub before_element: Vec<XmlEvent>,
    pub content: Vec<XmlEvent>,
    pub after_element: Vec<XmlEvent>,
}

impl<'a> DirectElement<'a> {
    pub fn new(name: OwnedName, element_info: ElementInfo) -> DirectElement<'a> {
        DirectElement {
            name: name,
            element_info: element_info,
            subelements: Vec::<Box<dyn Element<'a>>>::new(),
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        for attribute in &self.element_info.attributes {
            if attribute.name.local_name == name {
                return Some(&attribute.value);
            }
        }

        return None;
    }
}

impl<'a> fmt::Display for DirectElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl<'a> fmt::Debug for DirectElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

impl<'a> Element<'a> for DirectElement<'a> {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(depth);

        write!(f, "{}\"{}\"", indent_string, self.name())?;
        let subelements = &self.subelements;
        println!("subelements.len {}", subelements.len());

        if subelements.len() == 0 {
            write!(f, " []\n")?;
        } else {
            write!(f, " [\n")?;


            for _elem in subelements {
                todo!()
//                elem.display(f, depth + 1)?;
            }

            write!(f, "{}]\n", indent_string)?;
        }

        Ok(())
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }

    fn get<'aaa>(&self, _name: &str) -> Option<Box<dyn Element<'aaa>>> {
        todo!();
    }

    fn name<'aaa>(&'aaa self) -> &'aaa str {
        &self.name.local_name
    }

    /**
     * No subelements here
     */
    fn subelements(&self) -> &Vec<Box<dyn Element<'a>>> {
        &self.subelements
    }
}

/**
 * IndirectElements allow for duplicting part of the XML tree. They are
 * probably only going to be used for manually constructed trees, though
 * it would theoretically be possible to automatically extract them.
 */
pub struct IndirectElement<'a> {
    subelements:    Vec<Box<dyn Element<'a>>>,
}

impl<'a> IndirectElement<'_> {
    fn new() -> IndirectElement<'a> {
        IndirectElement {
            subelements:    Vec::new(),
        }
    }
}

impl<'a> Element<'a> for IndirectElement<'a> {
    fn display(&self, _f: &mut fmt::Formatter<'_>, _depth: usize) -> fmt::Result {
        todo!()
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }

    fn get<'b>(&self, _name: &str) -> Option<Box<dyn Element<'b>>> {
        todo!();
    }

    fn name<'b>(&'b self) -> &'b str {
        todo!();
    }

    fn subelements(&'a self) -> &'a Vec<Box<dyn Element<'a>>> {
        &self.subelements
    }
}

impl<'a> fmt::Display for IndirectElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl<'a> fmt::Debug for IndirectElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

/*
    pub fn start_string(&self, depth: usize) -> String {
        format!("{}<{}", Self::indent(depth), self.name.local_name)
    }

    pub fn attributes_string(&self) -> String {
        let mut result = "".to_string();

        for attribute in &self.element_info.attributes {
            result = result + format!(" {}=\"{}\"", attribute.name, attribute.value).as_str();
        }

        result
    }

    fn is_one_line(&self) -> bool {
        self.subelements.len() == 0 && self.content.len() == 0
    }

    pub fn end_first_line_string(&self) -> String {
        if self.is_one_line() {
            format!(" /> (line {})", self.element_info.lineno)
        } else {
            format!("> (line {})", self.element_info.lineno)
        }
    }

    pub fn end_n_line_string(&self, depth: usize) -> Option<String> {
        if !self.is_one_line() {
            Some(format!(
                "{}</{}> (line {})",
                Self::indent(depth),
                self.name.local_name,
                self.element_info.lineno
            ))
        } else {
            None
        }
    }

    pub fn display_start(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        write!(f, "{}", self.start_string(depth))?;
        write!(f, "{}", self.attributes_string())?;
        write!(f, "{}", self.end_first_line_string())
    }

    pub fn display_end(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        if let Some(string) = self.end_n_line_string(depth) {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
*/

/**
 * Basic information about the document
 */
#[derive(Clone, Debug)]
pub struct DocumentInfo {
    pub version: XmlVersion,
    pub encoding: String,
    pub standalone: Option<bool>,
}

impl DocumentInfo {
    pub fn new(version: XmlVersion, encoding: String, standalone: Option<bool>) -> DocumentInfo {
        DocumentInfo {
            version: version,
            encoding: encoding,
            standalone: standalone,
        }
    }
}

#[cfg(test)]
mod tests {
/*
        use lazy_static::lazy_static;

        use std::io::Cursor;

        use super::*;

        use crate::xml_schema::{DirectElement, Element};

        lazy_static!{
            static ref TEST_XML_DESC_TREE: XmlSchema<'static> =
                XmlSchema::new("MySchema",
                    Arc::new(DirectElement::new("XTCE", vec!(
                    Arc::new(DirectElement::new("SpaceSystem", vec!(
                        Arc::new(DirectElement::new("a1", vec!(
                            Arc::new(DirectElement::new("a2", vec!())),
                        ))),
                        Arc::new(DirectElement::new("a2", vec!(
                            Arc::new(DirectElement::new("a1", vec!()))
                        ))),
                    ))),
                ))),
            );
        }

        lazy_static!{
            static ref TEST_MATH: XmlSchema<'static> =
                XmlSchema::new("MathSchema",
                    Arc::new(DirectElement::new("Math", vec!(
                    Arc::new(DirectElement::new("operand", vec!(
                        Arc::new(DirectElement::new("int", vec!())),
                    ))),
                    Arc::new(DirectElement::new("operator", vec!())),
                ))),
            );
        }

        #[test] #[ignore]
        fn test1() {
            println!("Test: test1");
            (*TEST_XML_DESC_TREE).validate().unwrap();
            println!("-----------------------------");
            println!("Schema:");
            println!("{}", *TEST_XML_DESC_TREE);

            println!("-----------------------------");
            println!("Input:");
            let input = r#"<?xml version="1.0"?>
                <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                    <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                    <a1 />
                    <a2 attr1="xyz" attr2="abc">
                    </a2>
                    </SpaceSystem>
                </XTCE>"#;
            println!("{}", input);
            println!("-----------------------------");
            println!("Parsing:");
            let cursor = Cursor::new(input);
            let buf_reader = BufReader::new(cursor);

            match XmlDocument::new_from_reader(buf_reader, &TEST_XML_DESC_TREE) {
                Err(e) => println!("Failed: {}", e),
                Ok(xml_document) => {
                    println!("-----------------------------");
                    println!("Result:");
                    println!("{}", xml_document);
                },
            }
        }

        #[test]
        fn test2() {
            println!("Test: test2");
            (*TEST_MATH).validate().unwrap();
            println!("-----------------------------");
            println!("Schema:");
            println!("{}", *TEST_MATH);

            println!("-----------------------------");
            println!("Input:");
            let input = r#"<?xml version="1.0"?>
                <Math xmlns="http://www.omg.org/spec/XTCE/">
                    <operand>
                        <int>
                            27
                        </int>
                    </operand>
                    <operator>
                            +
                    </operator>
                    <operand>
                        <int>
                            12
                        </int>
                    </operand>
                </Math>"#;
            println!("{}", input);
            println!("-----------------------------");
            println!("Parsing:");
            let cursor = Cursor::new(input);
            let buf_reader = BufReader::new(cursor);

            match XmlDocument::new_from_reader(buf_reader, &TEST_MATH) {
                Err(e) => println!("Failed: {}", e),
                Ok(xml_document) => {
                    println!("-----------------------------");
                    println!("Result:");
                    println!("{}", xml_document);
                },
            }
        }

        #[test] #[ignore]
        fn test3() {
            use crate::xsd_schema::XSD_SCHEMA;

            println!("Test: test3");
            println!("XML Definition: {}", *XSD_SCHEMA);
            println!();

            match XmlDocument::new_from_path("schema/SpaceSystem-patched.xsd",
                &XSD_SCHEMA) {
                Err(e) => println!("Failed: {}", e),
                Ok(xml_document) => println!("XML Document: {}", xml_document),
            }
        }
    */

    /*
        #[test]
        fn test4() {
            println!("Test: test4");
            (*TEST_XML_DESC_TREE).validate().unwrap();
            println!("-----------------------------");
            println!("Schema:");
            println!("{}", *TEST_XML_DESC_TREE);

            println!("-----------------------------");
            println!("Input:");
            let input = r#"<?xml version="1.0"?>
                <XTCE xmlns="http://www.omg.org/spec/XTCE/">
                    <SpaceSystem xmlns="http://www.omg.org/space/xtce" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.omg.org/space/xtce ../SpaceSystemV1.0.xsd" name="TrivialSat">
                    <a1 />
                    <a2 attr1="xyz" attr2="abc">
                    </a2>
                    </SpaceSystem>
                </XTCE>"#;
            println!("{}", input);
            println!("-----------------------------");
            println!("Parsing:");
            let cursor = Cursor::new(input);
            let buf_reader = BufReader::new(cursor);

            let xml_document = match XmlDocument::new_from_reader(buf_reader,
                &TEST_XML_DESC_TREE) {
                Err(e) => {
                    println!("Failed: {}", e);
                    return Err(e);
                },
                Ok(xml_document) => xml_document,
            };

            println!("-----------------------------");
    //        println!("Result:");
    //        println!("{}", xml_document);
            let print_item = PrintItem::new();
            let print = Print::new(print_item);
            print.walk(&xml_document);
        }
    */
}
