/*
 * XML tree walker
 */

use std::error::Error;
use thiserror;

use crate::xml_document::{Element, XmlDocument};
use crate::xml_document_error::XmlDocumentError;

#[derive(Debug, thiserror::Error)]
    pub enum WalkerError {
    // Need full path to faulty element
    #[error("Unknown element \"{0}\"")]
    UnknownElement(String),

    #[error("XMLDocumentError: \"{0}\"")]
    XmlTreeError(XmlDocumentError),
}

pub trait Walker<'a, 'b, I: WalkerData<'a, 'b, O>, O> {
    fn xml_document(&self) -> &'a XmlDocument;
        
    fn walk(&mut self, element_data: &'b mut I) -> Result<O, Box<dyn Error>> {
        self.walk_n(&self.xml_document().root, element_data)
    }

    fn walk_n<'c>(&mut self, element: &'c Element, element_data: &'b mut I) ->
        Result<O, Box<dyn Error>> {
/*
        let mut subelement_data = element_data.element_start(element)?;
*/
        let subelements = Vec::<O>::new();
/*

        for subelement in &element.subelements {
            let subdata = self.walk_n(&subelement, &mut subelement_data)?;
            subelements.push(subdata);
        } 

*/
        element_data.element_end(element, subelements)
    }
}

pub trait WalkerData<'a, 'b, O> {
    fn element_start<'c: 'a>(&'c mut self, element: &Element) ->
        Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn element_end(&mut self, element: &Element, subelements: Vec<O>) ->
        Result<O, Box<dyn Error>>;
}

pub trait WalkerResult {
}
