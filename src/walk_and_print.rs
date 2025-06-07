/**
 * Basic structure for recursive printing
 */

//use std::cell::RefCell;
use std::fmt;
//use std::ops::{FromResidual, Try};

use crate::xml_document::{Element, XmlDocument};
use crate::walkable::{Accumulator/*, BaseLevel*/, ElemData/*, Walkable*/};
use crate::walkable::walk;

const INDENT: &str = "    ";

/*
pub struct XmlPrint<'a> {
    f:  &'a mut fmt::Formatter<'fmt>,
//    xml_doc:    &'a XmlDocument,
}

impl<'a> XmlPrint<'a> {
//    pub fn new(f: &'a mut fmt::Formatter<'fmt>, xml_doc: &'a XmlDocument) -> Self {
    pub fn new(f: &'a mut fmt::Formatter<'fmt>) -> Self {
        XmlPrint {
            f:          f,
//            xml_doc:    xml_doc,
        }
    }

    pub fn walk(&mut self, xml_doc: &'a XmlDocument) -> fmt::Result {
        let print_base_level = PrintBaseLevel::new(self.f);
        let print_walkable = PrintWalkable::new(print_base_level, &xml_doc);
        let print_elem_data = PrintElemData::new(0);
        print_walkable.walk_down(&xml_doc.root, &print_elem_data)
    }
}
*/

/*
pub struct WalkAndPrint<'a> {
    xml_doc:    &'a XmlDocument,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new(xml_doc: &'a XmlDocument) -> WalkAndPrint<'a> {
        WalkAndPrint {
            xml_doc:    xml_doc,
        }
    }
}
*/

pub fn print_walk(f: &mut fmt::Formatter<'_>, xml_doc: &XmlDocument) -> fmt::Result
{
    let depth = 2;
    write!(f, "{}XmlDocument::new(", indent(depth))?;

    let doc_info = &xml_doc.document_info;
    // FIXME: use indent()
    write!(f, "\n")?;
    write!(f, "            DocumentInfo::new(")?;
    write!(f, "XmlVersion::{}, ", doc_info.version)?;
    write!(f, "\"{}\", ", doc_info.encoding)?;
    write!(f, "{}", if doc_info.standalone.is_none() { "None" } else
        {if doc_info.standalone.unwrap() {"true"} else {"false"}})?;
    write!(f, "),\n")?;

    let mut bl = PrintBaseLevel::new(f);
    let ed = PrintElemData::new(0);
    walk::<PrintAccumulator, PrintBaseLevel, PrintElemData, PrintWalkData, PrintWalkResult>(&mut bl, xml_doc, &ed)?;
    write!(f, "{})\n", indent(depth))
}

/*
struct PrintWalkable<'a> {
    xml_doc:    &'a XmlDocument,
    base:       RefCell<PrintBaseLevel<'a>>,
}
*/

/*
impl<'a> PrintWalkable<'a> {
    pub fn new(base: PrintBaseLevel<'a>, xml_doc: &'a XmlDocument) -> PrintWalkable<'a> {
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

/*
pub struct PrintWalkableData<'a, 'b> {
//        xml_doc:    xml_doc,
        base:       RefCell<PrintBaseLevel<'a, 'b>>,
}

impl<'a, 'b> PrintWalkableData<'a, 'b> {
    pub fn new(base: PrintBaseLevel<'a, 'b>) -> PrintWalkableData<'a, 'b> {
        PrintWalkableData {
            base:   RefCell::new(base),
        }
    }
}
*/

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
    depth:  usize,
}

impl<'a> Accumulator<'a, PrintBaseLevel<'_, '_>, PrintElemData, PrintWalkData, PrintWalkResult>
for PrintAccumulator {
    fn new(bl: &mut PrintBaseLevel<'_, '_>, e: &Box<dyn Element>, ed: &PrintElemData) -> Self {
        // FIXME: use symbolic values for indentation
        let depth = ed.depth + 3;
//write!(bl.f, "depth {}:", depth);
        write!(bl.f, "{}Box::new(", indent(depth))
            .expect("Unable to write Box::new");
        write!(bl.f, "\"{}\"\n", e.name())
            .expect("Unable to write {}");
        PrintAccumulator {
            depth:  depth,
        }
    }

    fn add(&mut self, _wd: &PrintWalkData) -> PrintWalkResult {
        Ok(())
    }

    fn summary(&self, bl: &mut PrintBaseLevel<'_, '_>) -> PrintWalkResult {
        write!(bl.f, "{})\n", indent(self.depth))?;
        Ok(())
    }
}

/**
 * The BaseLevel data consists of just an fmt::Formatter passed to
 * fmt::Display::fmt().
 */
pub struct PrintBaseLevel<'a, 'b> {
    pub f: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> PrintBaseLevel<'a, 'b> {
    pub fn new(f: &'a mut fmt::Formatter<'b>) -> Self {
        PrintBaseLevel {
            f:  f,
        }
    }
}

/*
impl<'a, 'b> BaseLevel for PrintBaseLevel<'a, 'b> {}
*/

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

impl ElemData<PrintElemData> for PrintElemData {
    fn next_level(&self, _element: &Box<dyn Element>) -> PrintElemData {
        PrintElemData::new(self.depth + 1)
    }
}

/**
 * All we do is print, so there is no data to return. This is
 * consistent with the OK enum from fmt::Error
 */
pub type PrintWalkData = ();

pub fn indent(n: usize) -> String {
    INDENT.repeat(n)
}

#[cfg(test)]
mod print_tests {
/*
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;
*/

    use crate::xml_document::{create_test_doc, Element, XmlDocument};
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
}
