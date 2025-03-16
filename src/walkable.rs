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
//use std::fmt;

/*
 * This has to be a trait so the functions can be defined by users
 */
pub trait ElementData: Clone {
    fn element_start(&mut self, element: &Element) ->
        Result<impl ElementData, Box<dyn Error>>;
    fn element_end(&mut self, element: &Element, subelements: Vec<Box<dyn WalkData>>) -> Result<impl WalkData, Box<dyn Error>>;
}

pub trait WalkData {}

pub trait Walkable {
    fn xml_document(&self) -> &XmlDocument;

    fn walk<'a, D>(&'a mut self, element_data: &'a mut D) ->
        Result<impl WalkData + 'a, Box<dyn Error + 'a>>
    where
        D: ElementData + Drop + Copy,
    {
        let root = &self.xml_document().root;
        self.walk_i(root, element_data)
    }

    fn walk_i<'a, D>(&self, element: &'a Element, element_data: &'a mut D) ->
        Result<impl WalkData + 'a, Box<dyn Error + 'a>>
    where
        D: ElementData + Clone + Copy,
    {
        let mut subelements = Vec::<Box<dyn WalkData + 'a>>::new();
        let mut d = Vec::new();

        for subelement in &element.subelements {
            let mut element_subdata = element_data.clone();
            // Pass the same mutable reference to avoid overlapping borrows
            let subdata = self.walk_i(subelement, &mut element_subdata)?;
            subelements.push(Box::new(subdata));
            d.push(element_data.clone());
        }

        element_data.element_end(element, subelements)
    }

/*
    fn walk_i<'a, D>(&'a self, element: &'a Element, element_data: &'a mut D) ->
    Result<impl WalkData + 'a, Box<dyn Error + 'a>>
    where
        D: ElementData,
    {
        let mut subelements = Vec::<Box<dyn WalkData + 'a>>::new();
        {
            let subelement_data = element_data.element_start(element)?;

            for subelement in &element.subelements {
                let subdata = self.walk_i(subelement, &mut subelement_data)?;
                subelements.push(Box::new(subdata));
            }
            subelement_data
        };

        element_data.element_end(element, subelements)
    }
*/
}

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
