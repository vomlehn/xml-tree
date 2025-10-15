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
use crate::parse_item::{ParseLoc, Parser};
pub use crate::xml_document_error::XmlDocumentError;

/**
 * ParseXml - Parses an entire XML document
 * LI   Information passed top down during the parse which is specific to each
 *      level. This could be nothing, something simple like a depth of the tree
 *      being parsed, or a reference to one level of the tree being parsed.
 */
pub trait ParseXml<'a>
where
    Self::AC:Accumulator<DocType<'a> = Self>,
{
    type LI: LevelInfo<'a, ParseXmlType = Self>;
    type AC: Accumulator;

    // FIXME: rename to something like parse_from_path
    fn parse_path_base<'b, 'r>(
        &'r mut self,
        path: &'b str,
        element_level_info: &Self::LI,
//    ) -> Result<(DocumentInfo, <<Self::LI as LevelInfo<'a>>::AccumulatorType<'r> as Accumulator>::Value), XmlDocumentError>
    ) -> Result<(DocumentInfo, <<Self::LI as LevelInfo<'a>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    {
        let file = match File::open(path) {
            Err(e) => {
                panic!("FIXME: unable to open {}: {}", path, e);
            },
            Ok(f) => f,
        };
        let reader = BufReader::new(file);
        self.parse_base::<File>(reader, element_level_info)
    }

    /**
     * Top-level trait for parsing an XML document. The document is
     * provided via a reader built on the Read attribute.
     */
    fn parse_base<'r, R>(
        &'r mut self,
        buf_reader: BufReader<R>,
        element_level_info: &Self::LI,
    ) -> Result<(DocumentInfo, <<Self::LI as LevelInfo<'a>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
        // Create the factory using the reader and XML definition
        let mut parse_item = Parser::new(buf_reader);
        self.parse_document(&mut parse_item, &element_level_info)
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

    fn parse_document<'r, R>(
        &'r mut self,
        parse_item: &mut Parser<R>, 
        element_level_info: &Self::LI
    ) -> Result<(DocumentInfo, <<Self::LI as LevelInfo<'a>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
//println!("---> entering parse_document");
        let document_info = match self.parse_start_document(parse_item) {
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
                let element_info = ElementInfo::new(name, parse_element.parse_loc, attributes, namespace);
                match self.parse_element(parse_item, element_info, element_level_info) {
                    Err(e) => return Err(e),
                    Ok(top_elem) => top_elem,
                }
            },

            _ => panic!("FIXME: Expected element, got {:?}", parse_element.event),
        };

        // And, wrap up by making sure things conclude as expected.
        match self.parse_end_document(parse_item) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        Ok((document_info, top_element.expect("FIXME: figure this out")))
    }

    /*
     * Parse a StartDocument. Nothing can preceed this
     */
    fn parse_start_document<R>(&mut self, parse_item: &mut Parser<R>) ->
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
        &mut self,
        parse_item: &mut Parser<R>, 
        element_info: ElementInfo, 
        element_level_info: &Self::LI
    ) -> Result<Option<<<Self::LI as LevelInfo<'a>>::AccumulatorType as Accumulator>::Value>, XmlDocumentError>
    where
        R: Read,
    {
        parse_item.skip();
        
        // Create accumulator for this element
        let mut accumulator = element_level_info.create_accumulator(self,
            element_info)?;
        
        // Get level info for subelements
        let subelement_level_info = element_level_info.next_level();

        // Parse all subelements until we hit the EndElement
        loop {
println!("loop start");
            let parse_element = parse_item.lookahead()?;

            match parse_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
let x = name.local_name.clone();
                    let subelement_info = ElementInfo::new(name,
                        parse_element.parse_loc, attributes, namespace);
                    accumulator.start_subelement(self, &subelement_info);
println!("push {}", x);
                    let subelement_result = self.parse_element(parse_item,
                        subelement_info, &subelement_level_info)?;
println!("pop {}", x);
                    
                    match subelement_result {
                        None => println!("subelement None"),
                        Some(result) => {
                            println!("subelement Some");
                            accumulator.add_subelement(self, result);
                        },
                    };
                },

                XmlEvent::EndElement{name} => {
                    if accumulator.has_open_subelement() {
                        // We have an element open at this level, process it
println!("process element {} at level", name.local_name);
                        parse_item.skip();
                        
                        if name.local_name != accumulator.current_subelement_name() {
                            panic!("FIXME: Mismatched element tags: expected {}, got {}", 
                                   accumulator.current_subelement_name(), name.local_name);
                        }
                        
                        accumulator.end_subelement(self);
                    } else {
println!("pop to upper element for {}", name.local_name);
                        // No open element on this level, it must be from the
                        // level above.
                        return Ok(None);
                    }
                },

                XmlEvent::EndDocument => {
                    if accumulator.has_open_subelement() {
                        panic!("FIXME: Document ended with unclosed subelement");
                    }
                    break;
                }

                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {
                    parse_item.skip();
                },

                _ => {
                    panic!("FIXME: Unexpected XML event: {:?}", parse_element.event);
                }
            }
        }

        Ok(Some(accumulator.finish(self)))
    }

