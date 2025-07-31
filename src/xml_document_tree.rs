/**
 * Parse XML text input and produce an XML tree
 */

use std::convert::Infallible;
use std::io::Read;
use std::marker::PhantomData;
use std::ops::{FromResidual, Try};
use xml::name::OwnedName;

use crate::parser::LineNumber;
pub use crate::xml_document::{DirectElement, Element, ElementInfo, XmlDocument};
pub use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::{DocumentWorking, DocumentInfo, ElementWorking, XmlDocumentFactory, XmlDocumentFactoryImpl};
//use crate::xml_schema::XmlSchema;
use crate::parser::Parser;

pub struct XmlTreeFactory;

impl XmlDocumentFactory for XmlTreeFactory {
    type EW = XmlTreeElement;
    type DW = XmlDocumentTree;

    fn xyz<'a, R: Read + 'a>(
        &self,
        reader: R,
//        xml_schema: &'a XmlSchema<'a>,
    ) -> <Self::DW as DocumentWorking>::DocumentResult
    where
        <Self::DW as DocumentWorking>::DocumentResult: FromResidual<<<Self::EW as ElementWorking>::ElementResult as Try>::Residual>,
        <Self::EW as ElementWorking>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>,
    {
        let parser = Parser::<R>::new(reader);

        let mut xml_factory_impl = XmlDocumentFactoryImpl::<R, Self::EW, Self::DW> {
            parser,
//            xml_schema,
            marker1: PhantomData,
            marker2: PhantomData,
        };

        xml_factory_impl.parse_document()
    }
}

type XmlTreeResult = Box<dyn Element>;

/**
 * Information for one element in an XML tree
 */
#[derive(Debug)]
pub struct XmlTreeElement {
    element:            XmlTreeResult,
    open_subelement:    Option<XmlTreeResult>,
}

impl ElementWorking for XmlTreeElement
{
    type ElementValue = Box<dyn Element>;
    type ElementResult = Result<Self::ElementValue, XmlDocumentError>;

    fn start(name: OwnedName, element_info: ElementInfo) -> Self {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));
        XmlTreeElement {
            element:            element,
            open_subelement:    None,
        }
    }

    fn end(&self) -> Self::ElementResult {
        Ok(self.element.clone())
    }

    fn in_element(&self) -> bool {
        self.open_subelement.is_some()
    }

    fn start_subelement(&mut self, subelement: Box<dyn Element>) {
        self.open_subelement = Some(subelement);
    }

    fn end_subelement(&mut self) {
        let open_subelement = self.open_subelement().unwrap();
        self.element.subelements_mut().push(open_subelement);
        self.open_subelement = None;
    }

    fn open_subelement(&self) -> Option<Self::ElementValue> {
        self.open_subelement.clone()
    }

    fn name(&self) -> &str {
        self.element.name()
    }

    fn lineno(&self) -> LineNumber {
        self.element.lineno()
    }
}

pub struct XmlDocumentTree {
    document_info:  DocumentInfo,
}

impl DocumentWorking for XmlDocumentTree {
    type DocumentValue = XmlDocument;
    type DocumentResult = Result<Self::DocumentValue, XmlDocumentError>;

    fn start(document_info: DocumentInfo) -> Self {
        XmlDocumentTree {
            document_info:  document_info,
        }
    }

    fn end(&self, top_element: Vec<Box<dyn Element>>) -> Self::DocumentResult {
        Ok(XmlDocument::new(self.document_info.clone(), top_element))
    }
}
