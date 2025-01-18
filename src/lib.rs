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

pub struct ElementDesc {
    pub name:                   &'static str, 
    pub allowable_subelements:  &'static[ElementDesc]
}

impl ElementDesc {
    pub fn position(&self, target: &String) -> Option<usize> {
        let mut pos: usize = 0;

        for element in self.allowable_subelements {
            if element.name == target {
                return Some(pos);
            }
            pos += 1;
        }

        return None
    }

    fn fmt_no_circular(&self, f: &mut fmt::Formatter<'_>, active: &mut Vec<&String>) -> fmt::Result {
        let mut sep_subelem = "";

        write!(f, "{}:\n", self.name)?;
        write!(f, "   [")?;

        for element in self.allowable_subelements {
            for name in &mut *active {
                if *name == element.name {
                    eprintln!("Circular dependency starting at {}", name);
                    std::process::exit(1);
                }
            }

            write!(f, "{}{}", sep_subelem, element.name)?;
            sep_subelem = ", ";
        }

        write!(f, "]\n");
       
        for element in self.allowable_subelements {
            write!(f, "{}", element)?;
        }

        Ok(())
    }
}

impl fmt::Display for ElementDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut active = Vec::<&String>::new();
        self.fmt_no_circular(f, &mut active)
    }
}

impl fmt::Debug for ElementDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.allowable_subelements)
    }
}

/*
 * Define the structure used to construct the tree for the parsed document.
 */
#[derive(Clone, Debug)]
pub struct Element {
    depth:                  usize,
    lineno:                 LineNumber,
    pub name:               OwnedName,
    attributes:             Vec<OwnedAttribute>,
    namespace:              Namespace,
    pub subelements:        Vec<Element>,
    before_comments:        Vec<String>,
    after_comments:         Vec<String>,
}

impl Element {
    fn new(depth: usize, lineno: LineNumber, name: OwnedName, attributes: Vec<OwnedAttribute>,
        namespace: Namespace) -> Element {
        Element {
            depth:              depth,
            lineno:             lineno,
            name:               name,
            attributes:         attributes,
            namespace:          namespace,
            subelements:        Vec::<Element>::new(),
            before_comments:    Vec::<String>::new(),
            after_comments:     Vec::<String>::new(),
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        for attribute in &self.attributes {
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
        for attribute in self.attributes.clone() {
            write!(f, " {}={}", attribute.name.local_name, attribute.value)?;
        }

        if self.subelements.len() == 0 {
            write!(f, " /> (line {})\n", self.lineno)?;
        } else {
            write!(f, "> (line {})\n", self.lineno)?;

            for element in &self.subelements {
                element.fmt(f)?;
            }

            write!(f, "{}</{}>\n", indent_string, self.name.local_name)?;
        }


        Ok(())
    }
}


/* FIXME: remove this
const XXX: ElementDesc = ElementDesc {
    name:                   "XXX",
    allowable_subelements:  &[XXX],
};
*/

#[derive(Debug)]
pub struct XmlTree {
    version:        XmlVersion,
    encoding:       String,
    standalone:     Option<bool>,
    pub root:       Element,
}

impl XmlTree {
    pub fn new(path: String, root: &ElementDesc) ->
        Result<XmlTree, XmlTreeError> {
        let file = match File::open(path) {
            Err(e) => return Err(XmlTreeError::XmlError(0, Box::new(e))),
            Ok(f) => f,
        };
        let buf_reader = BufReader::new(file);
        Self::new_from_reader(buf_reader, root)
    }

    pub fn new_from_reader<R: Read>(buf_reader: BufReader<R>, root: &ElementDesc) -> Result<XmlTree, XmlTreeError> {
        let mut parser = Parser::<R>::new(buf_reader);
        let (lineno, version, encoding, standalone) =
            Self::parse_start_document(&mut parser)?;
        let xml_document = Self::parse_end_document(&mut parser, root,
            (lineno, version, encoding, standalone));

        xml_document
    }

    /*
     * Parse the StartDocument event.
     */
    fn parse_start_document<R: Read>(parser: &mut Parser<R>) ->
        Result<(LineNumber, XmlVersion, String, Option<bool>), XmlTreeError> {
        let mut comments_before = Vec::<String>::new();

        let (lineno, version, encoding, standalone) = loop {
            let xml_element = parser.next();

            match xml_element {
                Err(_) => return Err(XmlTreeError::NoXTCE()),
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{version, encoding, standalone} => {
                            break (lineno, version, encoding, standalone)
                        },
                        XmlEvent::EndDocument => {
                            return Err(XmlTreeError::NoXTCE());
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

        Ok((lineno, version, encoding, standalone))
    }

    /*
     * Parse until we find an EndDocument
     */
    fn parse_end_document<R: Read>(parser: &mut Parser<R>, desc: &ElementDesc,
        info: (LineNumber, XmlVersion, String, Option<bool>)) ->
        Result<XmlTree, XmlTreeError> {

        let mut start_name = "".to_string();
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
                            start_name = name.local_name.clone();
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XmlTreeError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(0, parser,
                                        attributes, namespace, &new_desc)?;
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
            return Err(XmlTreeError::OnlyOneRootElement(info.0));
        }

        let root = &subelements[0];

        Ok(XmlTree {
            version:    info.1,
            encoding:   info.2,
            standalone: info.3,
            root:       root.clone(),
        })
    }

    fn parse_subelement<R: Read>(depth: usize, parser: &mut Parser<R>,
        attributes: Vec<OwnedAttribute>, namespace: Namespace,
        desc: &ElementDesc) ->
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
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XmlTreeError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(depth + 1, parser,
                                        attributes, namespace,
                                        &new_desc)?;
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

                            let mut element = Element::new(depth, lineno, name, attributes, namespace);
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

/*
    pub fn dump(&self) {
        println!("<?xml {} {} {:?}>",
            self.version, self.encoding, self.standalone);
        self.root.dump();
    }
*/
}

impl fmt::Display for XmlTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
println!("document:");
        write!(f, "<?xml {} {} {:?}>\n",
            self.version, self.encoding, self.standalone)?;
        write!(f, "{}", self.root)       
    }
}
