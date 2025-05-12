/*
/*
 * Recursive print
 */

use std::fmt;

use crate::xml_document::{Element, XmlDocument};

use super::{Accumulator, ElemData, WalkData, WalkError, Walkable};
//use super::WalkableResult;

const INDENT: &str = "    ";

struct WalkAndPrint<'a> {
    xml_document: &'a XmlDocument,
    f: &'a mut fmt::Formatter<'a>,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new(xml_document: &'a XmlDocument, f: &'a mut fmt::Formatter<'a>) -> WalkAndPrint<'a> {
        WalkAndPrint {
            xml_document:   xml_document,
            f:              f,
        }
    }
}

impl Walkable<'_, PrintAccumulator, PrintElemData, PrintWalkData, fmt::Result>
for WalkAndPrint<'_> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

#[derive(Debug)]
pub struct PrintAccumulator {
    result: String,
}

impl Accumulator<'_, PrintElemData, PrintWalkData, fmt::Result> for PrintAccumulator {
    fn new(e: &Element, ed: &PrintElemData) -> Self {
        let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
        PrintAccumulator { result }
    }

    fn add(&mut self, wd: &PrintWalkData) -> 
        Result<(), WalkError> {
        self.result += &format!("\n{}", wd.data);
        Result::Ok(())
    }

    fn summary(&self) -> fmt::Result {
        Result::Ok(())
    }
}

// ----------------- Data Types ----------------

//type PrintWalkableResult = Result<PrintWalkData, WalkError>;

//impl WalkableResult for PrintWalkableResult {
//}

#[derive(Debug)]
pub struct PrintWalkData {
    pub data: String,
}

impl WalkData for PrintWalkData {}

#[derive(Debug)]
pub struct PrintElemData {
    pub depth: usize,
}

impl PrintElemData {
    fn new(depth: usize) -> PrintElemData {
        PrintElemData {
            depth:  depth,
        }
    }
}

impl ElemData for PrintElemData {
    type Output = PrintElemData;

    fn next_level<'a>(&'a self, element: &Element) ->
        Result<Self::Output, WalkError> {
        println!("{}{}", INDENT.repeat(self.depth), element.name.local_name);
        let ed = PrintElemData {
            depth: self.depth + 1,
        };

        Result::Ok(ed)
    }
}

/*
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use std::fmt;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::Walkable;
    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use super::{PrintElemData, WalkAndPrint};


    struct TestWalk<'a> {
        xml_doc:    &'a XmlDocument,
    }

    impl<'a> TestWalk<'a> {
        fn new(doc: &'_ XmlDocument) -> TestWalk {
            TestWalk {
                xml_doc:    doc,
            }
        }
    }

    impl<'a> fmt::Display for TestWalk<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let walker = WalkAndPrint::new(&self.xml_doc, f);
            let ped = PrintElemData {
                depth:  0,
            };
            walker.walk(ped)
        }
    }

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let test_doc = create_test_doc(); // build a sample XmlDocument
        println!("doc: {}", test_doc);

/*
        let res = match result {
            fmt::Result::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            io::Result::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("<n1>\n",
            "    <n2>\n",
            "    <n3>\n",
            "        <n4>"));
*/
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
            element_info: ei.clone(),
            subelements: subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}
*/

/*

// ----------------- Data Types ----------------

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
        element_info: ei.clone(),
        subelements: subelements,
        before_element: Vec::<XmlEvent>::new(),
        content: Vec::<XmlEvent>::new(),
        after_element: Vec::<XmlEvent>::new(),
    }
}
*/

