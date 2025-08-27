/*
 * Takes XML input from a Reader and parses it. It uses the LevelInfo and
 * DocumentWorking traits so that it can be used to do all sorts of things
 * while parsing.
 */
// FIXME: delete all uses of expect(), everywhere

use dyn_clone::DynClone;
use std::convert::Infallible;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
//use std::marker::PhantomData;
use std::ops::{FromResidual, Try};
//use std::sync::Arc;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

use crate::parser::{LineNumber, Parser};
//use crate::walk_and_print::print_walk;
use crate::walk_and_print::nl_indent;
use crate::walk_and_print::vec_display;
use crate::walk_and_print::XmlDisplay;
pub use crate::xml_document_error::XmlDocumentError;

/**
 * XmlDocumentFactory - Parses an entire XML document
 * LI   Information passed top down during the parse which is specific to each
 *      level. This could be nothing, something simple like a depth of the tree
 *      being parsed, or a reference to one level of the tree being parsed.
 * AC   Holds information internal processing of a level. It could, for example,
 *      a Vec() that accumulates the subelements of the element at a particular
 *      level
 */
pub trait XmlDocumentFactory {
    type LI: LevelInfo;
    // This doesn't seem right, should just be Accumulator
    type AC: Accumulator<Value = Box<dyn Element>>;

    // FIXME: rename to something like parse_from_path
    fn parse_path<'b>(path:       &'b str,
        level_info: &Self::LI,
    ) -> Result<(DocumentInfo, Box<dyn Element>), XmlDocumentError>
    {
        let file = match File::open(path) {
//            Err(e) => return Err(XmlDocumentError::Error(Arc::new(e))),
            Err(e) => {
/*
                // The 'end' method on a dummy accumulator can be used to construct the correct
                // error type.  This is a common pattern to leverage trait methods for generic
                // error handling.
                let mut dummy_acc = Self::accumulator_new(OwnedName::default(), ElementInfo::new(0, vec![], Namespace::new()));
                return dummy_acc.end().map_err(|_| XmlDocumentError::Error(Arc::new(e)));
*/
                panic!("FIXME: unable to open {}: {}", path, e);
            },
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        Self::parse::<File>(reader, level_info)
    }

    // FIXME: rename to something like parse_from_reader
    fn parse<R>(buf_reader: BufReader<R>,
        level_info: &Self::LI,
    ) -> Result<(DocumentInfo, Box<dyn Element>), XmlDocumentError>
    where
        R:  Read,
    {
        // Create the factory using the reader and XML definition
        // Create the factory implementation and call yz()
// FIXME: Change name of xyz.
        Self::xyz(buf_reader, level_info)
    }

    fn _display_piece(&self, f: &mut fmt::Formatter<'_>, pieces: &Vec<XmlEvent>) -> fmt::Result {
        for piece in pieces {
            match piece {
                XmlEvent::Comment(cmnt) => write!(f, "<!-- {} -->", cmnt)?,
                XmlEvent::Whitespace(ws) => write!(f, "{}", ws)?,
                XmlEvent::Characters(characters) => write!(f, "{}", characters)?,
                XmlEvent::CData(cdata) => write!(f, "{}", cdata)?,
                _ => return Err(fmt::Error),
            }
        };

        Ok(())
    }

    /**
     * Top-level trait for parsing an XML document. The document is
     * provided via a reader built on the Read attribute.
     */

// FIXME: rename this
//    fn xyz<'a, R: Read + 'a>(
    fn xyz<R>(
        reader:     BufReader<R>,
        level_info: &Self::LI
    ) -> Result<(DocumentInfo, Box<dyn Element>), XmlDocumentError>
    where
        R:  Read,
    {
        let mut parser = Parser::new(reader);
        Self::parse_document(&mut parser, &level_info)
    }

    fn parse_document<R>(parser: &mut Parser<R>, level_info: &Self::LI) ->
        Result<(DocumentInfo, Box<dyn Element>), XmlDocumentError>
    where
        R:  Read,
    {
        let document_info = match Self::parse_start_document(parser) {
            Err(e) => return Err(e),
            Ok(doc_info) => doc_info,
        };

        // Read the next XML event, which is expected to be the start of an element. We use a
        // lookahead so that we can be specific about an error if one occurred
        let xml_element = match parser.lookahead() {
            Err(e) => return Err(e),
            Ok(xml_elem) => xml_elem,
        };

        let top_element = match xml_element.event {
            XmlEvent::StartElement{name, attributes, namespace} => {
                let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                match Self::parse_element(parser, name, element_info, level_info) {
                    Err(e) => return Err(e),
                    Ok(top_elem) => top_elem,
                }
            },

            _ => panic!("FIXME: Expected element, got {:?}", xml_element.event),
        };

        match Self::parse_end_document(parser) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        Ok((document_info, top_element))
    }

    /*
     * Parse a StartDocument. Nothing can preceed this
     */
    fn parse_start_document<R>(parser: &mut Parser<R>) ->
        Result<DocumentInfo, XmlDocumentError>
    where
        R:  Read,
    {
        let xml_element = parser.next()?;

        if let XmlEvent::StartDocument{version, encoding, standalone} = xml_element.event {
            Ok(DocumentInfo::new(version, encoding, standalone))
        } else {
            panic!("FIXME: document doesn't start with StartDocument")
        }
    }

    /*
     * Parse an element. We have already seen the XmlStartElement as a lookahead.
     */
    fn parse_element<R>(parser: &mut Parser<R>, name: OwnedName, element_info: ElementInfo, parent_level_info: &Self::LI) ->
//        <<Self as XmlDocumentFactory>::AC as Accumulator>::Result
        Result<<<Self as XmlDocumentFactory>::AC as Accumulator>::Value, XmlDocumentError>
    where
        R:  Read,
        <Self::AC as Accumulator>::Result: FromResidual<Result<Infallible, XmlDocumentError>>,
    {
        parser.skip();
        let level_info = parent_level_info.next();
        let mut accumulator = Self::accumulator_new(name, element_info);

        // Now parse all subelements of this element until we get to the EndElement for this
        // element.
        loop {
            let xml_element = parser.lookahead()?;

            match xml_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
                    if accumulator.in_element() {
                        panic!("FIXME: element <{}> definition should be closed before defining <{}>",
                            accumulator.name(), accumulator.open_subelement().unwrap().name());
                    }

                    let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                    let subelement: <<Self as XmlDocumentFactory>::AC as Accumulator>::Value = Self::parse_element(parser, name, element_info, &level_info)?;
                    accumulator.start_subelement(subelement);
                },

                XmlEvent::EndElement{name} => {

                    // If we are not in an element, this end element is for the element we are
                    // entered this function to parse

                    match accumulator.open_subelement() {
                        None => {
                            break;
                        },
                        Some(subelement) => {
                            parser.skip();
                            if name.local_name != subelement.name() {
                                panic!("FIXME: name of element <{}> at {} does not match name of closing element <{}> at {}", name, xml_element.lineno, subelement.name(), subelement.lineno());
                            }

                            accumulator.end_subelement();
                        },
                    }
                },

                XmlEvent::EndDocument => {
                    if accumulator.in_element() {
                        panic!("FIXME: element <{}> at {} is not closed", accumulator.name(), accumulator.lineno());
                    }
                }

                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {
                    parser.skip();
                },

                _ => panic!("FIXME: got {:?} instead of closing element <{}> at {}", xml_element.event, accumulator.name(), accumulator.lineno()),
            }
        }

        accumulator.end()
    }

    /*
     * We expect EndDocument, parsed as a lookahead
     */
    // FIXME: return Ok() in some form
    fn parse_end_document<R>(parser: &mut Parser<R>) -> Result<(), XmlDocumentError>
    where
        R:  Read,
    {
        parser.skip();

        loop {
            let xml_element = parser.next()?;
            match xml_element.event {
                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {},

                XmlEvent::EndDocument => break,

                _ => panic!("FIXME: Expected end of document but found {:?}", xml_element.event)
            }
        }

        Ok(())
    }

    /**
     * Allocate a new Accumulator
     */
    fn accumulator_new(name: OwnedName, element_info: ElementInfo) ->
        Box<dyn Accumulator<Value = <<Self as XmlDocumentFactory>::AC as Accumulator>::Value, Result = <<Self as XmlDocumentFactory>::AC as Accumulator>::Result>>;

