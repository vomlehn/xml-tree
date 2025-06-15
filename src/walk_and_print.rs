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

pub fn print_walk(f: &mut fmt::Formatter<'_>, depth: usize, xml_doc: &XmlDocument) -> fmt::Result
{
    let mut indent_str = indent(depth);
    write!(f, "{}XmlDocument::new(", indent_str)?;

    indent_str = indent(depth + 1);
    let doc_info = &xml_doc.document_info;
    write!(f, "{}DocumentInfo::new(", indent_str)?;
    write!(f, "XmlVersion::{}, ", /*doc_info.version*/ "Version10")?;
    write!(f, "\"{}\".to_string(), ", doc_info.encoding)?;
    write!(f, "{}", if doc_info.standalone.is_none() { "None" } else
        {if doc_info.standalone.unwrap() {"true"} else {"false"}})?;
    write!(f, "),")?;

    let mut bl = PrintBaseLevel::new(f);
    let ed = PrintElemData::new(depth);
    walk::<PrintAccumulator, PrintBaseLevel, PrintElemData, PrintWalkData, PrintWalkResult>(&mut bl, xml_doc, &ed)?;
    write!(f, "{})", indent(depth))
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
    depth:  usize,
}

impl<'a> Accumulator<'a, PrintBaseLevel<'_, '_>, PrintElemData, PrintWalkData, PrintWalkResult>
for PrintAccumulator {
    fn new(bl: &mut PrintBaseLevel<'_, '_>, e: &Box<dyn Element>, ed: &PrintElemData) -> PrintAccumulator {
        let depth = ed.depth;
        e.display(bl.f, depth + 1)
            .expect("Unable to write Element");

        PrintAccumulator {
            depth:  depth + 1,
        }
    }

    fn add(&mut self, _wd: &PrintWalkData, _next_ed: &PrintElemData) -> PrintWalkResult {
        Ok(())
    }

    fn summary(&self, bl: &mut PrintBaseLevel<'_, '_>) -> PrintWalkResult {
        write!(bl.f, "{})", indent(self.depth + 1))?;
        write!(bl.f, "{})),", indent(self.depth))?;
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

impl ElemData<PrintAccumulator, PrintElemData> for PrintElemData {
    fn next_level(&self, acc: &PrintAccumulator,_element: &Box<dyn Element>) -> PrintElemData {
        PrintElemData::new(acc.depth + 1)
    }
}

/**
 * All we do is print, so there is no data to return. This is
 * consistent with the OK enum from fmt::Error
 */
pub type PrintWalkData = ();

pub fn indent(n: usize) -> String {
    "\n".to_owned() + &INDENT.repeat(n)
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
