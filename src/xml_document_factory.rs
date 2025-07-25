/*
 * Take an Element tree and generate an XmlFactorTree, which is used
 * to parse XML input
 */
// FIXME: delete all uses of expect(), everywhere

use std::io::Read;
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::parser::{LineNumber, Parser/*, XmlDirectElement*//*, XmlElement*/};
pub use crate::xml_document::{DocumentInfo, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
use crate::xml_tree_element::XmlTreeElement;
use crate::xml_schema::XmlSchema;

/**
 * Information about an element as we parse it
 * D    The return type
 */
pub trait ElementData
{
    type ElementResult;

    /**
     * Create a new struct for the currently parsed element
     */
    fn start(name: OwnedName, element_info: ElementInfo) -> Self;

    /**
     * Return the final result from processing an Element
     */
    fn end(&self) -> Self::ElementResult;

    /**
     * Start processing a subelement
     */
    fn start_subelement(&mut self, subelement: Self::ElementResult);

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
    fn open_subelement(&self) -> Option<Self::ElementResult>;
}

/*
 * Structure used to hold parsing information
 * parser:          Used to extract XmlElement objects from the input stream
 * xml_schema:  Definition of what the input is expected to look like
 */
pub struct XmlDocumentFactory<'a, R: Read + 'a> {
    parser: Parser<R>,
    pub xml_schema: &'a XmlSchema<'a>,
}

impl<'a, R: Read + 'a> XmlDocumentFactory<'_, R> {
    pub fn new_from_reader<T: Read + 'a> (
        reader: T,
        xml_schema: &'a XmlSchema<'a>,
    ) -> Result<XmlDocument, XmlDocumentError> {
        let parser = Parser::<T>::new(reader);

        let mut xml_factory = XmlDocumentFactory::<T> {
            parser: parser,
            xml_schema: xml_schema,
        };

        let xml_document = xml_factory.parse_document::<T>();
        xml_document
    }

    fn parse_document<T: Read + 'a>(&mut self) -> Result<XmlDocument, XmlDocumentError> {
        let document_info = self.parse_start_document()?;

        // Read the next XML event, which is expected to be the start of an element. We use a
        // lookahead so that we can be specific about an error if one occurred
        let xml_element = self.parser.lookahead()?;

        let top_element = match xml_element.event {
            XmlEvent::StartElement{name, attributes, namespace} => {
                let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                self.parse_element(name, element_info, 0)?
            },

            _ => panic!("FIXME: Expected element, got {:?}", xml_element.event),
        };

        self.parse_end_document()?;

        Ok(XmlDocument::new(document_info, vec!(top_element)))
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
    fn parse_element(&mut self, name: OwnedName, element_info: ElementInfo, depth: usize) ->
        Result<Box<dyn Element>, XmlDocumentError> {
        self.parser.skip();
        let mut element_data = XmlTreeElement::start(name, element_info);

        // Now parse all subelements of this element until we get to the EndElement for this
        // element.
        loop {
            let xml_element = self.parser.lookahead()?;

            match xml_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {

                    if element_data.in_element() {
                        panic!("FIXME: element <{}> definition should be closed before defining <{}>",
                            element_data.name(), element_data.open_subelement().unwrap().name());
                    }

                    let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                    let subelement = self.parse_element(name, element_info, depth + 1)?;
                    element_data.start_subelement(subelement);
                },

                XmlEvent::EndElement{name} => {

                    // If we are not in an element, this end element is for the element we are
                    // entered this function to parse

                    match element_data.open_subelement() {
                        None => {
                            break;
                        },
                        Some(subelement) => {
                            self.parser.skip();
                            if name.local_name != subelement.name() {
                                panic!("FIXME: name of element <{}> at {} does not match name of closing element <{}> at {}", name, xml_element.lineno, subelement.name(), subelement.lineno());
                            }

                            element_data.end_subelement();
                        },
                    }
                },

                XmlEvent::EndDocument => {
                    if element_data.in_element() {
                        panic!("FIXME: element <{}> at {} is not closed", element_data.name(), element_data.lineno());
                    }
                }

                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {
                    self.parser.skip();
                },

                _ => panic!("FIXME: got {:?} instead of closing element <{}> at {}", xml_element.event, element_data.name(), element_data.lineno()),
            }
        }

        Ok(element_data.end())
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
