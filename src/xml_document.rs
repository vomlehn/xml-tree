/*
 * Take an XML Definition tree and an input source, then use them to
 * generate an XmlDocument
 */

//use std::error::Error;
//use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
//use std::ops::Deref;
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
//use crate::walk_and_print::{PrintAccumulator, PrintBaseLevel, PrintElemData/*, PrintWalkable*/, PrintWalkData, PrintWalkResult};
use crate::walk_and_print::{print_walk, vec_display, XmlDisplay};
//use crate::walkable::Walkable;

// FIXME: where should this function go?
use crate::walk_and_print::nl_indent;

/*
 * Parsed XML document
 *
 * document_info    Information about the document
 * elements         The oarsed document
 */
//#[derive(Debug)]
pub struct XmlDocument {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
}

impl<'a> XmlDocument {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> XmlDocument {
        XmlDocument {
            document_info:  document_info,
            root:           root,
        }
    }

    pub fn new_from_path(
        path: &'a str,
        xml_schema: &'a XmlSchema<'a>,
    ) -> Result<XmlDocument, XmlDocumentError>
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
    ) -> Result<XmlDocument, XmlDocumentError> {
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

/* lifetime parameters on method 'fmt' to not match trait declaration
    fn fmt<'b, 'c>(&self, f: &'b mut fmt::Formatter<'_>) -> fmt::Result
    where
        'b: 'c,
*/
/* lifetime parameters on method 'fmt' to not match trait declaration
    fn fmt<'b, 'c>(&self, f: &'b mut fmt::Formatter<'c>) -> fmt::Result
    where
        'b: 'c
*/
/* method not compatible with trait
    fn fmt(&self, f: &'a mut fmt::Formatter<'a>) -> fmt::Result
*/
/* method not compatible with trait
    fn fmt<'b, 'c>(&'a self, f: &'b mut fmt::Formatter<'c>) -> fmt::Result
*/
/* method not compatible with trait
    fn fmt(&self, f: &'a mut fmt::Formatter<'a>) -> fmt::Result
*/
/* method not compatible with trait
    fn fmt(&self, f: &'a mut fmt::Formatter<'_>) -> fmt::Result
*/
/* impl item signiture does not match trait item signature
    fn fmt<'b, 'c>(&self, f: &'b mut fmt::Formatter<'b>) -> fmt::Result
*/
/* impl item signiture does not match trait item signature
    fn fmt<'b>(&self, f: &'b mut fmt::Formatter<'b>) -> fmt::Result
*/
/* '_ is a reserved lifetime name
    fn fmt<'b, 'c>(&self, f: &'b mut fmt::Formatter<'_>) -> fmt::Result
    where
        'b: '_
*/
/* lifetime may not live long enough
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
*/
impl<'a> fmt::Display for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        print_walk(f, 0, self)
    }
}

impl<'a> fmt::Debug for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_walk(f, 0, self)
    }
}

/*
impl<'a> PrintWalkable<'a, PrintAccumulator, PrintBaseLevel<'a>, PrintElemData, PrintWalkData, PrintWalkResult>
for XmlDocument {
}
*/

/*
impl<'a> Walkable<'a, PrintAccumulator, PrintBaseLevel<'a>, PrintElemData, PrintWalkData, PrintWalkResult>
for XmlDocument
{
    fn xml_document(&self) -> &XmlDocument {
        self
    }
}
*/

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
 * name:        Function that returns the name of the element
 * get:         Search for an element by name. FIXME: This is probably for
 *              future expansion.
 * name:        Returns the name for the element. FIXME: This really only
 *              makes sense for DirectElements and should probably be removed
 * subelements: Returns a reference to a vector of Elements. These are
 *              sub-Elements for DirectElements and a linear set of elements
 *              at the same depth as the parent element for IndirectElements.
 */