/*
    /**
     * Return an error value from parsing one level of the document
     */
    fn err(e: XmlDocumentError) -> Self::RES;

    /**
     * Return a success value from parsing one level of the document
     */
// FIXME: Rename DocumentInfo to XmlDocumentInfo
    fn ok(document_info: DocumentInfo, top_element: <<Self as XmlDocumentFactory>::AC as Accumulator>::Value) -> Self::RES;
*/
}

#[derive(Clone, Debug)]
pub struct ElementInfo {
    pub lineno: LineNumber,
/*
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
*/
}

impl ElementInfo {
    pub fn new(
        lineno: LineNumber,
        _attributes: Vec<OwnedAttribute>,
        _namespace: Namespace,
    ) -> ElementInfo {
        ElementInfo {
            lineno,
/*
            attributes: attributes,
            namespace: namespace,
*/
        }
    }
}

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
            version,
            encoding,
            standalone,
        }
    }
}

#[derive(Clone)]
pub struct DirectElement {
    pub name: OwnedName,
    pub element_info: ElementInfo,
    pub before_element: Vec<XmlEvent>,
    pub content: Vec<XmlEvent>,
    pub after_element: Vec<XmlEvent>,
    pub subelements: Vec<Box<dyn Element>>,
}

impl DirectElement {
    pub fn new(name: OwnedName, element_info: ElementInfo,
        before_element: Vec::<XmlEvent>,
        content: Vec::<XmlEvent>,
        after_element: Vec::<XmlEvent>,
        subelements: Vec<Box<dyn Element>>) -> DirectElement {
        DirectElement {
            name,
            element_info,
            subelements,
            before_element,
            content,
            after_element,
        }
    }

/*
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        for attribute in &self.element_info.attributes {
            if attribute.name.local_name == name {
                return Some(&attribute.value);
            }
        }

        return None;
    }
*/
}

