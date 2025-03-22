/*
 * XML tree walker
 */

use crate::xml_document::{Element, XmlDocument};

/*
 * Trait for walking an XML document
 * 'a       Lifetime for trait
 * 'b       
 * DATA1    Trait for data passed to walk() and returned as data from
 *          element_start.
 * RET1     Trait for Try value from element_start()
 * DATA2    Trait for data passed from element_end() 
 * RET2     Trait for Try value from element_end() and walk_n()
 */
// -----------------------------------------
use std::error::Error;
use std::fmt;
use std::ops::Try;

struct A<'a> {
    xml_document:   &'a XmlDocument,
}

impl<'a> Walkable for A<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

impl ElementData for A<'_> {
    type WD = u8;
    type WS = Result<Self::WD, Box<dyn Error>>;
    fn xml_document(&self) -> &XmlDocument { self.xml_document }
    fn element_end(&mut self) -> Self::WS {
        Ok(37)
    }
}

struct B<'a> {
    xml_document:   &'a XmlDocument,
}

impl<'a> Walkable for B<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

impl ElementData for B<'_> {
    type WD = ();
    type WS = fmt::Result;
    fn xml_document(&self) -> &XmlDocument { self.xml_document }
    fn element_end(&mut self) -> Self::WS {
        Ok(())
    }
}

/*
 * This has to be a trait so the functions can be defined by users
 */
pub trait ElementData:  {
    type WD;
    type WS: Try;

    fn xml_document(&self) -> &XmlDocument;
    fn element_end(&mut self) -> Self::WS;
/*
    fn element_start(&mut self, element: &Element) ->
        Result<Box<dyn E>, Box<dyn Error>>;
    fn element_end(&mut self, element: &Element,
        subelements: Vec<Box<dyn W>>) ->
        Result<Box<dyn W>, Box<dyn Error>>;
*/
}

pub trait Walkable {
    fn xml_document(&self) -> &XmlDocument;
}

/*
pub trait WalkData {}

pub trait Walkable {
    fn xml_document(&self) -> &XmlDocument;

    fn walk<'a, ED: ElementData>(&'a mut self, element_data: &'a mut ED) ->
        Result<dyn WalkData + 'a, Box<dyn Error + 'a>>
    {
        let root = &self.xml_document().root;
        self.walk_i(root, element_data)
    }

    fn walk_i<'a, ED: ElementData>(&self, element: &'a Element,
        element_data: &'a mut ED) ->
        Result<dyn WalkData + 'a, Box<dyn Error + 'a>>
    {
        let mut subelements = Vec::new();
        let mut d = Vec::new();

        for subelement in &element.subelements {
            let mut element_subdata = element_data.clone();
            // Pass the same mutable reference to avoid overlapping borrows
            let subdata = self.walk_i(subelement, &mut element_subdata)?;
            let s = subdata;
            subelements.push(Box::new(s));
            d.push(element_data.clone());
        }

        let e = element.clone();
        element_data.element_end(&e, subelements)
    }
}
*/

/*
pub struct PrintWalk<'a> {
    pub document: &'a XmlDocument,
    pub f: &'a mut fmt::Formatter<'a>,
}

impl<'a> Walkable for PrintWalk<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.document
    }
}

pub struct PrintWalkData<'a, 'b> {
    depth: usize,
    f: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> WalkData for () {}

impl<'a, 'b> ElementData for PrintWalkData<'a, 'b> {
    type StartStatus = Result<Self, Box<dyn Error>>;
    type EndStatus = Result<(), Box<dyn Error>>;

    fn element_start(&mut self, element: &Element) -> Self::StartStatus {
        writeln!(self.f, "{}<{}>", "  ".repeat(self.depth), element.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>);

        Ok(PrintWalkData {
            depth: self.depth + 1,
            f: self.f,
        })
    }

    fn element_end(&mut self, element: &Element, _: Vec<()>) -> Self::EndStatus {
        writeln!(self.f, "{}<\{}>", "  ".repeat(self.depth), element.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
*/