/*
use std::fmt;

use crate::WalkError;
use crate::xml_document::{Element, XmlDocument};

use crate::{Accumulator, ElemData, WalkData, Walkable};

const INDENT: &str = "    ";

pub struct WalkAndPrint<'a> {
    xml_document:   &'a XmlDocument,
    f:              &'a mut fmt::Formatter<'a>,
}

impl<'a> WalkAndPrint<'a> {
    pub fn new<'b>(xml_document: &'a XmlDocument, f: &'b mut fmt::Formatter<'b>) -> WalkAndPrint<'b> {
        WalkAndPrint {
            xml_document:   xml_document,
            f:              f,
        }
    }
}

impl<'a> Walkable<'a, PrintAccumulator, PrintElemData, PrintWalkData> for WalkAndPrint<'a> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

struct PrintWalkable<'a> {
    xml_document: &'a XmlDocument,
}

impl Walkable<'_, PrintAccumulator, PrintElemData, PrintWalkData> for PrintWalkable<'_> {
    fn xml_document(&self) -> &XmlDocument {
        self.xml_document
    }
}

// ----------------- Data Types ----------------

#[derive(Debug)]
pub struct PrintWalkData {
    pub data: String,
}

impl WalkData for PrintWalkData {}

#[derive(Debug)]
pub struct PrintElemData {
    pub depth: usize,
}

impl PrintElemData {
    fn new(depth: usize) -> PrintElemData {
        PrintElemData {
            depth:  depth,
        }
    }

    fn indent(&self) -> String {
        INDENT.repeat(self.depth)
    }
}

impl ElemData for PrintElemData {
    type Output = PrintElemData;

    fn next_level<'a>(&'a self, element: &Element) ->
        Result<Self::Output, WalkError> {
        println!("{}<{}", INDENT.repeat(self.depth), element.name.local_name);
        let ed = PrintElemData {
            depth: self.depth + 1,
        };

        Result::Ok(ed)
    }
}

#[derive(Debug)]
pub struct PrintAccumulator {
    result: String,
}

impl Accumulator<'_, PrintElemData, PrintWalkData> for PrintAccumulator {
    fn new(e: &Element, ed: &PrintElemData) -> Self {
        let result = format!("{}<{}>", ed.indent(), e.name.local_name);
        PrintAccumulator { result }
    }

    fn add(&mut self, wd: &PrintWalkData) -> fmt::Result {
        self.result += &format!("\n{}", wd.data);
        fmt::Result::Ok(())
    }

    fn summary(&self) -> fmt::Result {
        fn new(e: &Element, ed: &PrintElemData) -> PrintAccumulator {
            let result = format!("{}{}", INDENT.repeat(ed.depth), e.name.local_name);
            PrintAccumulator { result }
        }
    }
}


mod tests {
    use std::collections::BTreeMap;
    use xml::attribute::OwnedAttribute;
    use xml::common::XmlVersion;
    use xml::name::OwnedName;
    use xml::namespace::Namespace;
    use xml::reader::XmlEvent;

    use crate::xml_document::{Element, ElementInfo, XmlDocument};
    use crate::xml_document_factory::DocumentInfo;

    use super::{PrintElemData, PrintWalkable};

    const INDENT: &str = "    ";

    #[test]
    fn test_walk_tree_names() {
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = PrintWalkable { xml_document: &doc };
        println!("Processing:");
        let ed = PrintElemData::new(0);
        let result = walker.walk(ed);

        let res = match result {
            Result::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            Result::Err(e) => {
                let res = format!("{}", e);
                eprintln!("Error: {}", res);
                res
            }
        };
        assert_eq!(res, concat!("n1\n", "    n2\n", "    n3\n", "        n4"));
    }


    // ----------------- Data Types ----------------

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
            element_info: ei.clone(),
            subelements: subelements,
            before_element: Vec::<XmlEvent>::new(),
            content: Vec::<XmlEvent>::new(),
            after_element: Vec::<XmlEvent>::new(),
        }
    }
}

/*
// ----------------- Data Types ----------------

pub struct PrintWalkData {}

//impl () for PrintWalkData {}

impl WalkData for PrintWalkData {}

pub struct PrintElemData {
    pub depth:  usize,
}

impl PrintElemData {
    pub fn new(depth: usize) -> PrintElemData {
        PrintElemData {
            depth:  depth,
        }
    }

    fn indent(&self) -> String {
        INDENT.repeat(self.depth)
    }
}

    impl ElemData for PrintElemData {
        type Output = PrintElemData;

        fn next_level<'a>(&'a self, element: &Element) ->
            Result<Self::Output, WalkError> {
impl ElemData<'_, PrintElemData, PrintWalkData, PrintAccumulator, fmt::Result> for PrintElemData {
    fn next_level(&mut self, w: new(&WalkAndPrint), element: &Element) -> fmt::Result {
        write!(w.f, "{}<{}>", self.indent(), element.name);
        let ed = PrintElemData::new(self.depth + 1);
        fmt::Result::Ok(ed)
    }
}

#[derive(Debug)]
pub struct PrintAccumulator {
    result: String,
}

impl<'a> Accumulator<'a, PrintElemData, PrintWalkData, PrintAccumulator, fmt::Result> for PrintAccumulator {
    fn new(e: &Element, ed: &PrintElemData) -> Self {
        let result = format!("{}<{}>", ed.indent(), e.name.local_name);
        PrintAccumulator { result }
    }

    fn add(&mut self, wd: PrintWalkData) -> fmt::Result {
        self.result += &format!("\n{}", wd.data);
        fmt::Result::Ok(())
    }

    fn summary(&self) -> fmt::Result {
        fmt::Result::Ok(())
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

    use crate::{Accumulator, Walkable};
    use crate::xml_document::{Element, XmlDocument};
    use crate::xml_document_factory::{DocumentInfo, ElementInfo};
    use super::{PrintElemData, WalkAndPrint};

    #[test]
    fn test_walk_tree_print() {
        println!("\nStart test_walk_tree_print");
        let doc = create_test_doc(); // build a sample XmlDocument
        let walker = WalkAndPrint::new(&doc);
        let result = walker.walk(&PrintElemData { depth: 0 });

        let res = match result {
            fmt::Result::Ok(data) => {
                let res = format!("{}", data.data);
                println!("Output:\n{}", res);
                res
            }
            io::Result::Err(e) => {
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
*/
*/
*/