impl Default for DirectElement {
    fn default() -> DirectElement {
        DirectElement {
            name: OwnedName {
                local_name: "".to_string(),
                namespace:  None,
                prefix:     None
            },
            element_info: ElementInfo {
                lineno:     0,
/*
                attributes: vec!(),
                namespace:  Namespace(BTreeMap::<String, String>::new()),
*/
            },
            subelements: vec!(),
            before_element: vec!(),
            content: vec!(),
            after_element: vec!(),
        }
    }
}

impl fmt::Display for DirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl fmt::Debug for DirectElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

impl Element for DirectElement {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}vec!(Box::new(DirectElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let owned_name = OwnedName {
            local_name: self.name.to_string(),
            namespace:  None,
            prefix:     None,
        };
        owned_name_display(f, depth + 1, &owned_name)?;

        let element_info = ElementInfo {
            lineno:     0,
/*
            attributes: vec!(),
            namespace:  Namespace(BTreeMap::<String, String>::new()),
*/
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
        &self.name.local_name
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
/*
            attributes: vec!(),
            namespace:  Namespace(BTreeMap::<String, String>::new()),
*/
        };
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}vec!(), vec!(), vec!(),", nl_indent(depth + 1))?;

        write!(f, "{}vec!(", nl_indent(depth + 1))
    }
}

fn owned_name_display(f: &mut fmt::Formatter<'_>, depth: usize, owned_name: &OwnedName) -> fmt::Result {
    write!(f, "{}OwnedName{{local_name: \"{}\".to_string(),", nl_indent(depth), owned_name.local_name)?;
    write!(f, "{}namespace: {:?}, prefix: {:?}}},", nl_indent(depth + 1), owned_name.namespace, owned_name.prefix)
}

fn element_info_display(f: &mut fmt::Formatter<'_>, depth: usize, element_info: &ElementInfo) -> fmt::Result {
    write!(f, "{}ElementInfo::new({}, vec!(),", nl_indent(depth), element_info.lineno)?;
    write!(f, "{}Namespace(BTreeMap::<String, String>::new())),", nl_indent(depth + 1))
}

/**
 * Information passed to subelements at each level. It may be used to
 * provide recursive information to guide the parse.
 */
pub trait LevelInfo {
    fn next(&self) -> Self;
}

/**
 * Information about an element as we parse it
 */
pub trait Accumulator
{
//    type Value = Box<dyn Element>;
    type Value;
    // Return value for element processing
    type Result: Try<Output = Self::Value> + FromResidual<Result<Infallible, XmlDocumentError>>;
    //type Result;

    /**
     * Return the final result from processing an Element
     */
    fn end(&self) -> Result<Self::Value, XmlDocumentError>;

    /**
     * Start processing a subelement
     */
    fn start_subelement(&mut self, subelement: Self::Value);

    /**
     * Finish processing a subelement
     */
    fn end_subelement(&mut self);

    /**
     * Indicate whether we are in the middle of processing a subelement.
     */
    fn in_element(&self) -> bool;

    /**
     * Returns the name of the element we are working on
     */
    fn name(&self) -> &str;

    /**
     * Returns the line number of the start element we are working on
     */
    fn lineno(&self) -> LineNumber;

    /**
     * Get the subelement we have processed
     */
    fn open_subelement(&self) -> Option<Self::Value>;
}

/*
 * trait making DirectElement and IndirectElement work well together
 * name:            Function that returns the name of the element
 * get:             Search for an element by name. FIXME: This is probably for
 *                  future expansion.
 * name:            Returns the name for the element. FIXME: This really only
 *                  makes sense for DirectElements and should probably be removed
 * subelements:     Returns a reference to a vector of Elements. These are
 *                  sub-elements for DirectElements and a linear set of elements
 *                  at the same depth as the parent element for IndirectElements.
 * subelements_mut: Like subelements but returns a mutable value
 */
pub trait Element: DynClone {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
    fn get(&self, name: &str) -> Option<&dyn Element>;
    fn name(&self) -> &str;
    fn lineno(&self) -> LineNumber;
    fn subelements(&self) -> &Vec<Box<dyn Element>>;
    fn subelements_mut(&mut self) -> &mut Vec<Box<dyn Element>>;
}

dyn_clone::clone_trait_object!(Element);

/* Check all Display impls to ensure status is passed back properly */
impl fmt::Display for Box<dyn Element> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl fmt::Debug for Box<dyn Element> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// FIXME: do better
        self.display(f, 0)
    }
}
