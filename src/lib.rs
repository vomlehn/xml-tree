use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

pub mod parser;
pub mod xml_tree_error;

pub use crate::parser::{LineNumber, Parser};
pub use crate::xml_tree_error::XmlTreeError;

pub struct ElementDesc<'a> {
    name:                   &'a str,
    allowable_subelements:  &'a mut[ElementDescRef<'a>],
}

#[derive(Debug)]
pub struct ElementDescTree<'a> {
    pub root:            ElementDescRef<'a>,
    pub element_descs:    &'a [ElementDesc<'a>],
}

#[derive(Debug)]
enum ElementDescRef<'a> {
    Name(&'a str),
    Ref(&'a ElementDesc<'a>),
}

impl<'a> ElementDescRef<'a> {
    fn name(&self) -> &str {
        match self {
            ElementDescRef::Name(n) => *n,
            ElementDescRef::Ref(r) => r.name,
        }
    }

    fn r#ref(&self) -> Result<&ElementDesc, XmlTreeError> {
        match self {
            ElementDescRef::Name(n) => Err(XmlTreeError::RefNotSet(n.to_string())),
            ElementDescRef::Ref(r) => Ok(r),
        }
    }
}

// FIXME: for testing only, remove me
static test_element_desc_tree: ElementDescTree = ElementDescTree {
    root:           ElementDescRef::Name("a1"),
    element_descs:   &[
        ElementDesc {
            name:   "a1",
            allowable_subelements: &mut [ElementDescRef::Name("a2")],
        },
        ElementDesc {
            name:   "a2",
            allowable_subelements: &mut [ElementDescRef::Name("a1")],
        }
    ]
};

impl<'a> ElementDesc<'a> {
    pub fn position(&self, target: &String) -> Option<usize> {
        let mut pos: usize = 0;

        for element in self.allowable_subelements {
            let name = match element {
                // FIXME: this is an internal consistency failure
                ElementDescRef::Name(name) => *name,
                ElementDescRef::Ref(r) => r.name,
            };
            if name == target.as_str() {
                    return Some(pos);
            };
            pos += 1;
        }

        return None
    }

    fn fmt_no_circular(&self, f: &mut fmt::Formatter<'_>, active: &mut Vec<&String>) -> fmt::Result {
        let mut sep_subelem = "";

        write!(f, "{}:\n", self.name)?;
        write!(f, "   [")?;

        for element in self.allowable_subelements {
            let element_name = element.name();

            for name in &mut *active {
                if *name == element_name {
                    eprintln!("Circular dependency starting at {}", name);
                    std::process::exit(1);
                }
            }

            write!(f, "{}{}", sep_subelem, element_name)?;
            sep_subelem = ", ";
        }

        write!(f, "]\n")?;
       
        for element in self.allowable_subelements {
            let element_name = element.name();
            write!(f, "{:?}", element_name)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for ElementDesc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut active = Vec::<&String>::new();
        self.fmt_no_circular(f, &mut active)
    }
}

impl<'a> fmt::Debug for ElementDesc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.allowable_subelements)
    }
}

#[derive(Clone, Debug)]
struct ElementInfo {
    lineno:                 LineNumber,
    attributes:             Vec<OwnedAttribute>,
    namespace:              Namespace,
}

impl ElementInfo {
    fn new(lineno: LineNumber, attributes: Vec<OwnedAttribute>, namespace: Namespace) -> ElementInfo {
        ElementInfo {
            lineno:     lineno,
            attributes: attributes,
            namespace:  namespace,
        }
    }
}

/*
 * Define the structure used to construct the tree for the parsed document.
 */
#[derive(Clone, Debug)]
pub struct Element {
    name:                   OwnedName,
    depth:                  usize,
    element_info:           ElementInfo,
    pub subelements:        Vec<Element>,
    before_comments:        Vec<String>,
    after_comments:         Vec<String>,
}

