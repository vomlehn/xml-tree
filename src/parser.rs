/*
 * A layer built on top of Xml::EventReader to provide look-ahead and line
 * numbers.
 */
// FIXME: should probably rename BaseElement to something like BaseElement.

use std::fmt;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use crate::xml_document_error::XmlDocumentError;

const VERBOSE: bool = false;

/**
 * Parser
 * parse_loc:       Reference counted reference to current parse location
 *                  FIXME: check that this is appropriate
 * pending:         If None, we don't have a lookahead token. Otherwise,
 *                  this is the lookahead token wrapped in Some()
 * event_reader:    Object for reading the next XmlEvent
 */
pub struct Parser<R: Read> {
    parse_loc:      ParseLoc,
    pending:        Option<Result<BaseElement, XmlDocumentError>>,
    event_reader:   EventReader<LinenoReader<R>>,
}

impl<R: Read> Parser<R> {
    pub fn new(reader: R) -> Self {
        let line_reader = LinenoReader::new(reader);
        let parse_loc = ParseLoc::new("TBD".to_string(), line_reader.lineno);
        let event_reader = EventReader::new(line_reader);

        Parser {
            parse_loc,
            pending: None,
            event_reader,
        }
    }

    /**
     * Read the next BaseElement. Each read returns a new value. This
     * BaseElement is always an BaseElement
     *
     * self:    &mut Parser
     *
     * Returns:
     * Ok(BaseElement)
     * Err(XmlDocumentError)
     */
    pub fn next(&mut self) -> Result<BaseElement, XmlDocumentError> {
        let result = self.lookahead()?;
/*
        if let Err(e) = result {
            return Err(e);
        }
*/
        if VERBOSE {
            print!("(next {})", result.name());
        }

        self.skip();
        Ok(result)
    }

    /*
     * Discard the current BaseElement, forcing a fetch of the next item
     * if current() is used. This BaseElement is always an BaseElement
     *
     * self:    &mut Parser
     */
    pub fn skip(&mut self) {
        if VERBOSE {
            print!("(skip)");
        }

        self.pending = None;
    }

    /*
     * Read the next BaseElement from the input stream, without removing
     * it from the stream. This BaseElement is always an BaseElement
     *
     * self:    &mut Parser
     *
     * Returns:
     * Ok(BaseElement)
     * Err(XmlDocumentError)
     */
    pub fn lookahead(&mut self) -> Result<BaseElement, XmlDocumentError> {
        // If we don't have any lookahead token, read another token to be
        // the lookahead token.
        if self.pending.is_none() {
            let parse_loc = self.parse_loc.clone();
            let evt = self.event_reader.next();

            // We tried to read another lookahead token, but we might have
            // gotten an error. Check for this.
            match evt {
                Err(e) => {
                    // Indicate we have something, but that the something
                    // we have is an error
                    let error = XmlDocumentError::XmlError(parse_loc, e);
                    let err = Err(error.clone());
                    let pending_err = Some(Err(error));
                    self.pending = pending_err;
                    err
                },
                Ok(xml_event) => {
                    let item = BaseElement::new(parse_loc, xml_event);

                    if VERBOSE {
                        println!("(lookahead {})", item.name());
                    }

                    let ok = Ok(item.clone());
                    let pending_ok = Some(Ok(item));
                    self.pending = pending_ok;
                    ok
                }
            }
        } else {
            // We do have a pending token. If it's an error, return that. If
            // it's a token, return that, but in either case, don't remove it.
let e = {
            match self.pending.take() {
                None => Err(XmlDocumentError::InternalError(
                    self.parse_loc.clone(),
                    "self.pending is None when it must be Some".to_string(),
                )),
                Some(item) => item,
            }
};

            if VERBOSE {
                println!("(lookahead {})", e.clone().unwrap().name());
            }
e
        }
    }
}

impl<R: Read> fmt::Debug for Parser<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser: {:?} {:?}", self.parse_loc, self.pending)
    }
}

/**
 * Object for reading an std::io::Read implementation, as annotated with
 * a line number.
 */
pub struct LinenoReader<R: Read> {
    inner: R,
    lineno: LineNumber,
}

impl<R: Read> LinenoReader<R> {
    pub fn new(inner: R) -> Self {
        LinenoReader {
            inner,
            lineno: 1,
        }
    }

    pub fn lineno_ref(&self) -> LineNumber {
        self.lineno
    }
}

impl<R: Read> Read for LinenoReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<LineNumber> {
        let bytes_read = self.inner.read(buf)?;
/*
        let mut lineno = self.lineno.borrow_mut();
        *lineno += buf[..bytes_read].iter().filter(|&&c| c == b'\n').count();
*/
        self.lineno += buf[..bytes_read].iter().filter(|&&c| c == b'\n').count();
        Ok(bytes_read)
    }
}

/* Parsing location */
pub type LineNumber = usize;

#[derive(Clone)]
pub struct ParseLoc {
    pub path:   String,
    pub lineno: LineNumber,
}

impl ParseLoc {
    pub fn new(path: String, lineno: LineNumber) -> ParseLoc {
        ParseLoc {
            path,
            lineno,
        }
    }

