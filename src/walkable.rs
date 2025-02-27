/*
 * XML tree walker
 */

use std::error::Error;
//use std::ops::Try;
//use thiserror;

use crate::xml_document::{Element, XmlDocument};

pub trait Walkable<'a, 'b: 'a, DATA: WalkableData<'a, 'b, RET1, RET2>, RET1: std::ops::Try<Output = DATA>,
    RET2: std::ops::FromResidual<<RET1 as std::ops::Try>::Residual> + std::ops::Try<Output = RET2>> {
    fn xml_document(&self) -> &'a XmlDocument;
        
    fn walk(&mut self, element_data: &'b mut DATA) -> RET2 {
        self.walk_n(&self.xml_document().root, element_data)
    }

    fn walk_n<'c, 'd>(&mut self, element: &'c Element, element_data: &'d mut DATA) ->
        RET2
    where
        'a: 'c,
        'b: 'd {
        let mut subelement_data = element_data.element_start(element)?;
        let mut subelements = Vec::<RET2>::new();

        for subelement in &element.subelements {
            let subdata = self.walk_n(&subelement, &mut subelement_data)?;
            subelements.push(subdata);
        } 

        subelement_data.element_end(element, subelements)
    }
}

pub trait WalkableData<'a, 'b, RET1/*: Try*/, RET2/*: Try*/> {
    fn element_start<'c>(&'c mut self, element: &Element) -> RET1
    where
        'a: 'c;

    fn element_end(&mut self, element: &Element, subelements: Vec<RET2>) ->
        RET2;
}
