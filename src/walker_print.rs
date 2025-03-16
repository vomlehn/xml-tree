/*
 * Recursive print
 */

/*
use std::error::Error;
use std::fmt;

use crate::xml_document::{Element, XmlDocument};
use crate::walkable::{Walkable, WalkableData};

/*
type D<'a, 'b> = PrintWalkData<'a, 'b>;
type R1<'a, 'b> = Result<D<'a, 'b>, Box<dyn Error>>;
type R2 = fmt::Result;
*/

// ---------------------------

pub struct PrintWalk<'a> {
    document: &'a XmlDocument,
    f: &'a mut fmt::Formatter<'a>,
}

impl<'a> PrintWalk<'a> {
    pub fn new(document: &'a XmlDocument, f: &'a mut fmt::Formatter<'a>) -> Self {
        Self { document, f }
    }
}

impl<'a, 'b> Walkable<'a, 'b, PrintWalkData<'a, 'b>, Result<PrintWalkData<'a, 'b>, Box<dyn Error>>, fmt::Result> for PrintWalk<'a> {
    fn xml_document(&self) -> &'a XmlDocument {
        self.document
    }
}

/*
impl<'a, 'b> Walkable<'a, 'b, PrintWalkData<'a, 'b>, Result<PrintWalkData<'a, 'b>, Box<dyn Error>>, fmt::Result> for PrintWalk<'a> {
    fn xml_document(&self) -> &'a XmlDocument {
        self.document
    }
}
*/

pub struct PrintWalkData<'a, 'b> {
    depth: usize,
    f: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> PrintWalkData<'a, 'b> {
    pub fn new(depth: usize, f: &'a mut fmt::Formatter<'b>) -> Self {
        Self { depth, f }
    }
}

impl<'a, 'b> WalkableData<'a, 'b, Result<PrintWalkData<'a, 'b>, Box<dyn Error>>, (), fmt::Result> for PrintWalkData<'a, 'b> {
    fn element_start<'c>(&'c mut self, element: &Element) -> Result<PrintWalkData<'a, 'b>, Box<dyn Error>>
    where
        'a: 'c,
    {
        writeln!(self.f, "{}<{}>", "  ".repeat(self.depth), element.name)?;
        Ok(PrintWalkData {
            depth: self.depth + 1,
            f: self.f,
        })
    }

    fn element_end(&mut self, element: &Element, _: Vec<Result<(), std::fmt::Error>>) -> std::fmt::Error {
        writeln!(self.f, "{}</{}>", "  ".repeat(self.depth), element.name).map_err(|e| Box::new(e) as Box<dyn Error>)
    }


/*
    fn element_end(&mut self, element: &Element, _: Vec<fmt::Result>) -> fmt::Result {
        writeln!(self.f, "{}</{}>", "  ".repeat(self.depth), element.name)
    }
*/
}
*/