pub trait Element {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn get(&self, name: &str) -> Option<Box<dyn Element>>;
    fn name<'b>(&'b self) -> &'b str;
    fn subelements<'b>(&'b self) -> &'b Vec<Box<(dyn Element)>>;
}

/* Check all Display impls to ensure status is passed back properly */
// FIXME: why do I need two dyn Element? Maybe eliminate everything
// with Sync or everything without Sync.
impl fmt::Display for Box<dyn Element> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
//        write!(f, "{}", *self)
    }
}

impl<'a> fmt::Debug for Box<dyn Element> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
todo!();
    }
}

pub struct DirectElement {
    pub name: OwnedName,
    pub element_info: ElementInfo,
    pub before_element: Vec<XmlEvent>,
    pub content: Vec<XmlEvent>,
    pub after_element: Vec<XmlEvent>,
    pub subelements: Vec<Box<(dyn Element)>>,
}

impl<'a> DirectElement {
    pub fn new(name: OwnedName, element_info: ElementInfo,
        before_element: Vec::<XmlEvent>,
        content: Vec::<XmlEvent>,
        after_element: Vec::<XmlEvent>,
        subelements: Vec<Box<dyn Element>>) -> DirectElement {
        DirectElement {
            name: name,
            element_info: element_info,
            subelements: subelements,
            before_element: before_element,
            content: content,
            after_element: after_element,
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

impl<'a> fmt::Display for DirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl<'a> fmt::Debug for DirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

impl<'a> Element for DirectElement {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}Box::new(DirectElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let owned_name = OwnedName {
            local_name: self.name.to_string(),
            namespace:  None,
            prefix:     None,
        };
        owned_name_display(f, depth + 1, &owned_name)?;

        let element_info = ElementInfo {
            lineno:     0,
            attributes: vec!(),
            namespace:  Namespace(BTreeMap::<String, String>::new()),
        };
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}", nl_indent(depth + 1))?;
        vec_display::<XmlEvent>(f, depth, &self.before_element)?;
        write!(f, ",")?;
        vec_display::<XmlEvent>(f, depth, &self.content)?;
        write!(f, ",")?;
        vec_display::<XmlEvent>(f, depth, &self.after_element)?;
        write!(f, ",")?;
/*
        vec!(), vec!(), vec!(),", 
*/
        write!(f, "{}vec!(", nl_indent(depth + 1))
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }

    fn get(&self, _name: &str) -> Option<Box<dyn Element>> {
        todo!();
    }

    fn name<'aaa>(&'aaa self) -> &'aaa str {
        &self.name.local_name
    }

    /**
     * No subelements here
     */
    fn subelements<'b>(&'b self) -> &'b Vec<Box<(dyn Element)>> {
        &self.subelements
    }
}

impl XmlDisplay for DirectElement {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}Box::new(DirectElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let owned_name = OwnedName {
            local_name: self.name.to_string(),
            namespace:  None,
            prefix:     None,
        };
        owned_name_display(f, depth + 1, &owned_name)?;

        let element_info = ElementInfo {
            lineno:     0,
            attributes: vec!(),
            namespace:  Namespace(BTreeMap::<String, String>::new()),
        };
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}vec!(), vec!(), vec!(),", nl_indent(depth + 1))?;

        write!(f, "{}vec!(", nl_indent(depth + 1))
    }
}

fn owned_name_display(f: &mut fmt::Formatter<'_>, depth: usize, owned_name: &OwnedName) -> fmt::Result {
    write!(f, "{}OwnedName{{local_name: \"{}\".to_string(),", nl_indent(depth), owned_name.local_name)?;
// FIXME: handle Option<> better
    write!(f, "{}namespace: {:?}, prefix: {:?}}},", nl_indent(depth + 1), owned_name.namespace, owned_name.prefix)
}

fn element_info_display(f: &mut fmt::Formatter<'_>, depth: usize, element_info: &ElementInfo) -> fmt::Result {
    write!(f, "{}ElementInfo::new({}, vec!(),", nl_indent(depth), element_info.lineno)?;
    write!(f, "{}Namespace(BTreeMap::<String, String>::new())),", nl_indent(depth + 1))
}