impl Element {
    fn new(name: OwnedName, depth: usize, element_info: ElementInfo) -> Element {
        Element {
            name:               name,
            depth:              depth,
            element_info:       element_info,
            subelements:        Vec::<Element>::new(),
            before_comments:    Vec::<String>::new(),
            after_comments:     Vec::<String>::new(),
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

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const INDENT_STR: &str = "   ";
        let indent_string = INDENT_STR.to_string().repeat(self.depth);

        write!(f, "{}<{}", indent_string, self.name.local_name)?;
        for attribute in self.element_info.attributes.clone() {
            write!(f, " {}={}", attribute.name.local_name, attribute.value)?;
        }

        if self.subelements.len() == 0 {
            write!(f, " /> (line {})\n", self.element_info.lineno)?;
        } else {
            write!(f, "> (line {})\n", self.element_info.lineno)?;

            for element in &self.subelements {
                element.fmt(f)?;
            }

            write!(f, "{}</{}>\n", indent_string, self.name.local_name)?;
        }


        Ok(())
    }
}

#[derive(Clone, Debug)]
struct DocumentInfo {
    version:    XmlVersion,
    encoding:   String,
    standalone: Option<bool>,
}

impl DocumentInfo {
    fn new(version: XmlVersion, encoding: String, standalone: Option<bool>) ->
        DocumentInfo {
        DocumentInfo {
            version:    version,
            encoding:   encoding,
            standalone: standalone,
        }
    }
}

#[derive(Debug)]
pub struct XmlTree {
    document_info:  DocumentInfo,
    tree:           Element,
}

impl XmlTree {
    pub fn new(path: String, root: &ElementDescTree) ->
        Result<XmlTree, XmlTreeError> {
        let file = match File::open(path) {
            Err(e) => return Err(XmlTreeError::XmlError(0, Box::new(e))),
            Ok(f) => f,
        };
        let buf_reader = BufReader::new(file);
        Self::new_from_reader(buf_reader, &test_element_desc_tree)
    }

    pub fn new_from_reader<R: Read>(buf_reader: BufReader<R>, element_desc_tree: &ElementDescTree) ->
        Result<XmlTree, XmlTreeError> {
        if element_desc_tree.element_descs.len() == 0 {
            return Err(XmlTreeError::XmlNoElementDefined());
        }
        
        let mut parser = Parser::<R>::new(buf_reader);
        let document_info = Self::parse_start_document(&mut parser)?;
        let root = element_desc_tree.root.r#ref()?;
        let xml_document = Self::parse_end_document(&mut parser, root,
            document_info);

        xml_document
    }

    /*
     * Parse the StartDocument event.
     */
    fn parse_start_document<R: Read>(parser: &mut Parser<R>) ->
        Result<DocumentInfo, XmlTreeError> {
        let mut comments_before = Vec::<String>::new();

        let document_info = loop {
            let xml_element = parser.next();

            match xml_element {
                Err(e) => return Err(XmlTreeError::XmlError(0, Box::new(e))),
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{version, encoding, standalone} => {
                            break DocumentInfo::new(version, encoding, standalone);
                        },
                        XmlEvent::EndDocument => {
                            return Err(XmlTreeError::NoEndDocument());
                        },
                        XmlEvent::Comment(cmnt) => {
                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        _ => return Err(XmlTreeError::UnexpectedXml(evt.event))
                    }
                }
            };
        };

        Ok(document_info)
    }

