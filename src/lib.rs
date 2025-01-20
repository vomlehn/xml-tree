use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
//use std::ptr::addr_of_mut;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

pub mod parser;
pub mod xml_tree_error;

pub use crate::parser::{LineNumber, Parser};
pub use crate::xml_tree_error::XmlTreeError;

// FIXME: for testing only, remove me
static mut test_xml_desc_tree: XmlDescTree = XmlDescTree {
    root:           "a1",
    xml_descs:   &mut [
        XmlDesc {
            name:   "a1",
            allowable_subelements: &mut [XmlDescRef::Name("a2")],
        },
        XmlDesc {
            name:   "a2",
            allowable_subelements: &mut [XmlDescRef::Name("a1")],
        }
    ]
};

/*
 * Define the data structures used to describe the XML used for parsing.
 */
#[derive(Debug)]
pub struct XmlDescTree<'a> {
    pub root:       &'a str,
    pub xml_descs:  &'a mut [XmlDesc<'a>],
}

impl<'a> XmlDescTree<'a> {
    fn patch_xml_desc_tree(&mut self) -> Result<(), XmlTreeError> {
        let mut patches = Vec::<(&mut XmlDesc, &mut XmlDesc)>::new();

        for desc in self.xml_descs.iter_mut() {
            for subelement in desc.allowable_subelements.iter_mut() {
                let name = subelement.name();

                let patch = match Self::find_xml_desc(self.xml_descs, name) {
                    None => return Err(XmlTreeError::NoSuchElement(
                        name.to_string(), desc.name.to_string())),
                    Some(p) => p,
                };

                let subelement_ref = match subelement {
                    XmlDescRef::Name(n) => return Err(XmlTreeError::UnresolvedRef(n.to_string())),
                    XmlDescRef::Ref(r) => r,
                };

                patches.push((subelement_ref, patch));
            }
        }

        for (subelement_ref, patch) in patches {
            subelement_ref = patch;
        }

        Ok(())
    }

    // Find an XmlDesc with the given name
    fn find_xml_desc<'b>(xml_descs: &'b mut [XmlDesc<'a>], name: &str) -> Option<&'b mut XmlDesc<'a>> {
        xml_descs.iter_mut().find(|desc| desc.name == name)
    }
}

pub struct XmlDesc<'a> {
    name:                   &'a str,
    allowable_subelements:  &'a mut [XmlDescRef<'a>],
}

impl<'a> XmlDesc<'a> {
    pub fn position(&self, target: &String) -> Option<usize> {
        let mut pos: usize = 0;

        for element in self.allowable_subelements.iter() {
            let name = match element {
                // FIXME: this is an internal consistency failure
                XmlDescRef::Name(name) => *name,
                XmlDescRef::Ref(r) => r.name,
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

        for element in self.allowable_subelements.iter() {
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
       
        for element in self.allowable_subelements.iter() {
            let element_name = element.name();
            write!(f, "{:?}", element_name)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for XmlDesc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut active = Vec::<&String>::new();
        self.fmt_no_circular(f, &mut active)
    }
}

impl<'a> fmt::Debug for XmlDesc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.allowable_subelements)
    }
}

#[derive(Clone, Debug)]
enum XmlDescRef<'a> {
    Name(&'a str),
    Ref(&'a XmlDesc<'a>),
}

impl<'a> XmlDescRef<'a> {
    fn name(&self) -> &str {
        match self {
            XmlDescRef::Name(n) => *n,
            XmlDescRef::Ref(r) => r.name,
        }
    }

    fn r#ref(&self) -> Result<&'a XmlDesc, XmlTreeError> {
        match self {
            XmlDescRef::Name(n) => Err(XmlTreeError::UnresolvedRef(n.to_string())),
            XmlDescRef::Ref(r) => Ok(r),
        }
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
pub struct XmlTree<'a> {
    document_info:  DocumentInfo,
    root:           Option<&'a Element>,
    tree:           Vec<Element>,
}

impl XmlTree<'_> {
    pub fn new<'a>(path: String, root: &'a mut XmlDescTree<'a>) ->
        Result<XmlTree<'a>, XmlTreeError> {
        let file = match File::open(path) {
            Err(e) => return Err(XmlTreeError::XmlError(0, Box::new(e))),
            Ok(f) => f,
        };
        let buf_reader = BufReader::new(file);
        Self::new_from_reader(buf_reader, root)
    }

    pub fn new_from_reader<'a, R: Read + 'a>(buf_reader: BufReader<R>,
        xml_desc_tree: &'a mut XmlDescTree<'a>)
->
        Result<XmlTree<'a>, XmlTreeError> {

        XmlDescTree::patch_xml_desc_tree(xml_desc_tree);
        return Ok(XmlTree {
            document_info:  DocumentInfo {
                version:    XmlVersion::Version10,
                encoding:   "tbd".to_string(),
                standalone: None,
            },
            root:   None,
            tree:   Vec::<Element>::new(),
        });

/*
        if xml_desc_tree.xml_descs.len() == 0 {
            return Err(XmlTreeError::XmlNoElementDefined());
        }
        
        let mut parser = Parser::<R>::new(buf_reader);
        let document_info = Self::parse_start_document(&mut parser)?;
        let xml_document = Self::parse_end_document(&mut parser,
            document_info, xml_desc_tree);

        xml_document
*/
    }

/*
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
        document_info: DocumentInfo, xml_desc_tree: &XmlDescTree) ->
        Result<XmlTree<'b>, XmlTreeError> {

        let mut xml_tree = XmlTree {
            document_info:  document_info,
            root:           None,
            tree:           Vec::<Element>::new(),
        };

        let mut start_name = "".to_string();

        loop {
            let xml_element = parser.next();

            match xml_element {
                Err(e) => {
                    return Err(XmlTreeError::XmlError(0, Box::new(e))); // FIXME: line number
                },
                Ok(evt) => {
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

                            match xml_desc_tree.xml_descs.iter().position(|x| x.name == start_name) {
                                None => return Err(XmlTreeError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &xml_desc_tree.xml_descs[pos];
                                    let subelement = Self::parse_subelement(0, parser,
                                        element_info, new_desc)?;
                                    xml_tree.tree.push(subelement);
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
        if xml_tree.tree.len() != 1 {
            return Err(XmlTreeError::OnlyOneRootElement(lineno));
        }

        let root_name = xml_desc_tree.root;

        xml_tree.root = match Self::find_element(&xml_tree, &root_name) {
            None => return Err(XmlTreeError::RootNotFound(root_name.to_string())),
            Some(r) => Some(r),
        };

        return Ok(xml_tree);
    }

    // Find an element with the given name in the XmlTree
    fn find_element<'a>(xml_tree: &'a XmlTree,  name: &str) -> Option<&'a Element> {
        for element in xml_tree.tree.iter() {
            if name == element.name.local_name {
                return Some(&element);
            }
        }

        return None;
    }

    fn parse_subelement<R: Read>(depth: usize, parser: &mut Parser<R>,
        element_info: ElementInfo, desc: &XmlDesc) ->
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
                                    subelements.push(subelement);
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
*/
}

impl fmt::Display for XmlTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
println!("document:");
        write!(f, "<?xml {} {} {:?}>\n",
            self.document_info.version, self.document_info.encoding, self.document_info.standalone)?;
        write!(f, "{:?}", self.tree)       
    }
}
