/*
 * XML tree walker
 */

use thiserror;
use std::marker::PhantomData;

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

pub trait WalkerData<O> {
    fn element_start(&self, element: &Element) ->
        Result<Self, WalkerError>
        where
            Self: Sized;
    fn element_end(&self, element: &Element, subelements: Vec<O>) ->
        Result<O, WalkerError>;
}

pub trait WalkerResult {
}

pub struct Walker<'a, I: WalkerData<O>, O> {
    xml_document:   &'a XmlDocument,
    marker1:        PhantomData<I>,
    marker2:        PhantomData<O>,
}

impl<'a, I: WalkerData<O>, O> Walker<'a, I, O> {
    pub fn new(xml_document: &'a XmlDocument) -> Self {
        Walker::<I, O> {
            xml_document:   xml_document,
            marker1:        PhantomData,
            marker2:        PhantomData,
        }
    }
        
    pub fn walk(&self, element_data: &I) -> Result<O, WalkerError> {
        self.walk_n(&self.xml_document.root, element_data)
    }

    fn walk_n<'b>(&self, element: &Element, element_data: &I) ->
        Result<O, WalkerError> {
        let subelement_data = element_data.element_start(element)?;
        let mut subelements = Vec::<O>::new();

        for subelement in &element.subelements {
            let subdata = self.walk_n(&subelement, &subelement_data)?;
            subelements.push(subdata);
        } 

        element_data.element_end(element, subelements)
    }
}