    pub fn display(&self) -> String {
        self.path.clone() + ":" + &self.lineno.to_string()
    }
}

impl fmt::Display for ParseLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl fmt::Debug for ParseLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

/**
 * An XML element
 * parse_loc:   Location of the start of this element
 * event:       XmlEvent returned by the XML low level parse_item
 */
#[derive(Clone, Debug)]
pub struct BaseElement {
    pub parse_loc:  ParseLoc,
    pub event:      XmlEvent,
}

impl BaseElement {
    fn new(parse_loc: ParseLoc, event: XmlEvent) -> BaseElement {
/*
if let XmlEvent::StartElement{name, attributes, ..} = &event {
    println!("BaseElement.event {}: {:?}", name.local_name, attributes);
}
*/
        BaseElement {
            parse_loc,
            event,
        }
    }

    pub fn name(&self) -> String {
        let result = match &self.event {
            XmlEvent::StartDocument{version: _, encoding: _, standalone: _} =>
                "StartDocument".to_string(),
            XmlEvent::EndDocument => "EndDocument".to_string(),
            XmlEvent::StartElement{name, attributes: _, namespace: _} =>
                format!("StartElement<{}>", name.local_name),
            XmlEvent::EndElement{name} => format!("EndElement<{}>", name.local_name),
            XmlEvent::ProcessingInstruction{name: _, data: _} =>
                "ProcessingInstruction".to_string(),
            XmlEvent::CData(_) => "CData".to_string(),
            XmlEvent::Comment(_) => "Comment".to_string(),
            XmlEvent::Characters(_) => "Characters".to_string(),
            XmlEvent::Whitespace(_) => "Whitespace".to_string(),
        };
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use stdext::function_name;
    use std::io::{BufReader, Cursor};
    use xml::reader::ErrorKind;
    use xml::common::Position;

    use crate::parser::Parser;
    use crate::xml_document_error::XmlDocumentError;

    /*
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
    */

    fn parser_new(input: &str) -> Parser<BufReader<Cursor<Vec<u8>>>> {
        let input_bytes = input.as_bytes().to_vec();
        let cursor = Cursor::new(input_bytes);
        let reader = BufReader::new(cursor);
        Parser::new(reader)
    }

    #[test]
    fn parser_test_empty() {
        println!("Running test {}", function_name!());
        let mut parser = parser_new("");

        match parser.next() {
            Err(XmlDocumentError::XmlError(pos, xml_error)) => {
                let error_pos = xml_error.position();
                match xml_error.kind() {
                    ErrorKind::Syntax(msg) => {
                        println!("Got syntax error at line {}, XML pos {}:{}: {}",
                             pos, error_pos.row, error_pos.column, msg);
                    },
                    ErrorKind::UnexpectedEof => {
                        println!("Got unexpected EOF at line {}, XML pos {}:{}",
                                 pos, error_pos.row, error_pos.column);
                    },
                    other => {
                        println!("Got other XML error: {:?}", other);
                    }
                }
            },
            other => panic!("Unexpected result: {:?}", other),
        };
    }

    #[test]
    fn parser_test_one_element() {
        println!("\nRunning test {}", function_name!());
        const INPUT: &str = concat!("<schema>\n",
            "</schema>\n");
        print!("INPUT:\n{}", INPUT);
        println!("OUTPUT:");

        let mut parser = parser_new(INPUT);

        start_document(&mut parser);
        start_element(&mut parser, &"schema".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"schema".to_string());
        end_document(&mut parser);

        println!();
    }

    #[test]
    fn parser_test_nested_elements() {
        println!("\nRunning test {}", function_name!());
        const INPUT: &str = concat!("<schema>\n",
            "   <one>\n",
            "   </one>\n",
            "   <two>\n",
            "   </two>\n",
            "</schema>\n");
        print!("INPUT:\n{}", INPUT);
        println!("OUTPUT:");

        let mut parser = parser_new(INPUT);

        start_document(&mut parser);
        start_element(&mut parser, &"schema".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"one".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"one".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"schema".to_string());
        end_document(&mut parser);

        println!();
    }

    #[test]
    fn parser_test_nest_and_multiple() {
        println!("\nRunning test {}", function_name!());
        const INPUT: &str = concat!(
            "<schema>\n",
            "   <one>\n",
            "   <two>\n",
            "   <three>\n",
            "   </three>\n",
            "   </two>\n",
            "   </one>\n",
            "   <four>\n",
            "   </four>\n",
            "</schema>\n");
        print!("INPUT:\n{}", INPUT);
        println!("OUTPUT:");

        let mut parser = parser_new(INPUT);

        start_document(&mut parser);
        start_element(&mut parser, &"schema".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"one".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"three".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"three".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"one".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"four".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"four".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"schema".to_string());
        end_document(&mut parser);

        println!();
    }

    #[test]
    fn parser_test_full() {
        println!("\nRunning test {}", function_name!());

        const INPUT: &str = concat!(
            "<!--  \n",
            "\n",
            "Just supply a few elements. This will only work for non-checking code.\n",
            " -->\n",
            "<schema xmlns:xtce=\"http://www.omg.org/spec/XTCE/20180204\" xmlns=\"http://www.w3.org/2001/XMLSchema\" targetNamespace=\"http://www.omg.org/spec/XTCE/20180204\" elementFormDefault=\"qualified\" attributeFormDefault=\"unqualified\" version=\"1.2\">\n",
            "    <one>\n",
            "       <two>\n",
            "          <three>\n",
            "          </three>\n",
            "       </two>\n",
            "    </one>\n",
            "    <four>\n",
            "    </four>\n",
            "</schema>\n");

        print!("INPUT:\n{}", INPUT);
        println!("OUTPUT:");

        let mut parser = parser_new(INPUT);

        start_document(&mut parser);
        start_element_lookahead(&mut parser, &"schema".to_string());

        // top of parse_element
        skip(&mut parser);
        whitespace(&mut parser);

        // top of loop
        start_element_lookahead(&mut parser, &"one".to_string());
        skip(&mut parser);

        whitespace(&mut parser);
        start_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"three".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"three".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"one".to_string());
        whitespace(&mut parser);

        start_element_lookahead(&mut parser, &"four".to_string());
        skip(&mut parser);
        whitespace(&mut parser);

        end_element_lookahead(&mut parser, &"four".to_string());
        skip(&mut parser);
        whitespace(&mut parser);

        end_element(&mut parser, &"schema".to_string());
        end_document(&mut parser);

        println!();
    }

