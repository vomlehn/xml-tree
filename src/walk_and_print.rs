/**
 * Basic structure for recursive printing
 */

use std::cell::RefCell;
use std::fmt;
use std::ops::{FromResidual, Try};

use crate::xml_document::{Element, XmlDocument};
use crate::walkable::{Accumulator, BaseLevel, ElemData, Walkable};

const INDENT: &str = "    ";

/*
pub struct XmlPrint<'a> {
    f:  &'a mut fmt::Formatter<'fmt>,
//    xml_doc:    &'a XmlDocument<'a>,
}

impl<'a> XmlPrint<'a> {
//    pub fn new(f: &'a mut fmt::Formatter<'fmt>, xml_doc: &'a XmlDocument<'a>) -> Self {
    pub fn new(f: &'a mut fmt::Formatter<'fmt>) -> Self {
        XmlPrint {
            f:          f,
//            xml_doc:    xml_doc,
        }
    }

    pub fn walk(&mut self, xml_doc: &'a XmlDocument<'a>) -> fmt::Result {
        let print_base_level = PrintBaseLevel::new(self.f);
        let print_walkable = PrintWalkable::new(print_base_level, &xml_doc);
        let print_elem_data = PrintElemData::new(0);
        print_walkable.walk_down(&xml_doc.root, &print_elem_data)
    }
}
*/

pub struct WalkAndPrint<'a> {
    xml_doc:    &'a XmlDocument<'a>,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new(xml_doc: &'a XmlDocument<'a>) -> WalkAndPrint<'a> {
        WalkAndPrint {
            xml_doc:    xml_doc,
        }
    }
}

/*
impl<'a> fmt::Display for WalkAndPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut print_walk = XmlPrint::new(f);
        print_walk.walk(self.xml_doc)
    }
}
*/

pub trait PrintWalkable<'a, AC, BL, ED, WD, WR>: Walkable<'a, AC, BL, ED, WD, WR>
where
    AC: Accumulator<'a, BL, ED, WD, WR>,
    BL: 'a + BaseLevel,
    ED: ElemData<BL, ED>,
    WR: Try<Output = WD>,
    WR: FromResidual,
{
}

/*
struct PrintWalkable<'a> {
    xml_doc:    &'a XmlDocument<'a>,
    base:       RefCell<PrintBaseLevel<'a>>,
}
*/

/*
impl<'a> PrintWalkable<'a> {
    pub fn new(base: PrintBaseLevel<'a>, xml_doc: &'a XmlDocument<'a>) -> PrintWalkable<'a> {
        PrintWalkable {
            xml_doc:    xml_doc,
            base:       RefCell::new(base),
        }
    }
}

impl<'a> Walkable<'a, PrintAccumulator, PrintBaseLevel<'a>, PrintElemData, PrintWalkData, PrintWalkResult> for PrintWalkable<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_doc
    }

    fn base_level_cell(&self) -> &RefCell<PrintBaseLevel<'a>>
    {
            &self.base
    }
}
*/

pub struct PrintWalkableData<'a> {
//        xml_doc:    xml_doc,
        base:       RefCell<PrintBaseLevel<'a>>,
}

impl<'a> PrintWalkableData<'a> {
    pub fn new(base: PrintBaseLevel<'a>) -> PrintWalkableData<'a> {
        PrintWalkableData {
            base:   RefCell::new(base),
        }
    }
}

/**
 * Since we're printing, our return type is the same as the type
 * returned from std::fmt::Display::fmt(). This means we have to
 * returns one of the fmt::Error types if we encounter another
 * error, or simply panic!.
 */
pub type PrintWalkResult = fmt::Result;

/**
 * We don't return any data, but do print the element name each
 * time we enter PrintWalkable::walk_down().
 */
pub struct PrintAccumulator {
}

impl<'a> Accumulator<'a, PrintBaseLevel<'a>, PrintElemData, PrintWalkData, PrintWalkResult>
for PrintAccumulator {
    fn new(bl: &mut PrintBaseLevel<'a>, e: &Box<dyn Element>, ed: &PrintElemData) -> Self {
        write!(bl.f, "{}{}\n", indent(ed.depth), e.name())
            .expect("Unable to write result");
        PrintAccumulator {}
    }

    fn add(&mut self, _wd: &PrintWalkData) -> PrintWalkResult {
        Ok(())
    }

    fn summary(&self) -> PrintWalkResult {
        Ok(())
    }
}

/**
 * The BaseLevel data consists of just an fmt::Formatter passed to
 * fmt::Display::fmt().
 */
pub struct PrintBaseLevel<'a> {
    pub f: &'a mut fmt::Formatter<'a>,
}

impl<'a> PrintBaseLevel<'a> {
    pub fn new(f: &'a mut fmt::Formatter<'a>) -> Self {
        PrintBaseLevel {
            f:  f,
        }
    }
}

impl<'a> BaseLevel for PrintBaseLevel<'a> {}

/**
 * Keep track of the depth so we can do proper indentation
 */
pub struct PrintElemData {
    depth:  usize,
}

impl PrintElemData {
    pub fn new(depth: usize) -> PrintElemData {
        PrintElemData {
            depth:  depth,
        }
    }
}

impl ElemData<PrintBaseLevel<'_>, PrintElemData> for PrintElemData {
    fn next_level(&self, _element: &Box<dyn Element>) -> PrintElemData {
        PrintElemData::new(self.depth + 1)
    }
}

/**
 * All we do is print, so there is no data to return. This is
 * consistent with the OK enum from fmt::Error
 */
pub type PrintWalkData = ();

fn indent(n: usize) -> String {
    INDENT.repeat(n)
}

#[cfg(test)]
mod print_tests {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};

    use super::WalkAndPrint;

    #[test]
    fn test_fmt_result() {
        println!();
        println!("Try with a fmt::Result");
        println!("----------------------");
        let xml_document = create_test_doc();
        let po = WalkAndPrint::new(&xml_document);
        println!("Display WalkAndPrint:");
        println!("{}", po);
    }

    /**
     * Manually create an XmlDocument.
     */
     // FIXME: This should be moved to a common area
    fn create_test_doc() -> XmlDocument {
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno: 1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace: ns,
        };

        XmlDocument {
            root:           branch("n1", &ei, vec![
                                leaf("n2", &ei),
                                branch("n3", &ei, vec![
                                    leaf("n4", &ei)])
                            ]),
            document_info:  DocumentInfo {
                                version: XmlVersion::Version10,
                                encoding: "xxx".to_string(),
                                standalone: None,
                            },
        }
    }

    fn leaf(name: &str, ei: &ElementInfo) -> Element {
        node(name, ei, Vec::<Element>::new())
    }

    fn branch(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        node(name, ei, subelements)
    }

    fn node(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        Element {
            name: OwnedName {
                local_name: name.to_string(),
                namespace: None,
                prefix: None,
            },
            element_info: (*ei).clone(),
            subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