/*
    fn parse_element<R>(
        &mut self,
        parse_item: &mut Parser<R>, 
        element_info: ElementInfo, 
        element_level_info: &Self::LI
    ) -> Result<Option<<<Self::LI as LevelInfo<'a>>::AccumulatorType as Accumulator>::Value>, XmlDocumentError>
    where
        R: Read,
    {
        parse_item.skip();
        
        // Create accumulator for this element
        let mut accumulator = element_level_info.create_accumulator(self,
            element_info)?;
        
        // Get level info for subelements
        let subelement_level_info = element_level_info.next_level();

        // Parse all subelements until we hit the EndElement
        loop {
println!("loop start");
            let parse_element = parse_item.lookahead()?;

            match parse_element.event {
                XmlEvent::StartElement{name, attributes, namespace} => {
let x = name.local_name.clone();
                    let subelement_info = ElementInfo::new(name,
                        parse_element.parse_loc, attributes, namespace);
                    accumulator.start_subelement(self, &subelement_info);
println!("push {}", x);
                    let subelement_result = self.parse_element(parse_item,
                        subelement_info, &subelement_level_info)?;
println!("pop {}", x);
                    
                    match subelement_result {
                        None => println!("subelement None"),
                        Some(result) => {
                            println!("subelement Some");
                            accumulator.add_subelement(self, result);
                        },
                    };
                },

                XmlEvent::EndElement{name} => {
                    if accumulator.has_open_subelement() {
                        // We have an element open at this level, process it
println!("process element {} at level", name.local_name);
                        parse_item.skip();
                        
                        if name.local_name != accumulator.current_subelement_name() {
                            panic!("FIXME: Mismatched element tags: expected {}, got {}", 
                                   accumulator.current_subelement_name(), name.local_name);
                        }
                        
                        accumulator.end_subelement(self);
                    } else {
println!("pop to upper element for {}", name.local_name);
                        // No open element on this level, it must be from the
                        // level above.
                        return Ok(None);
                    }
                },

                XmlEvent::EndDocument => {
                    if accumulator.has_open_subelement() {
                        panic!("FIXME: Document ended with unclosed subelement");
                    }
                    break;
                }

                XmlEvent::Whitespace(_) |
                    XmlEvent::Characters(_) => {
                    parse_item.skip();
                },

                _ => {
                    panic!("FIXME: Unexpected XML event: {:?}", parse_element.event);
                }
            }
        }

        Ok(Some(accumulator.finish(self)))
    }
*/

    /*
     * We expect EndDocument, parsed as a lookahead
     */
    fn parse_end_document<R>(&mut self, parse_item: &mut Parser<R>) -> Result<(), XmlDocumentError>
    where
        R: Read,
    {
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
 * LevelInfo<'_> trait - tracks nesting information passed down to subelements
 */
pub trait LevelInfo<'a> {
    type ParseXmlType: ParseXml<'a, LI = Self>;
    type AccumulatorType: Accumulator<DocType<'a> = Self::ParseXmlType>;
//    where
//        Self: 'c;
//    type AccumulatorType: Accumulator;

    /// Create the next level info for subelements
    fn next_level(&self) -> Self;
    
    /// Create an accumulator for processing an element at this level. This is called
    /// when we start the processing.
    fn create_accumulator(&self, parse_xml: &mut Self::ParseXmlType, element_info: ElementInfo) -> 
        Result<Self::AccumulatorType, XmlDocumentError>;
}