    /*
     * Parse until we find an EndDocument
     */
    fn parse_end_document<'b, R: Read>(parser: &'b mut Parser<R>,
        root_desc: &ElementDesc,
        document_info: DocumentInfo) ->
        Result<XmlTree, XmlTreeError> {

        let mut start_name = "".to_string();
        let mut subelements = Vec::<Element>::new();
        let mut lineno: LineNumber = 0;

        loop {
            let xml_element = parser.next();

            match xml_element {
                Err(e) => {
                    return Err(XmlTreeError::XmlError(0, Box::new(e))); // FIXME: line number
                },
                Ok(evt) => {
                    lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{..} => {
                            return Err(XmlTreeError::StartAfterStart(lineno));
                        },
                        XmlEvent::EndDocument => {
                            return Err(XmlTreeError::Unknown(0));
                        },
                        XmlEvent::StartElement{name, attributes, namespace} => {
                            start_name = name.local_name.clone();
                            let element_info = ElementInfo::new(lineno,
                                attributes, namespace);
                            match root_desc.allowable_subelements.iter().position(|x| x.name() == start_name) {
                                None => return Err(XmlTreeError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = root_desc.allowable_subelements[pos].r#ref()?;
                                    let subelement = Self::parse_subelement(0, parser,
                                        element_info, new_desc)?;
                                    Self::push_subelement(&mut subelements, 
                                        subelement);
                                    break;
                                }
                            };
                        }
                        XmlEvent::EndElement{name} => {
                            return Err(XmlTreeError::MisplacedElementEnd(lineno,
                                name.local_name));
                        },
                        XmlEvent::Comment(_cmnt) => {
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
                            continue;
                        },
/*
                        XmlEvent::ProcessingInstruction(processing_instruction) => {
println!("Skipping processing_instruction");
                            continue;
                        },
*/
                        _ => return Err(XmlTreeError::UnexpectedXml(evt.event))
                    }
                }
            }
        }

        // Get the root element
        if subelements.len() != 1 {
            return Err(XmlTreeError::OnlyOneRootElement(lineno));
        }

        Ok(XmlTree {
            document_info:  document_info,
            tree:           subelements[0],
        })
    }

    fn parse_subelement<R: Read>(depth: usize, parser: &mut Parser<R>,
        element_info: ElementInfo, desc: &ElementDesc) ->
        Result<Element, XmlTreeError> {
        let mut subelements = Vec::<Element>::new();

        loop {
            let xml_element = parser.next();

            match xml_element {
                Err(e) => {
                    return Err(XmlTreeError::XmlError(0, Box::new(e))); // FIXME: line number
                },
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{..} => {
                            return Err(XmlTreeError::StartAfterStart(lineno));
                        },
                        XmlEvent::EndDocument => {
                            return Err(XmlTreeError::Unknown(0));
                        },
                        XmlEvent::StartElement{name, attributes, namespace} => {
                            let start_name = name.local_name.clone();
                            let element_info = ElementInfo::new(lineno, attributes,
                                namespace);
                            match desc.allowable_subelements.iter().position(|x| x.name() == start_name) {
                                None => return Err(XmlTreeError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = desc.allowable_subelements[pos].r#ref()?;
                                    let subelement = Self::parse_subelement(depth + 1, parser,
                                        element_info, new_desc)?;
                                    Self::push_subelement(&mut subelements,
                                        subelement);
                                }
                            }
                            
                        }
                        XmlEvent::EndElement{name} => {
                            if name.local_name != desc.name {
                                return Err(XmlTreeError::MisplacedElementEnd(lineno,
                                    name.local_name));
                            }

                            let mut element = Element::new(name, depth, element_info);
                            element.subelements = subelements;
                            return Ok(element)
                        },
                        XmlEvent::Comment(_cmnt) => {
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
                            continue;
                        },
/*
                        XmlEvent::ProcessingInstruction(processing_instruction) => {
println!("Skipping processing_instruction");
                            continue;
                        },
*/
                        _ => {
                            return Err(XmlTreeError::UnexpectedXml(evt.event));
                        }
//                        _ => return Err(XmlTreeError::UnexpectedXml(evt.event));
                    }
                }
            }
        }
    }

    fn push_subelement(subelements: &mut Vec<Element>, element: Element) {
        subelements.push(element)
    }

    fn patch_element_desc_tree(element_desc_tree: &mut ElementDescTree) -> 
        Result<(), XmlTreeError> {

        for desc in element_desc_tree.element_descs {
            for subelement in desc.allowable_subelements.iter_mut( ) {
                let patch = match Self::find_element_desc(element_desc_tree,
                    subelement.name()) {
                    None => return Err(XmlTreeError::NoSuchElement(
                        subelement.name().to_string(), desc.name.to_string())),
                    Some(p) => p,
                };
                *subelement = ElementDescRef::Ref(patch);
            }
        }

        Ok(())
    }

    fn find_element_desc<'b>(element_desc_tree: &'b ElementDescTree, name: &'b str) ->
        Option<&'b ElementDesc<'b>> {
        for desc in element_desc_tree.element_descs {
            if desc.name == name {
                return Some(desc);
            }
        }

        return None;
    }
}

impl fmt::Display for XmlTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
println!("document:");
        write!(f, "<?xml {} {} {:?}>\n",
            self.document_info.version, self.document_info.encoding, self.document_info.standalone)?;
        write!(f, "{}", self.tree)       
    }
}
