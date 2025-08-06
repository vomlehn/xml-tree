/**
 * Parse XML text input and produce an XML tree
 */

use std::convert::Infallible;
use std::io::Read;
use std::marker::PhantomData;
use std::ops::{FromResidual, Try};
use xml::name::OwnedName;

use crate::parser::LineNumber;
pub use crate::xml_document_error::XmlDocumentError;
use crate::xml_document_factory::{Accumulator, DirectElement, DocumentWorking, DocumentInfo, Element, ElementInfo, LevelInfo, XmlDocument, XmlDocumentFactory, XmlDocumentFactoryImpl};
use crate::parser::Parser;

pub struct XmlTreeFactory;

impl XmlDocumentFactory for XmlTreeFactory {
    type LI = TreeLevelInfo;
    type AC = TreeAccumulator;
    type DW = XmlDocumentTree;

// FIXME: rename this
    fn xyz<'a, R: Read + 'a>(
        &self,
        reader:     R,
//        xml_schema: &'a XmlSchema<'a>,
    ) -> <Self::DW as DocumentWorking>::DocumentResult
    where
        <Self::DW as DocumentWorking>::DocumentResult: FromResidual<<<Self::AC as Accumulator>::ElementResult as Try>::Residual>,
        <Self::AC as Accumulator>::ElementResult: FromResidual<Result<Infallible, XmlDocumentError>>,
    {
        let parser = Parser::<R>::new(reader);

        let mut xml_factory_impl = XmlDocumentFactoryImpl::<R, Self::LI, Self::AC, Self::DW> {
            parser,
//            xml_schema,
            marker1: PhantomData,
            marker2: PhantomData,
            marker3: PhantomData,
        };

/*
        let name = OwnedName {
            local_name: "".to_string(),
            namespace:  None,
            prefix:     None
        };
        let element_info = ElementInfo {
            lineno:         0,
            attributes:     Vec::new(),
            namespace:      xml::namespace::Namespace(std::collections::BTreeMap::<String, String>::new()),
        };
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));
*/
        let level_info = TreeLevelInfo::new();
        xml_factory_impl.parse_document(&level_info)
    }
}

type XmlTreeResult = Box<dyn Element>;

/**
 * Information for one element in an XML tree
 */
#[derive(Debug)]
pub struct TreeLevelInfo {
/*
    depth:              usize,
*/
}

impl TreeLevelInfo {
    fn new() -> Box<TreeLevelInfo> {
        Box::new(TreeLevelInfo {
/*
            depth:              0,
*/
        })
    }
}

impl LevelInfo for TreeLevelInfo
{
/*
    type ElementValue = Box<dyn Element>;
    type ElementResult = Result<Self::ElementValue, XmlDocumentError>;
*/

    fn next(&self) -> Self {
        TreeLevelInfo {
/*
            depth:              self.depth + 1,
*/
        }
    }
}

/**
 * Information for one element in an XML tree
 * element:         A Boxed value for the Element that we're working on in parse_element().
 * open_subelement: Either None, if we don't have an unclosed Element for Some() if we do.
 */
#[derive(Debug)]
pub struct TreeAccumulator {
    element:            Box<dyn Element>,
    open_subelement:    Option<XmlTreeResult>,
}

impl Accumulator for TreeAccumulator
{
    type ElementValue = Box<dyn Element>;
    type ElementResult = Result<Self::ElementValue, XmlDocumentError>;

    fn new(name: OwnedName, element_info: ElementInfo) -> Self {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));

        TreeAccumulator {
            element,
            open_subelement:    None,
        }
    }

/*
    fn start(&self, name: OwnedName, element_info: ElementInfo) -> Self {
        let element = Box::new(DirectElement::new(name, element_info, vec!(), vec!(), vec!(), vec!()));
        TreeAccumulator {
            open_subelement:    None,
        }
    }
*/

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