/**
 * Accumulator trait - manages processing of an element and its subelements
 */
pub trait Accumulator {
    type Value;
    type DocType<'a>: ParseXml<'a> + ?Sized;

    /// Called when starting to process a subelement
    fn start_subelement(&mut self, parse_xml: &mut Self::DocType<'_>, element_info: &ElementInfo);
    
    /// Called when finishing processing a subelement
    fn end_subelement(&mut self, parse_xml: &mut Self::DocType<'_>);
    
    /// Add a completed subelement to this accumulator
    fn add_subelement(&mut self, parse_xml: &mut Self::DocType<'_>, subelement: Self::Value);
    
    /// Return and consume the final result for this element
    fn finish(self, parse_xml: &mut Self::DocType<'_>) -> Self::Value;
    
    /// Determine whether we're currently processing a subelement
    /// Returns: true if we are nested in a subelement, false otherwise
    fn has_open_subelement(&self) -> bool;
    
    /// Get the name of the current subelement
    fn current_subelement_name(&self) -> &str;
    
    /// Get element name (for error reporting)
    fn element_name(&self) -> &str;
    
    /// Get element line number (for error reporting)
    fn parse_loc(&self) -> ParseLoc;
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::io::{BufReader, Cursor, Read};
    use std::ops::{ControlFlow, FromResidual, Try};
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::banner::print_banner_file;
    use crate::element::{Element, ElementInfo, element_info_display};
    use crate::misc::{nl_indent, owned_name_display, vec_display, XmlDisplay};
    use crate::parse_item::ParseLoc;
    pub use crate::xml_document_error::XmlDocumentError;
    use crate::parse_xml::{Accumulator, LevelInfo, ParseXml};
    use crate::document::DocumentInfo;

    const TREE_DEPTH: usize = 2;

    // FIXME: unignore tests
    #[test]
    #[ignore]
    fn parse_xml_test1() {
        // FIXME: check return value
        let input = r#""#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    #[test]
    fn parse_xml_test2() {
        // FIXME: check return value
        let input = r#"<one />"#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    #[test]
    #[ignore]
    fn parse_xml_test3() {
        // FIXME: check return value
        let input = r#"<one>
            </one>"#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    #[test]
    #[ignore]
    fn parse_xml_test4() {
        // FIXME: check return value
        let input = r#"<one>
                <two>
                </two>
            </one>"#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    #[test]
    #[ignore]
    fn parse_xml_test5() {
        // FIXME: check return value
        let input = r#"<one>
                <two>
                </two>
                <three>
                </three>
            </one>"#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    #[test]
    #[ignore]
    fn parse_xml_test6() {
        // FIXME: check return value
        let input = r#"<one>
                <two>
                </twoo>
                <three>
                </three>
            </one>"#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    #[test]
    #[ignore]
    fn parse_xml_test7() {
        // FIXME: check return value
        let input = r#"<one>
                <two>
                </two>
                <three>
                </threee>
            </one>"#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    #[test]
    #[ignore]
    fn parse_xml_test8() {
        // FIXME: check return value
        let input = r#"<one>
                <two>
                    <three>
                    </three>
                    <four>
                    </four>
                    <five>
                    </five>
                </two>
                <six>
                    <seven>
                    </seven>
                </six>
                <eight>
                    <nine>
                    </nine>
                    <ten>
                    </ten>
                </eight>
            </one>"#;
        let result = create_test(input);
        println!("test result: {:?}", result);
    }

    pub struct ParseTestParams {
    }

    fn create_test(input_str: &str) -> Result<(), XmlDocumentError> {
        println!("Start test parse of {}", input_str);
        let cursor = Cursor::new(input_str);
        let buf_reader = BufReader::new(cursor);

        let params = ParseTestParams {
        };
        let owned_name = OwnedName {
            local_name: "schema".to_string(),
            namespace:  None,
            prefix:     None,
        };
        let namespace = Namespace::empty();
        let element_info = ElementInfo::new(owned_name, ParseLoc::new("".to_string(), 0), vec!(), namespace);
        // FIXME: remove depth?
        let element = TestElement::new(element_info, 0, vec!(), vec!(), vec!(), vec!());
        let root: Box<dyn Element> = Box::new(element);
        let xtce_level_info = TestLevelInfo::new(&root);

        let document_info = DocumentInfo::new(XmlVersion::Version10, "encoding".to_string(), None);
//        let mut output = std::io::stdout();
        let mut output = String::new();

        let mut parse_schema = ParseTest::new(document_info, root, &mut output);
        // FIXME: Handle returned error
        let _ = parse_schema.parse(&params, buf_reader, &xtce_level_info);

        println!("End XTCE parse");
        
        Ok(())
    }

    /*
     * Parse an input stream of XSD code and generate Rust code. That code is
     * then used to guide the parsing of XML code. The XSD is actually XML.
     */
    pub struct ParseTest {
/*
        pub document_info:  DocumentInfo,
        pub root:           Box<dyn Element>,
        pub output:         &'a mut String,
*/
    }

    impl<'a> ParseTest {
        pub fn new(_document_info: DocumentInfo, _root: Box<dyn Element>, _output: &'a mut String) -> ParseTest {
            ParseTest {
/*
                document_info,
                root,
                output,
*/
            }
        }

/*
        pub fn parse_path<'b> (
            &mut self,
            params:             &ParseTestParams,
            path:               &'b str,
            element_level_info: &<ParseTest<'a> as ParseXml<'a>>::LI,
        ) -> Result<(DocumentInfo, <<<ParseTest<'_> as ParseXml<'_>>::LI as LevelInfo<'_>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
        {
            // FIXME: check for error
            let _ = self.display_start(&params);
            let res = self.parse_path_base(path, element_level_info)?;
            self.display_end();
            Ok(res)
        }
*/

        pub fn parse<'b, R>(
            &mut self,
            params:             &ParseTestParams,
            buf_reader:         BufReader<R>,
            element_level_info: &<ParseTest as ParseXml<'b>>::LI,
        ) -> Result<(DocumentInfo, <<<ParseTest as ParseXml<'b>>::LI as LevelInfo<'b>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
        where
            R: Read,
        {
            // FIXME: check for error
            let _ = self.display_start(&params);
            let res = self.parse_base(buf_reader, element_level_info)?;
            self.display_end();
            Ok(res)
        }

        fn display_start(&mut self, _params: &ParseTestParams) -> fmt::Result {
            let depth = 0;
            self.front_matter_display(depth)?;

            let indent_str = nl_indent(depth);
            // FIXME: check for error
            print!("{}lazy_static! {{", indent_str);

//            self.static_parse_test_display(depth + 1, params.const_name, params.test_type, params.test_name)?;

            Ok(())
        }

        fn front_matter_display(&mut self, depth: usize) -> fmt::Result {
            let front_matter: Vec::<&str> = vec!(
                "// FIXME: insert banner",
                "// Auto-generated file",
                "use lazy_static::lazy_static;", 
                "use std::collections::BTreeMap;",
                "", 
                "use xml::common::XmlVersion;",
                "use xml::name::OwnedName;",
                "use xml::namespace::Namespace;",
                "",
                "use crate::xml_document::TreeElement;", 
                "use crate::parse_tree::{DocumentInfo, ElementInfo};",
                "use crate::parse_test::ParseTest;", 
                "use crate::XmlTree;",
                "", 
            );

            print_banner_file()?;

            let indent_str = nl_indent(depth);

            for front in front_matter {
                // FIXME: check for error
                print!("{}{}", indent_str, front);
            }

            Ok(())
        }

/*
        pub fn static_parse_test_display(&mut self, depth: usize, const_name: &str, test_type: &str, test_name: &str) -> fmt::Result {
            let indent_str = nl_indent(depth);
            // FIXME: check for error
            print!("{}pub static ref {const_name}: {test_type}<'_> = {test_name}::new(", indent_str);

            let indent_str = nl_indent(depth + 1);
            for name in [const_name, test_type, test_name] {
            // FIXME: check for error
                print!("{}\"{}\",", indent_str, name);
            }

            Ok(())
        }
*/

        pub fn display_end(&mut self, ) {
            // FIXME: check for error
            let _ = self.back_matter_display(1);
        }

        fn back_matter_display(&mut self, depth: usize) -> fmt::Result {
            // FIXME: check for error
            print!("{});", nl_indent(depth));
            print!("{}}}", nl_indent(depth - 1));
            Ok(())
        // FIXME: is this needed?
        // write!(f, "\n")
        }
    }

    impl<'a> ParseXml<'a> for ParseTest {
        type LI = TestLevelInfo;
        type AC = TestAccumulator;
    }

    /*
    pub struct ParseTestPrint {
    }

    impl fmt::Display for ParseTest {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let depth = 0;
            front_matter_display(f, depth)?;

            let indent_str = nl_indent(depth);
            write!(f, "{}lazy_static! {{", indent_str)?;

            static_parse_test_display(f, depth + 1, self.const_name, self.test_type, self.test_name)?;

    unimplemented!();
    //        print_walk(f, depth + 2, &self.xml_document)?;

    //        back_matter_display(f, 1)?;
    //        Ok(())
        }
    }

    impl fmt::Debug for ParseTest {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "inner \"{}\" (\"{}\")", self.const_name, self.test_name)
    //        writeln!(f, "inner \"{}\" (\"{}\")", self.const_name, self.test_name)?;
    //        writeln!(f, "xml_document {:?}", self.xml_document)
        }
    }

    fn front_matter_display(f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        let front_matter: Vec::<&str> = vec!(
            "// FIXME: insert banner",
            "// Auto-generated file",
            "use lazy_static::lazy_static;", 
            "use std::collections::BTreeMap;",
            "", 
            "use xml::common::XmlVersion;",
            "use xml::name::OwnedName;",
            "use xml::namespace::Namespace;",
            "",
            "use crate::xml_document::TreeElement;", 
            "use crate::parse_tree::{DocumentInfo, ElementInfo};",
            "use crate::parse_test::ParseTest;", 
            "use crate::XmlTree;",
            "", 
        );

        write_banner_file(f)?;

        let indent_str = nl_indent(depth);

        for front in front_matter {
            write!(f, "{}{}", indent_str, front)?;
        }

        Ok(())
    }

    fn static_parse_test_display(f: &mut fmt::Formatter, depth: usize, const_name: &str, test_type: &str, test_name: &str) -> fmt::Result {
        let indent_str = nl_indent(depth);
        write!(f, "{}pub static ref {const_name}: {test_type}<'_> = {test_type}::new(", indent_str)?;

        let indent_str = nl_indent(depth + 1);
        for name in [const_name, test_type, test_name] {
            write!(f, "{}\"{}\",", indent_str, name)?;
        }

        Ok(())
    }
    */

    /*
    fn back_matter_display(f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        write!(f, "{});", nl_indent(depth))?;
        write!(f, "{}}}", nl_indent(depth - 1))
    // FIXME: is this needed?
    // write!(f, "\n")
    }

    */

    impl Try for ParseTest 
    where
        for<'a> ParseTest: ParseXml<'a>,
    {
    //    type Output<'b> = <<ParseTest as ParseXml<'a>>::AC as Accumulator>::Value;
        type Output = <<ParseTest as ParseXml<'static>>::AC as Accumulator>::Value;
        type Residual = XmlDocumentError;

        fn from_output(_: <Self as Try>::Output) -> Self
        { todo!() }
        fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
        { todo!() }
    }

/*
    impl Try for ParseTest 
    where
        for<'a> ParseTest: ParseXml<'a>,
    {
    //    type Output<'b> = <<ParseTest as ParseXml<'a>>::AC as Accumulator>::Value;
        type Output = <<ParseTest as ParseXml<'a>>::AC as Accumulator>::Value;
        type Residual = XmlDocumentError;

        fn from_output(_: <Self as Try>::Output) -> Self
        { todo!() }
        fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
        { todo!() }
    }
*/

    impl<'a> FromResidual<XmlDocumentError> for ParseTest {
        fn from_residual(_: XmlDocumentError) -> Self
        { todo!() }
    }
/*
    impl<'a> FromResidual for ParseTest {
        fn from_residual(_: <ParseTest as Try>::Residual) -> Self
        { todo!() }
    }
*/

    /// LevelInfo<'_> that tracks depth for indented output
    #[derive(Debug, Clone)]
    pub struct TestLevelInfo {
        depth: usize,
    }

    impl TestLevelInfo {
        pub fn new(_test: &Box<dyn Element>) -> Self {
            TestLevelInfo { depth: 0 }
        }
    }

    impl<'a> LevelInfo<'a> for TestLevelInfo {
        type ParseXmlType = ParseTest;
        type AccumulatorType = TestAccumulator;

        fn next_level(&self) -> Self {
            TestLevelInfo { depth: self.depth + 1 }
        }

        fn create_accumulator(&self, parse_xml: &mut Self::ParseXmlType, element_info: ElementInfo) ->
            Result<TestAccumulator, XmlDocumentError>
        {
            Ok(TestAccumulator::new(element_info, self.depth, parse_xml))
        }
    }

    /// Accumulator that just echoes structure (doesn't build elements)
    pub struct TestAccumulator {
        element: TestElement,
        element_name: String,
        parse_loc: ParseLoc,
        depth: usize,
        current_subelement_name: Option<String>,
    }

    impl TestAccumulator {
        pub fn new(element_info: ElementInfo, depth: usize, _parse_xml: &mut ParseTest) -> Self {
            let ei = element_info.clone();
            let element = TestElement::new(ei, depth, vec![], vec![], vec![], vec![]);
            print!("{}", element);

            TestAccumulator {
                element,
                // FIXME: should use element.name()
                element_name: element_info.owned_name.local_name.clone(),
                parse_loc: element_info.parse_loc,
                depth: depth,
                current_subelement_name: None,
            }
        }
    }

    impl Accumulator for TestAccumulator {
        type Value = ();  // Test doesn't return meaningful data
        type DocType<'a> = ParseTest;

        /*
         * Note that we have started a sublement
         */
        fn start_subelement(&mut self, _parse_xml: &mut ParseTest, element_info: &ElementInfo) {
            // FIXME: probably needs to be fully qualified
            // FIXME: propagate to other parse_.*() code
            self.current_subelement_name = Some(element_info.owned_name.local_name.clone());
        }
        
        fn add_subelement(&mut self, _parse_xml: &mut ParseTest, _subelement: ()) {
            // For echo, subelements have already been printed
            // We don't need to do anything with the () value
        }
        
        fn end_subelement(&mut self, _parse_xml: &mut ParseTest) {
            // FIXME: what's this for?
            if let Some(_name) = &self.current_subelement_name {
            }
            self.current_subelement_name = None;
            print!(",");
        }
        
        fn finish(self, _parse_xml: &mut ParseTest) -> Self::Value {
            // FIXME: return error
            let _ = self.element.display_end(self.depth);
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

    #[derive(Clone)]
    pub struct TestElement {
        pub element_info:   ElementInfo,
        pub depth:          usize,
        pub before_element: Vec<XmlEvent>,
        pub content:        Vec<XmlEvent>,
        pub after_element:  Vec<XmlEvent>,
        pub subelements:    Vec<Box<dyn Element>>,
    }

    impl TestElement {
        pub fn new(element_info: ElementInfo,
            depth:          usize,
            before_element: Vec::<XmlEvent>,
            content: Vec::<XmlEvent>,
            after_element: Vec::<XmlEvent>,
            subelements: Vec<Box<dyn Element>>) -> TestElement {
            TestElement {
                element_info,
                depth,
                subelements,
                before_element,
                content,
                after_element,
            }
        }

        fn display_start(&self, f: &mut fmt::Formatter::<'_>, depth: usize) -> fmt::Result {
            let depth0 = TREE_DEPTH + 3 * depth;
            let depth1 = depth0 + 1;

            // FIXME: return error code
            let _ = write!(f, "{}vec!(Box::new(TestElement::new(",
                nl_indent(depth0));

            let owned_name = OwnedName {
                local_name: self.name().to_string(),
                namespace:  None,
                prefix:     None,
            };
            owned_name_display(f, depth1, &owned_name)?;

            let element_info = ElementInfo {
                parse_loc:     ParseLoc::new("".to_string(), 0),
                owned_name: owned_name,
            };
            element_info_display(f, depth1, &element_info)?;
            write!(f, "{}", nl_indent(depth1))?;
            vec_display::<XmlEvent>(f, depth1, &self.before_element)?;
            write!(f, ", ")?;
            vec_display::<XmlEvent>(f, depth1, &self.content)?;
            write!(f, ", ")?;
            vec_display::<XmlEvent>(f, depth1, &self.after_element)?;
            write!(f, ",")?;
            write!(f, "{}vec!(", nl_indent(depth1 + 1))
        }

        fn display_end(&self, depth: usize) -> fmt::Result {
            let depth0 = TREE_DEPTH + 3 * depth;
            let depth1 = depth0 + 1;
            let depth2 = depth1 + 2;

            print!("{})", nl_indent(depth2));
            print!("{})", nl_indent(depth1));
            print!("{})", nl_indent(depth0));
                // FIXME: return error
            Ok(())
        }
    }

    impl Default for TestElement {
        fn default() -> TestElement {
            TestElement {
                element_info: ElementInfo {
                    owned_name: OwnedName {
                        local_name: "".to_string(),
                        namespace:  None,
                        prefix:     None
                    },
                    parse_loc:     ParseLoc::new("".to_string(), 0),
                },
                depth: 0,
                subelements: vec!(),
                before_element: vec!(),
                content: vec!(),
                after_element: vec!(),
            }
        }
    }

    impl fmt::Display for TestElement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.display(f, self.depth)
        }
    }

    impl fmt::Debug for TestElement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.debug(f, self.depth)
        }
    }

    impl Element for TestElement {
        fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
            self.display_start(f, depth)
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

    impl XmlDisplay for TestElement {
        fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

            write!(f, "{}Box::new(TestElement::new(", nl_indent(depth))
                .expect("Unable to write Box::new");

            let element_info = ElementInfo {
                parse_loc: ParseLoc::new("".to_string(), 0),
                owned_name: OwnedName {
                            local_name: self.name().to_string(),
                            namespace:  None,
                            prefix:     None,
                },
            };

            owned_name_display(f, depth + 1, &element_info.owned_name)?;
            element_info_display(f, depth + 1, &element_info)?;
            write!(f, "{}vec!(), vec!(), vec!(),", nl_indent(depth + 1))?;

            write!(f, "{}vec!(", nl_indent(depth + 1))
        }
    }

/*
    use std::io::{BufReader, Read};
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;

    use crate::{Accumulator, DocumentInfo, Element, ParseLoc, ParseXml};
    use crate::parse_xml::LevelInfo;
    use crate::element::{ElementInfo};
    use crate::xml_document_error::XmlDocumentError;

    #[test]
    fn parse_xml_test1() {
    }

    struct ParseTestParams {
    }

    fn create_test(input_file: &str) -> Result<(), XmlDocumentError> {
        println!("Start test parse of {}", input_file);
        let params = ParseTestParams {
        };
        let owned_name = OwnedName {
            local_name: "schema".to_string(),
            namespace:  None,
            prefix:     None,
        };
        let namespace = Namespace::empty();
        let element_info = ElementInfo::new(owned_name, ParseLoc::new("".to_string(), 0), vec!(), namespace);
        // FIXME: remove depth?
        let element = TestElement::new(element_info, 0, vec!(), vec!(), vec!(), vec!());
        let root: Box<dyn Element> = Box::new(element);
        let xtce_level_info = TestLevelInfo::new(&root);

        let document_info = DocumentInfo::new(XmlVersion::Version10, "encoding".to_string(), None);
        let mut output = std::io::stdout();

        let mut parse_schema = ParseTest::new(document_info, root, &mut output);
        // FIXME: Handle returned error
        let _ = parse_schema.parse_path(&params, input_file, &xtce_level_info);

        println!("End XTCE parse");
        
        Ok(())
    }

    pub struct ParseTest {
        pub output:         &'a mut String,
    }

    impl<'a> ParseTest {
        pub fn new(document_info: DocumentInfo, root: Box<dyn Element>,
            output: &'a mut String) -> Self {
            ParseTest {
                document_info,
                root,
                output,
            }
        }

        pub fn parse_path<'b>(
            path: &'b str,
            element_level_info: &<ParseTest as ParseXml<'b>>::LI,
        ) -> Result<(DocumentInfo, <<<ParseTest as ParseXml<'b>>::LI as LevelInfo<'b>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
        {
            Self::parse_path_base(path, element_level_info)
        }

        pub fn parse<'b, R>(
            buf_reader: BufReader<R>,
            element_level_info: &<ParseTest as ParseXml>::LI,
        ) -> Result<(DocumentInfo, <<<ParseTest as ParseXml<'b>>::LI as LevelInfo<'b>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
        where
            R: Read,
        {
            Self::parse_base(buf_reader, element_level_info)
        }
    }

    /// LevelInfo that doesn't track depth or any other information
    #[derive(Debug, Clone)]
    pub struct TestLevelInfo;

    impl TestLevelInfo {
        pub fn new() -> Self {
            TestLevelInfo
        }
    }

    impl<'a> LevelInfo<'a> for TestLevelInfo {
        type ParseXmlType = ParseTest<'a>;
        type AccumulatorType = TestAccumulator;

        fn next_level(&self) -> Self {
            TestLevelInfo  // Always the same
        }
        
        fn create_accumulator(&self, element: &Box<&dyn Element>, element_info: ElementInfo) -> 
            Result<TestAccumulator, XmlDocumentError> 
        {
            Ok(TestAccumulator::new(element_info))
        }
    }

    /// Test accumulator that just validates structure
    pub struct TestAccumulator {
        element_name: String,
        parse_loc: ParseLoc,
        has_subelement: bool,
    }

    impl TestAccumulator {
        pub fn new(element_info: ElementInfo) -> Self {
            TestAccumulator {
                element_name: element_info.owned_name.local_name.clone(),
                parse_loc: element_info.parse_loc,
                has_subelement: false,
            }
        }
    }

    impl Accumulator for TestAccumulator {
        type Value = ();
        type DocType<'a> = ParseTest<'a>;

        fn start_subelement(&mut self, parse_xml: &mut Self::DocType<'_>, element_info: &ElementInfo) {
            self.has_subelement = true;
        }
        
        fn add_subelement(&mut self, parse_xml: &mut Self::DocType<'_>, _subelement: ()) {
            // Just validate that we're in the right state
        }
        
        fn end_subelement(&mut self, parse_xml: &mut Self::DocType<'_>) {
            self.has_subelement = false;
        }
        
        fn finish(self, parse_xml: &mut Self::DocType<'_>) -> () {
            ()
        }
        
        fn has_open_subelement(&self) -> bool {
            self.has_subelement
        }
        
        fn current_subelement_name(&self) -> &str {
            &self.element_name  // Test implementation
        }
        
        fn element_name(&self) -> &str {
            &self.element_name
        }
        
        fn parse_loc(&self) -> ParseLoc {
            self.parse_loc
        }
    }
*/
}
