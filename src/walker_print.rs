/*
 * Recursive print
 */

use crate::{Accumulator, ElemData, WalkData, WalkError, WalkResult, Walkable};
use crate::xml_document::{Element, XmlDocument};

const INDENT: &str = "    ";

struct WalkablePrint<'a> {
    xml_document: &'a XmlDocument,
}

impl WalkablePrint<'_> {
    pub fn new<'a>(xml_document: &'a XmlDocument) -> WalkablePrint<'a> {
        WalkablePrint {
            xml_document:   xml_document,
        }
    }
}

impl Walkable<ElemDataPrint, WalkDataPrint, AccumulatorPrint> for WalkablePrint<'_> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

// ----------------- Data Types ----------------

pub struct WalkDataPrint {
    pub data: String,
}

impl WalkDataPrint {
    pub fn new() -> WalkDataPrint {
        WalkDataPrint {
            data:   "".to_string(),
        }
    }
}

impl WalkData for WalkDataPrint {}

#[derive(Debug)]
pub struct ElemDataPrint {
    pub depth: usize,
}

impl ElemData<ElemDataPrint> for ElemDataPrint {
    fn start(&self, element: &Element) -> WalkResult<ElemDataPrint, WalkError> {
        println!("{}<{}>", INDENT.repeat(self.depth), element.name);
        let ed = ElemDataPrint {
            depth: self.depth + 1,
        };
        WalkResult::Ok(ed)
    }
}

#[derive(Debug)]
pub struct AccumulatorPrint {
    result: String,
}

impl Accumulator<ElemDataPrint, WalkDataPrint> for AccumulatorPrint {
    fn new(e: &Element, ed: &ElemDataPrint) -> Self {
        let result = format!("{}<{}>", INDENT.repeat(ed.depth), e.name.local_name);
        AccumulatorPrint { result }
    }

    fn add(&mut self, wd: &WalkDataPrint) -> WalkResult<WalkDataPrint, WalkError> {
        self.result += &format!("\n{}", wd.data);
        WalkResult::Ok(WalkDataPrint {
            data: self.result.clone(),
        })
    }

    fn summary(&self) -> WalkResult<WalkDataPrint, WalkError> {
        WalkResult::Ok(WalkDataPrint {
            data: self.result.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::{Accumulator, WalkResult, Walkable};
    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use super::{ElemDataPrint, WalkablePrint};

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = WalkablePrint { xml_document: &doc };
        let result = walker.walk(&ElemDataPrint { depth: 0 });

        let res = match result {
            WalkResult::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            WalkResult::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("<n1>\n",
            "    <n2>\n",
            "    <n3>\n",
            "        <n4>"));
    }

    fn create_test_doc() -> XmlDocument {
        let ns: Namespace = Namespace(BTreeMap::<String, String>::new());

        let ei: ElementInfo = ElementInfo {
            lineno: 1,
            attributes: Vec::<OwnedAttribute>::new(),
            namespace: ns,
        };

        let e1 = branch("n1", &ei, vec![
            leaf("n2", &ei),
            branch("n3", &ei, vec![
                leaf("n4", &ei)])
        ]);

        let di = DocumentInfo {
            version: XmlVersion::Version10,
            encoding: "xxx".to_string(),
            standalone: None,
        };

        let d: XmlDocument = XmlDocument {
            root: e1,
            document_info: di,
        };

        d
    }

    fn leaf(name: &str, ei: &ElementInfo) -> Element {
        node(name, ei, Vec::<Element>::new())
    }

    fn branch(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        node(name, ei, subelements)
    }

    fn node(name: &str, ei: &ElementInfo, subelements: Vec<Element>) -> Element {
        Element {
            name: OwnedName {
                local_name: name.to_string(),
                namespace: None,
                prefix: None,
            },
            depth: 0,
            element_info: ei.clone(),
            subelements: subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