/*
/**
 * IndirectElements allow for duplicting part of the XML tree. They are
 * probably only going to be used for manually constructed trees, though
 * it would theoretically be possible to automatically extract them.
 */
pub struct IndirectElement {
    subelements:    Vec<Box<(dyn Element)>>,
}

impl<'a> IndirectElement {
    fn new() -> IndirectElement {
        IndirectElement {
            subelements:    Vec::new(),
        }
    }
}

impl<'a> Element for IndirectElement {
    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }

    fn get(&self, _name: &str) -> Option<Box<dyn Element>> {
        todo!();
    }

    fn name<'b>(&'b self) -> &'b str {
        todo!();
    }

    fn subelements<'b>(&'b self) -> &'b Vec<Box<(dyn Element)>> {
        &self.subelements
    }
}

impl<'a> fmt::Display for IndirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl<'a> fmt::Debug for IndirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

impl XmlDisplay for IndirectElement {
    fn print(&self, _f: &mut fmt::Formatter<'_>, _depth: usize) -> fmt::Result {
        todo!()
    }
}

*/

/*
    pub fn start_string(&self, depth: usize) -> String {
        format!("{}<{}", Self::nl_indent(depth), self.name.local_name)
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
                Self::nl_indent(depth),
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

#[cfg(test)]
/*
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;
*/

/**
 * Manually create an XmlDocument.
 */
 // FIXME: This should be moved to a common area
pub fn create_test_doc() -> XmlDocument {
    let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

    let ei: ElementInfo = ElementInfo {
        lineno: 1,
        attributes: Vec::<OwnedAttribute>::new(),
        namespace: ns,
    };

    XmlDocument {
        root:           branch("n1", &ei, vec![
                            leaf("n2", &ei),
                            branch("n3", &ei, vec![
                                leaf("n4", &ei)])
                        ]),
        document_info:  DocumentInfo {
                            version: XmlVersion::Version10,
                            encoding: "encoding".to_string(),
                            standalone: None,
                        },
    }
}

#[cfg(test)]
fn leaf(name: &str, ei: &ElementInfo) -> Box<dyn Element> {
    Box::new(node(name, ei, Vec::<Element>::new()))
}

#[cfg(test)]
fn branch(name: &str, ei: &ElementInfo, subelements: Vec<dyn Element>) -> Box<dyn Element> {
    Box::new(node(name, ei, subelements))
}

#[cfg(test)]
fn node(name: &str, ei: &ElementInfo, subelements: Vec<dyn Element>) -> Box<dyn Element> {
    Box::new(DirectElement {
        name: OwnedName {
            local_name: name.to_string(),
            namespace: None,
            prefix: None,
        },
        element_info: (*ei).clone(),
        subelements,
        before_element: Vec::<XmlEvent>::new(),
        content: Vec::<XmlEvent>::new(),
        after_element: Vec::<XmlEvent>::new(),
    })
}

pub trait ElemData<ED>
{
    fn next_level(&self, element: &Box<dyn Element>) -> ED;
}

/**
 * Data returned by Accumulator functions.
 */
pub trait WalkData {}

/**
 * Data stored at the root level of the Walkable and a reference to which is
 * returned by the Walkable base_level_cell() function.
 */
pub trait BaseLevel {}

/**
 * Data stored for the peers of the Element a given invocation of walk_down()
 */
pub trait Accumulator<'a, BL, ED, WD, WR> {
    fn new(bl: &mut BL, e: &Box<dyn Element>, ed: &ED) -> ED
    where
        Self: Sized;
    fn add(&mut self, wd: &WD) -> WR;
    fn summary(&self, bl: &mut BL) -> WR;
}

impl XmlDisplay for XmlEvent {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        write!(f, "{}{:?}", nl_indent(depth), self)
    }
}
