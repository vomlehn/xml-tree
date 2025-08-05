/*
 * Takes XML input from a Reader and parses it. It uses the LevelInfo and
 * DocumentWorking traits so that it can be used to do all sorts of things
 * while parsing.
 */
// FIXME: delete all uses of expect(), everywhere

use std::io::Read;
use std::marker::PhantomData;
use std::ops::{FromResidual, Try};
use std::convert::Infallible;
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::parser::{LineNumber, Parser};
pub use crate::xml_document::{DocumentInfo, Element, ElementInfo};
pub use crate::xml_document_error::XmlDocumentError;
//use crate::xml_schema::XmlSchema;

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
//        xml_schema: &'a XmlSchema<'a>,
    ) -> <Self::DW as DocumentWorking>::DocumentResult
    where ;
/*
        <Self::DW as DocumentWorking>::DocumentResult: FromResidual<<<Self::LI as LevelInfo>::ElementResult as Try>::Residual>,
        <Self::LI as LevelInfo>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>;
*/
/*
        <Self::DW as DocumentWorking>::DocumentResult: FromResidual<<<Self::AC as Accumulator>::ElementResult as Try>::Residual>,
        <Self::AC as Accumulator>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>;
*/
}

pub struct XmlDocumentFactoryImpl<R: Read, LI, AC, DW>
where
    LI: LevelInfo,
    AC: Accumulator,
    DW: DocumentWorking,
{
    pub parser: Parser<R>,
// FIXME: remove PhantomData items, if possible
//    pub xml_schema: &'a XmlSchema<'a>,
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
                self.parse_element(name, element_info, &level_info)?
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

/*
// FIXME: new stuff
pub trait XmlDocumentFactory {
    type LI: LevelInfo<ElementValue = Box<dyn Element>>;
    type DW: DocumentWorking;

    fn xyz<'a, R: Read + 'a>(
        &self,
        reader: R,
        xml_schema: &'a XmlSchema<'a>,
    ) -> <Self::DW as DocumentWorking>::DocumentResult
    where
        <Self::DW as DocumentWorking>::DocumentResult: FromResidual<<<Self::LI as LevelInfo>::ElementResult as Try>::Residual>,
        <Self::LI as LevelInfo>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>;
}
*/

/*
/*
 * Structure used to hold parsing information
 *
 * LI:
 * DW:
 * parser:          Used to extract XmlElement objects from the input stream
 * xml_schema:  Definition of what the input is expected to look like
 */
pub struct XmlDocumentFactory<'a, R: Read + 'a, LI, DW>
where
    LI:  LevelInfo,
    DW:  DocumentWorking,
{
    parser:         Parser<R>,
    pub xml_schema: &'a XmlSchema<'a>,
    /* FIXME: can I remove these */
    marker1:        PhantomData<LI>,
    marker2:        PhantomData<DW>,
}

impl<'a, R: Read + 'a, LI, DW> XmlDocumentFactory<'_, R, LI, DW>
where
    LI: LevelInfo<ElementValue = Box<dyn Element>>,
    DW: DocumentWorking,
{
//    FIXME: should T be R?
    pub fn new<T: Read + 'a> (
        reader: T,
        xml_schema: &'a XmlSchema<'a>,
    ) -> DW::DocumentResult
        where
            <DW as DocumentWorking>::DocumentResult: FromResidual<<<LI as LevelInfo>::ElementResult as Try>::Residual>,
            <LI as LevelInfo>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>,
        {
        let parser = Parser::<T>::new(reader);

        let mut xml_factory = XmlDocumentFactory::<T, LI, DW> {
            parser:     parser,
            xml_schema: xml_schema,
            marker1:    PhantomData,
            marker2:    PhantomData,
        };

        let xml_document = xml_factory.parse_document::<T>();
        xml_document
    }

    fn parse_document<T: Read + 'a>(&mut self) -> <DW as DocumentWorking>::DocumentResult
    where
        <DW as DocumentWorking>::DocumentResult: FromResidual<<<LI as LevelInfo>::ElementResult as Try>::Residual>,
        <LI as LevelInfo>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>,
    {
        let document_info = self.parse_start_document()?;
        let document_data = DW::start(document_info);

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
    fn parse_element(&mut self, name: OwnedName, element_info: ElementInfo, depth: usize) -> LI::ElementResult
    where
        <LI as LevelInfo>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>,
    {
        self.parser.skip();
        let mut level_info = LI::start(name, element_info);

        // Now parse all subelements of this element until we get to the EndElement for this
        // element.
        loop {
            let xml_element = self.parser.lookahead()?;

            match xml_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
                    if level_info.in_element() {
                        panic!("FIXME: element <{}> definition should be closed before defining <{}>",
                            level_info.name(), level_info.open_subelement().unwrap().name());
                    }

                    let element_info = ElementInfo::new(xml_element.lineno, attributes, namespace);
                    let subelement = self.parse_element(name, element_info, depth + 1)?;
                    level_info.start_subelement(subelement);
                },

                XmlEvent::EndElement{name} => {

                    // If we are not in an element, this end element is for the element we are
                    // entered this function to parse

                    match level_info.open_subelement() {
                        None => {
                            break;
                        },
                        Some(subelement) => {
                            self.parser.skip();
                            if name.local_name != subelement.name() {
                                panic!("FIXME: name of element <{}> at {} does not match name of closing element <{}> at {}", name, xml_element.lineno, subelement.name(), subelement.lineno());
                            }

                            level_info.end_subelement();
                        },
                    }
                },

                XmlEvent::EndDocument => {
                    if level_info.in_element() {
                        panic!("FIXME: element <{}> at {} is not closed", level_info.name(), level_info.lineno());
                    }
                }

                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {
                    self.parser.skip();
                },

                _ => panic!("FIXME: got {:?} instead of closing element <{}> at {}", xml_element.event, level_info.name(), level_info.lineno()),
            }
        }

        level_info.end()
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
*/
