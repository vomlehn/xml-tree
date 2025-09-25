/*
 * Takes XML input from a Reader and parses the whole thing at a high
 * level, that is, element and attributes as strings. Derived types
 * handle the specific XML means of those elements and attributes.
 */
// FIXME: delete all uses of expect(), everywhere

use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::reader::XmlEvent;

use crate::document::DocumentInfo;
use crate::element::{ElementInfo};
use crate::parse_item::{LineNumber, Parser};
pub use crate::xml_document_error::XmlDocumentError;

/**
 * ParseDoc - Parses an entire XML document
 * LI   Information passed top down during the parse which is specific to each
 *      level. This could be nothing, something simple like a depth of the tree
 *      being parsed, or a reference to one level of the tree being parsed.
 */
pub trait ParseDoc {
    type LI: LevelInfo;
    type AC: Accumulator;

    // FIXME: rename to something like parse_from_path
    fn parse_path<'b>(
        path: &'b str,
        element_level_info: &Self::LI,
    ) -> Result<(DocumentInfo, <<Self::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    {
        let file = match File::open(path) {
            Err(e) => {
                panic!("FIXME: unable to open {}: {}", path, e);
            },
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        Self::parse::<File>(reader, element_level_info)
    }

    /**
     * Top-level trait for parsing an XML document. The document is
     * provided via a reader built on the Read attribute.
     */
    fn parse<R>(
        buf_reader: BufReader<R>,
        element_level_info: &Self::LI,
    ) -> Result<(DocumentInfo, <<Self::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
        // Create the factory using the reader and XML definition
        let mut parse_item = Parser::new(buf_reader);
        Self::parse_document(&mut parse_item, &element_level_info)
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

    fn parse_document<R>(
        parse_item: &mut Parser<R>, 
        element_level_info: &Self::LI
    ) -> Result<(DocumentInfo, <<Self::LI as LevelInfo>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
        let document_info = match Self::parse_start_document(parse_item) {
            Err(e) => return Err(e),
            Ok(doc_info) => doc_info,
        };

        // Read the next XML event, which is expected to be the start of an
        // element. We use a lookahead so that we can be specific about an error
        // if one occurred
        let lookahead_item = parse_item.lookahead();
        let parse_element = match lookahead_item {
            Err(e) => return Err(e),
            Ok(xml_elem) => xml_elem,
        };

        // Now verify that the token we just read starts an element.
        let top_element = match parse_element.event {
            XmlEvent::StartElement{name, attributes, namespace} => {
                let element_info = ElementInfo::new(name, parse_element.lineno, attributes, namespace);
                match Self::parse_element(parse_item, element_info, element_level_info) {
                    Err(e) => return Err(e),
                    Ok(top_elem) => top_elem,
                }
            },

            _ => panic!("FIXME: Expected element, got {:?}", parse_element.event),
        };

        // And, wrap up by making sure things conclude as expected.
        match Self::parse_end_document(parse_item) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        Ok((document_info, top_element))
    }

    /*
     * Parse a StartDocument. Nothing can preceed this
     */
    fn parse_start_document<R>(parse_item: &mut Parser<R>) ->
        Result<DocumentInfo, XmlDocumentError>
    where
        R: Read,
    {
        let parse_element = parse_item.next()?;

        if let XmlEvent::StartDocument{version, encoding, standalone} = parse_element.event {
            Ok(DocumentInfo::new(version, encoding, standalone))
        } else {
            panic!("FIXME: document doesn't start with StartDocument")
        }
    }

    /*
     * Parse an element. We have already seen the XmlStartElement as a lookahead.
     */
    fn parse_element<R>(
        parse_item: &mut Parser<R>, 
        element_info: ElementInfo, 
        element_level_info: &Self::LI
    ) -> Result<<<Self::LI as LevelInfo>::AccumulatorType as Accumulator>::Value, XmlDocumentError>
    where
        R: Read,
    {
        parse_item.skip();
        
        // Create accumulator for this element
        let mut accumulator = element_level_info.create_accumulator(element_info)?;
        
        // Get level info for subelements
        let subelement_level_info = element_level_info.next_level();

        // Parse all subelements until we hit the EndElement
        loop {
            let parse_element = parse_item.lookahead()?;

            match parse_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
println!("Start element {}", name.local_name);
                    let subelement_info = ElementInfo::new(name, parse_element.lineno, attributes, namespace);
                    accumulator.start_subelement(&subelement_info);
                    let subelement_result = Self::parse_element(parse_item, subelement_info, &subelement_level_info)?;
                    
                    accumulator.add_subelement(subelement_result);
                },

                XmlEvent::EndElement{name} => {
                    if accumulator.has_open_subelement() {
                        // We have an element optn at this level, process it
println!("looping with EndElement {}", name.local_name);
                        parse_item.skip();
                        
                        if name.local_name != accumulator.current_subelement_name() {
                            panic!("FIXME: Mismatched element tags: expected {}, got {}", 
                                   accumulator.current_subelement_name(), name.local_name);
                        }
                        
                        accumulator.end_subelement();
                    } else {
                        // No open element on this level, it must be from the
                        // level above.
println!("break from EndElement {}", name.local_name);
                        break;
                    }
                },

                XmlEvent::EndDocument => {
                    if accumulator.has_open_subelement() {
                        panic!("FIXME: Document ended with unclosed subelement");
                    }
                    break;
                }

                XmlEvent::Whitespace(_) | XmlEvent::Characters(_) => {
                    parse_item.skip();
                },

                _ => {
                    panic!("FIXME: Unexpected XML event: {:?}", parse_element.event);
                }
            }
        }

println!("return from parse_element");
        Ok(accumulator.finish())
    }

    /*
     * We expect EndDocument, parsed as a lookahead
     */
    fn parse_end_document<R>(parse_item: &mut Parser<R>) -> Result<(), XmlDocumentError>
    where
        R: Read,
    {
println!("---");
        parse_item.skip();

        loop {
            let parse_element = parse_item.next()?;

            match parse_element.event {
                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {},

                XmlEvent::EndDocument => break,

                _ => panic!("FIXME: Expected end of document but found {:?}", parse_element.event)
            }
        }

        Ok(())
    }
}

/**
 * LevelInfo trait - tracks nesting information passed down to subelements
 */
pub trait LevelInfo {
    type AccumulatorType: Accumulator;

    /// Create the next level info for subelements
    fn next_level(&self) -> Self;
    
    /// Create an accumulator for processing an element at this level
    fn create_accumulator(&self, element_info: ElementInfo) -> 
        Result<Self::AccumulatorType, XmlDocumentError>;
}

/**
 * Accumulator trait - manages processing of an element and its subelements
 */
pub trait Accumulator {
    type Value;

    /// Called when starting to process a subelement
    fn start_subelement(&mut self, element_info: &ElementInfo);
    
    /// Add a completed subelement to this accumulator
    fn add_subelement(&mut self, subelement: Self::Value);
    
    /// Called when finishing processing a subelement
    fn end_subelement(&mut self);
    
    /// Check if we're currently processing a subelement
    fn has_open_subelement(&self) -> bool;
    
    /// Get the name of the current subelement (for error reporting)
    fn current_subelement_name(&self) -> &str;
    
    /// Return the final result for this element
    fn finish(self) -> Self::Value;
    
    /// Get element name (for error reporting)
    fn element_name(&self) -> &str;
    
    /// Get element line number (for error reporting)
    fn element_lineno(&self) -> LineNumber;
}