    #[test]
    fn parser_test_lookahead() {
        println!("\nRunning test {}", function_name!());
        const INPUT: &str = concat!("<schema>\n",
            "   <one>\n",
            "   <two>\n",
            "   <three>\n",
            "   </three>\n",
            "   </two>\n",
            "   </one>\n",
            "   <four>\n",
            "   </four>\n",
            "</schema>\n");
        print!("INPUT:\n{}", INPUT);
        println!("OUTPUT:");

        let mut parser = parser_new(INPUT);

        start_document(&mut parser);
        start_element_lookahead(&mut parser, &"schema".to_string());

        // top of parse_element
        skip(&mut parser);

        // top of loop
        whitespace(&mut parser);
        start_element_lookahead(&mut parser, &"one".to_string());
        skip(&mut parser);

        whitespace(&mut parser);
        start_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        start_element(&mut parser, &"three".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"three".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"two".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"one".to_string());
        whitespace(&mut parser);
        start_element_lookahead(&mut parser, &"four".to_string());
        skip(&mut parser);
        whitespace(&mut parser);
        end_element(&mut parser, &"four".to_string());
        whitespace(&mut parser);
        end_element(&mut parser, &"schema".to_string());
        end_document(&mut parser);

        println!();
    }

    fn start_element(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>, element_name: &String) {
        let item = parser.next();
        if let xml::reader::XmlEvent::StartElement { name, .. } =
            &item.as_ref().unwrap().event {
            print!("<{}>", name.local_name);
            assert_eq!(&name.local_name, element_name);
        } else {
            panic!("Failed to get <{}>, got {:?}", element_name, item);
        }
    }

    fn end_element(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>, element_name: &String) {
        let item = parser.next();
        if let xml::reader::XmlEvent::EndElement { name, .. } =
            &item.as_ref().unwrap().event {
            print!("</{}>", name.local_name);
            assert_eq!(&name.local_name, element_name);
        } else {
            panic!("Failed to get </{}>, got {:?}", element_name, item);
        }
    }

    fn whitespace(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>) {
        let item = parser.next();
        if let xml::reader::XmlEvent::Whitespace(ws) = &item.as_ref().unwrap().event {
            print!("{}", ws);
        } else {
            panic!("Failed to get Whitespace, got {:?}", item);
        }
    }

    fn start_element_lookahead(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>, element_name: &String) {
        let item = parser.lookahead();
        if let xml::reader::XmlEvent::StartElement { name, .. } =
            &item.as_ref().unwrap().event {
            print!("<{}>", name.local_name);
            assert_eq!(&name.local_name, element_name);
        } else {
            panic!("Failed to get <{}>, got {:?}", element_name, item);
        }
    }

    fn end_element_lookahead(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>, element_name: &String) {
        let item = parser.lookahead();
        if let xml::reader::XmlEvent::EndElement { name, .. } =
            &item.as_ref().unwrap().event {
            print!("</{}>", name.local_name);
            assert_eq!(&name.local_name, element_name);
        } else {
            panic!("Failed to get </{}>, got {:?}", element_name, item);
        }
    }

    fn skip(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>) {
        parser.skip();
    }

    fn start_document(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>) {
        let item = parser.next();
        if let xml::reader::XmlEvent::StartDocument { version: _, encoding: _, standalone: _ } = &item.as_ref().unwrap().event {
        } else {
            panic!("Failed to get StartDocument, got {:?}", item);
        }
    }

    fn end_document(parser: &mut Parser<BufReader<Cursor<Vec<u8>>>>) {
        let item = parser.next();
        if let xml::reader::XmlEvent::EndDocument = &item.as_ref().unwrap().event {
        } else {
            panic!("Failed to get EndDocument, got {:?}", item);
        }
    }
}
