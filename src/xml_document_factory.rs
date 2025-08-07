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
use std::marker::PhantomData;
use std::ops::{FromResidual, Try};
use std::sync::Arc;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

use crate::parser::{LineNumber, Parser};
use crate::walk_and_print::print_walk;
use crate::walk_and_print::nl_indent;
use crate::walk_and_print::vec_display;
use crate::walk_and_print::XmlDisplay;
pub use crate::xml_document_error::XmlDocumentError;
// FIXME: remove this
use crate::xml_document_tree::XmlTreeFactory;


/**
 * Trait for XML document factories
 */
pub trait XmlDocumentFactory {
    type LI: LevelInfo;
    type AC: Accumulator<ElementValue = Box<dyn Element>>;
    type DW: DocumentWorking;

    fn xyz<'a, R: Read + 'a>(
        &self,
        reader: R,
    ) -> <Self::DW as DocumentWorking>::DocumentResult
    where ;
}

pub struct XmlDocumentFactoryImpl<R: Read, LI, AC, DW>
where
    LI: LevelInfo,
    AC: Accumulator,
    DW: DocumentWorking,
{
    pub parser: Parser<R>,
    pub marker1: PhantomData<LI>,
    pub marker3: PhantomData<AC>,
    pub marker2: PhantomData<DW>,
}

impl<R: Read, LI, AC, DW> XmlDocumentFactoryImpl<R, LI, AC, DW>
where
    LI: LevelInfo,
    AC: Accumulator<ElementValue = Box<dyn Element>>,
    DW: DocumentWorking,
{
    pub fn parse_document(&mut self, level_info: &LI) -> <DW as DocumentWorking>::DocumentResult
    where
        <DW as DocumentWorking>::DocumentResult: FromResidual<<<AC as Accumulator>::ElementResult as Try>::Residual>,
        <AC as Accumulator>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>,
    {
        let document_info = self.parse_start_document()?;
        let document_data = DW::start(document_info);

        // Read the next XML event, which is expected to be the start of an element. We use a
        // lookahead so that we can be specific about an error if one occurred
        let xml_element = self.parser.lookahead()?;

        let top_element = match xml_element.event {
            XmlEvent::StartElement{name, attributes, namespace} => {
                let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                self.parse_element(name, element_info, level_info)?
            },

            _ => panic!("FIXME: Expected element, got {:?}", xml_element.event),
        };

        self.parse_end_document()?;
        DW::end(&document_data, vec!(top_element))
    }

    /*
     * Parse a StartDocument. Nothing can preceed this
     */
    fn parse_start_document(&mut self) -> Result<DocumentInfo, XmlDocumentError> {
        let xml_element = self.parser.next()?;

        if let XmlEvent::StartDocument{version, encoding, standalone} = xml_element.event {
            Ok(DocumentInfo::new(version, encoding, standalone))
        } else {
            panic!("FIXME: document doesn't start with StartDocument")
        }
    }

    /*
     * Parse an element. We have already seen the XmlStartElement as a lookahead.
     */
    fn parse_element(&mut self, name: OwnedName, element_info: ElementInfo, parent_level_info: &LI) -> AC::ElementResult
    where
        <AC as Accumulator>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>,
    {
        self.parser.skip();
        let level_info = parent_level_info.next();
        let mut accumulator = AC::new(name, element_info);

        // Now parse all subelements of this element until we get to the EndElement for this
        // element.
        loop {
            let xml_element = self.parser.lookahead()?;

            match xml_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
                    if accumulator.in_element() {
                        panic!("FIXME: element <{}> definition should be closed before defining <{}>",
                            accumulator.name(), accumulator.open_subelement().unwrap().name());
                    }

                    let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                    let subelement = self.parse_element(name, element_info, &level_info)?;
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
                            self.parser.skip();
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
                    self.parser.skip();
                },

                _ => panic!("FIXME: got {:?} instead of closing element <{}> at {}", xml_element.event, accumulator.name(), accumulator.lineno()),
            }
        }

        accumulator.end()
    }

    /*
     * We expect EndDocument, parsed as a lookahead
     */
    fn parse_end_document(&mut self) -> Result<(), XmlDocumentError> {
        self.parser.skip();

        loop {
            let xml_element = self.parser.next()?;
            match xml_element.event {
                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {},

                XmlEvent::EndDocument => break,

                _ => panic!("FIXME: Expected end of document but found {:?}", xml_element.event)
            }
        }

        Ok(())
    }
}

/**
 * Information passed to subelements
 */
pub trait LevelInfo {
    fn next(&self) -> Self;
}

/**
 * Information about an element as we parse it
 */
pub trait Accumulator
{
    type ElementValue;

    // Return value for element processing
    type ElementResult: Try<Output = Self::ElementValue> + FromResidual<Result<Infallible, XmlDocumentError>>;

    /**
     * Create a new struct for the currently parsed element
     */
    fn new(name: OwnedName, element_info: ElementInfo) -> Self;

    /**
     * Return the final result from processing an Element
     */
    fn end(&self) -> Self::ElementResult;

    /**
     * Start processing a subelement
     */
    fn start_subelement(&mut self, subelement: Self::ElementValue);

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
    fn open_subelement(&self) -> Option<Self::ElementValue>;
}

pub trait DocumentWorking {
    type DocumentValue;

    type DocumentResult: Try<Output = Self::DocumentValue> + FromResidual<Result<Infallible, XmlDocumentError>>;

    /**
     * Create a new struct for the currently parsed document
     */
    fn start(document_info: DocumentInfo) -> Self;

    /**
     * Return the final result from processing an Element
     */
    fn end(&self, top_element: Vec<Box<dyn Element>>) -> Self::DocumentResult;
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

/*
 * Parsed XML document
 *
 * document_info    Information about the document
 * elements         The oarsed document
 */
pub struct XmlDocument {
    pub document_info:  DocumentInfo,
    pub root:           Vec<Box<dyn Element>>,
}

impl<'a> XmlDocument {
    pub fn new(document_info: DocumentInfo, root: Vec<Box<dyn Element>>) -> XmlDocument {
        XmlDocument {
            document_info,
            root,
        }
    }

    pub fn new_from_path(
        path: &'a str,
//        xml_schema: &'a XmlSchema<'a>,
    ) -> Result<XmlDocument, XmlDocumentError>
    {
        let file = match File::open(path) {
            Err(e) => return Err(XmlDocumentError::Error(Arc::new(e))),
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        XmlDocument::new_from_reader(reader)
    }

    pub fn new_from_reader<'b, R: Read + 'b>(
        buf_reader: BufReader<R>,
//        xml_schema: &'b XmlSchema<'b>,
    ) -> Result<XmlDocument, XmlDocumentError> {
        // Create the factory using the reader and XML definition
        // let xml_document = XmlDocumentFactory::<R, XmlTreeElement, XmlDocumentTree>::new(buf_reader, xml_schema)?;
        // Create the factory implementation and call xyz()
        let factory = XmlTreeFactory;
// FIXME: Change name of xyz.
        let xml_document = factory.xyz(buf_reader)?;
        Ok(xml_document)
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
}
impl fmt::Display for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        print_walk(f, 0, self)
    }
}

impl fmt::Debug for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_walk(f, 0, self)
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
