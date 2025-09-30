/**
 * Parse XML text input and produce Rust Schema code.
 */
use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::banner::write_banner_file;
use crate::element::{Element, ElementInfo, element_info_display};
use crate::misc::{nl_indent, owned_name_display, vec_display, XmlDisplay};
use crate::parse_item::LineNumber;
pub use crate::xml_document_error::XmlDocumentError;
use crate::parse_doc::{Accumulator, LevelInfo, ParseDoc};
use crate::document::DocumentInfo;

/*
 * Parse an input stream of XSD code and generate Rust code. That code is
 * then used to guide the parsing of XML code. The XSD is actually XML.
 */
pub struct ParseSchema<'a> {
    pub const_name:     &'a str,
    pub schema_type:    &'a str,
    pub schema_name:    &'a str,
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
    pub depth:          usize,
}

impl<'a> ParseSchema <'a>{
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        ParseSchema {
            const_name:     "FIXMEconstname",
            schema_type:    "FIXMEschematype",
            schema_name:    "FIXMEschemaname",
            document_info,
            root,
            depth:          0,
        }
    }
}

impl<'a> ParseDoc for ParseSchema<'a> {
    type LI = SchemaLevelInfo;
    type AC = SchemaAccumulator;
}

impl LevelInfo for SchemaLevelInfo {
    type AccumulatorType = SchemaAccumulator;

    fn next_level(&self) -> Self {
        SchemaLevelInfo { depth: self.depth + 1 }
    }

    fn create_accumulator(&self, element_info: ElementInfo) ->
        Result<SchemaAccumulator, XmlDocumentError>
    {
        print!("{}<{}>", nl_indent(self.depth), element_info.owned_name.local_name);
        Ok(SchemaAccumulator::new(element_info, self.depth))
    }
}

/*
impl fmt::Display for ParseSchema {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl fmt::Debug for ParseSchema {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        //print_walk(f, 0, self)
    }
}
*/

impl<'a> Try for ParseSchema<'a>
{
    type Output = <<ParseSchema<'a> as ParseDoc>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl<'a> FromResidual for ParseSchema <'a>{
    fn from_residual(_: <ParseSchema as Try>::Residual) -> Self
    { todo!() }
}

/// LevelInfo that tracks depth for indented output
#[derive(Debug, Clone)]
pub struct SchemaLevelInfo {
    depth: usize,
}

impl SchemaLevelInfo {
    pub fn new(_schema: &Box<dyn Element>) -> Self {
        SchemaLevelInfo { depth: 0 }
    }
}

/// Accumulator that just echoes structure (doesn't build elements)
pub struct SchemaAccumulator {
    element: SchemaElement,
    element_name: String,
    element_lineno: LineNumber,
    depth: usize,
    current_subelement_name: Option<String>,
}

impl SchemaAccumulator {
    pub fn new(element_info: ElementInfo, depth: usize) -> Self {
        let ei = element_info.clone();
        let element = SchemaElement::new(ei, vec![], vec![], vec![], vec![]);
        SchemaAccumulator {
            element,
            // FIXME: should use element.name()
            element_name: element_info.owned_name.local_name.clone(),
            element_lineno: element_info.lineno,
            depth,
            current_subelement_name: None,
        }
    }
}

impl Accumulator for SchemaAccumulator {
    type Value = ();  // Schema doesn't return meaningful data

    /*
     * Note that we have started a sublement
     */
    fn start_subelement(&mut self, element_info: &ElementInfo) {
//        println!("SchemaElement::newXXX(\"{}\"", element_info.owned_name.local_name);
        print!("{}", self.element);
        
        // FIXME: probably needs to be fully qualified
        // FIXME: propagate to other parse_.*() code
        self.current_subelement_name = Some(element_info.owned_name.local_name.clone());
    }
    
    fn add_subelement(&mut self, _subelement: ()) {
        // For echo, subelements have already been printed
        // We don't need to do anything with the () value
    }
    
    fn end_subelement(&mut self) {
        if let Some(name) = &self.current_subelement_name {
            print!("{}</{}>", nl_indent(self.depth + 1), name);
        }
        self.current_subelement_name = None;
    }
    
    fn has_open_subelement(&self) -> bool {
        self.current_subelement_name.is_some()
    }
    
    fn current_subelement_name(&self) -> &str {
        self.current_subelement_name.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("")
    }
    
    fn finish(self) -> () {
        print!("{}</{}>", nl_indent(self.depth), self.element_name);
        ()
    }
    
    fn element_name(&self) -> &str {
        &self.element_name
    }
    
    fn element_lineno(&self) -> LineNumber {
        self.element_lineno
    }
}

#[derive(Clone)]
pub struct SchemaElement {
    pub element_info: ElementInfo,
    pub before_element: Vec<XmlEvent>,
    pub content: Vec<XmlEvent>,
    pub after_element: Vec<XmlEvent>,
    pub subelements: Vec<Box<dyn Element>>,
}

impl SchemaElement {
    pub fn new(element_info: ElementInfo,
        before_element: Vec::<XmlEvent>,
        content: Vec::<XmlEvent>,
        after_element: Vec::<XmlEvent>,
        subelements: Vec<Box<dyn Element>>) -> SchemaElement {
        SchemaElement {
            element_info,
            subelements,
            before_element,
            content,
            after_element,
        }
    }
}

impl Default for SchemaElement {
    fn default() -> SchemaElement {
        SchemaElement {
            element_info: ElementInfo {
                owned_name: OwnedName {
                    local_name: "".to_string(),
                    namespace:  None,
                    prefix:     None
                },
                lineno:     0,
            },
            subelements: vec!(),
            before_element: vec!(),
            content: vec!(),
            after_element: vec!(),
        }
    }
}

impl fmt::Display for SchemaElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl fmt::Debug for SchemaElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug(f, 0)
    }
}

impl Element for SchemaElement {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}vec!(Box::new(SchemaElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let owned_name = OwnedName {
            local_name: self.name().to_string(),
            namespace:  None,
            prefix:     None,
        };
        owned_name_display(f, depth + 1, &owned_name)?;

        let element_info = ElementInfo {
            lineno:     0,
            owned_name: owned_name,
        };
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}", nl_indent(depth + 1))?;
        vec_display::<XmlEvent>(f, depth, &self.before_element)?;
        write!(f, ", ")?;
        vec_display::<XmlEvent>(f, depth, &self.content)?;
        write!(f, ", ")?;
        vec_display::<XmlEvent>(f, depth, &self.after_element)?;
        write!(f, ",")?;
        write!(f, "{}vec!(", nl_indent(depth + 1))
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        self.display(f, depth)
    }

    /**
     * Find a subelement (one level deeper) with the given name
     */
    fn get(&self, name: &str) -> Option<&dyn Element> {
println!("get: looking for {} in {}", name, self.name());
println!("...");
for x in self.subelements() {
    println!(" {}", x);
}
        self.subelements()
            .iter()
            .find(|&x| {
                println!("get: is {} == {}", x.name(), name);
                x.name() == name
            })
            .map(|v| &**v)
    }

    /*
     * Return the element name
     */
    // FIXME: maybe remove this from Element
    fn name(&self) -> &str {
        &self.element_info.owned_name.local_name
    }

    fn lineno(&self) -> LineNumber {
        self.element_info.lineno
    }

    /**
     * Return a vector of all subelements.
     */
    fn subelements<'b>(&'b self) -> &'b Vec<Box<dyn Element + 'static>> {
        &self.subelements
    }

    /**
     * Return a mutable vector of all subelements.
     */
    fn subelements_mut<'b>(&'b mut self) -> &'b mut Vec<Box<dyn Element + 'static>> {
        &mut self.subelements
    }
}

impl XmlDisplay for SchemaElement {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {

        write!(f, "{}Box::new(SchemaElement::new(", nl_indent(depth))
            .expect("Unable to write Box::new");

        let element_info = ElementInfo {
            lineno: 0,
            owned_name: OwnedName {
                        local_name: self.name().to_string(),
                        namespace:  None,
                        prefix:     None,
            },
        };

        owned_name_display(f, depth + 1, &element_info.owned_name)?;
        element_info_display(f, depth + 1, &element_info)?;
        write!(f, "{}vec!(), vec!(), vec!(),", nl_indent(depth + 1))?;

        write!(f, "{}vec!(", nl_indent(depth + 1))
    }
}

/*
impl<'a> XmlDisplay for ParseSchema<'a> {
    fn print(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        write!(f, "{}{}", nl_indent(depth), self.inner)
    }
}
*/

pub struct ParseSchemaPrint {
}

/*
impl<'a> fmt::Display for ParseSchema<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)?;
        writeln!(f, "schema_name: {}", self.inner.schema_name)?;
        Ok(())
    }
}
*/

/*
 * Top-level definition of the schema
 * schema_name:     Name of the schema
 * const_name:      Const name. This is how the code refers to the schema
 * xml_document:    XML document
 */
//#[derive(Clone)]
/*
pub struct ParseSchemaInner<'a> {
    pub const_name:     &'a str,
    pub schema_type:    &'a str,
    pub schema_name:    &'a str,
//    pub xml_document:   XmlTree,
}

impl<'a> ParseSchemaInner<'a> {
    pub fn display(&self) {
        println!("ParseSchemaInner::display");
    }
}
*/

impl fmt::Display for ParseSchema<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = 0;
        front_matter_display(f, depth)?;

        let indent_str = nl_indent(depth);
        write!(f, "{}lazy_static! {{", indent_str)?;

        static_parse_schema_display(f, depth + 1, self.const_name, self.schema_type, self.schema_name)?;

unimplemented!();
//        print_walk(f, depth + 2, &self.xml_document)?;

//        back_matter_display(f, 1)?;
//        Ok(())
    }
}

impl fmt::Debug for ParseSchema<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "inner \"{}\" (\"{}\")", self.const_name, self.schema_name)
//        writeln!(f, "inner \"{}\" (\"{}\")", self.const_name, self.schema_name)?;
//        writeln!(f, "xml_document {:?}", self.xml_document)
    }
}

impl XmlDisplay for ParseSchema<'_> {
    fn print(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        write!(f, "{}{}", nl_indent(depth), self)
    }
}

fn front_matter_display(f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    let front_matter: Vec::<&str> = vec!(
        "// FIXME: insert banner",
        "// Auto-generated file",
        "use lazy_static::lazy_static;", 
        "use std::collections::BTreeMap;",
        "", 
        "use xml::common::XmlVersion;",
        "use xml::name::OwnedName;",
        "use xml::namespace::Namespace;",
        "",
        "use crate::xml_document::TreeElement;", 
        "use crate::parse_tree::{DocumentInfo, ElementInfo};",
        "use crate::parse_schema::ParseSchema;", 
        "use crate::XmlTree;",
        "", 
    );

    write_banner_file(f)?;

    let indent_str = nl_indent(depth);

    for front in front_matter {
        write!(f, "{}{}", indent_str, front)?;
    }

    Ok(())
}

fn static_parse_schema_display(f: &mut fmt::Formatter, depth: usize, const_name: &str, schema_type: &str, schema_name: &str) -> fmt::Result {
    let indent_str = nl_indent(depth);
    write!(f, "{}pub static ref {const_name}: {schema_type}<'static> = {schema_type}::new(", indent_str)?;

    let indent_str = nl_indent(depth + 1);
    for name in [const_name, schema_type, schema_name] {
        write!(f, "{}\"{}\",", indent_str, name)?;
    }

    Ok(())
}

fn back_matter_display(f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    write!(f, "{});", nl_indent(depth))?;
    write!(f, "{}}}", nl_indent(depth - 1))
// FIXME: is this needed?
// write!(f, "\n")
}

/*
#[cfg(test)]
mod tests {
    use stdext::function_name;
    use std::io::{BufReader, Cursor};

    use crate::parse_doc::ParseDoc;

    use super::{SchemaLevelInfo, ParseSchema};

    #[test]
    fn testit() {
        println!("Running test {}", function_name!());

        let input_str = 
            "<!--  \n".to_owned() +
            "\n" +
            "Just supply a few elements. This will only work for non-checking code.\n" +
            " -->\n" +
            "<schema xmlns:xtce=\"http://www.omg.org/spec/XTCE/20180204\" xmlns=\"http://www.w3.org/2001/XMLSchema\" targetNamespace=\"http://www.omg.org/spec/XTCE/20180204\" elementFormDefault=\"qualified\" attributeFormDefault=\"unqualified\" version=\"1.2\">\n" +
            "    <one>\n" +
            "       <two>\n" +
            "          <three>\n" +
            "          </three>\n" +
            "       </two>\n" +
            "    </one>\n" +
            "    <four>\n" +
            "    </four>\n" +
            "</schema>\n";
        for (lineno, line) in input_str.split('\n').enumerate() {
            println!("{} {}", lineno, line);
        }

        let cursor = Cursor::new((&input_str).as_bytes());
        let reader = BufReader::new(cursor);

        let echo_level_info = SchemaLevelInfo::new();

        // FIXME: Handle returned error
        let _ = ParseSchema::parse(reader, &echo_level_info);
        println!();
    }
}
*/

/*
use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};

use crate::element::{Element, ElementInfo};
use crate::parse_item::LineNumber;
pub use crate::xml_document_error::XmlDocumentError;
use crate::parse_doc::{Accumulator, LevelInfo, ParseDoc};
use crate::document::DocumentInfo;
use crate::walk_print::nl_indent;

pub struct ParseSchema {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
    pub depth:          usize,
}
/// LevelInfo that doesn't track depth or any other information

impl ParseSchema {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> Self {
        ParseSchema {
            document_info,
            root,
            depth:          0,
        }
    }
}

impl ParseDoc for ParseSchema {
    type LI = SchemaLevelInfo;
    type AC = SchemaAccumulator;
}

impl LevelInfo for SchemaLevelInfo {
    type AccumulatorType = SchemaAccumulator;

    fn next_level(&self) -> Self {
        SchemaLevelInfo { depth: self.depth + 1 }
    }

    fn create_accumulator(&self, element_info: ElementInfo) ->
        Result<SchemaAccumulator, XmlDocumentError>
    {
        print!("{}<{}>", nl_indent(self.depth), element_info.owned_name.local_name);
        Ok(SchemaAccumulator::new(element_info, self.depth))
    }
}

impl fmt::Display for ParseSchema {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl fmt::Debug for ParseSchema {
// FIXME: make this work
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        //print_walk(f, 0, self)
    }
}

impl Try for ParseSchema
{
    type Output = <<ParseSchema as ParseDoc>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl FromResidual for ParseSchema {
    fn from_residual(_: <ParseSchema as Try>::Residual) -> Self
    { todo!() }
}

/// LevelInfo that tracks depth for indented output
#[derive(Debug, Clone)]
pub struct SchemaLevelInfo {
    depth: usize,
}

impl SchemaLevelInfo {
    pub fn new() -> Self {
        SchemaLevelInfo { depth: 0 }
    }
}

/// Accumulator that just echoes structure (doesn't build elements)
pub struct SchemaAccumulator {
    element_name: String,
    element_lineno: LineNumber,
    depth: usize,
    current_subelement_name: Option<String>,
}

impl SchemaAccumulator {
    pub fn new(element_info: ElementInfo, depth: usize) -> Self {
        SchemaAccumulator {
            element_name: element_info.owned_name.local_name.clone(),
            element_lineno: element_info.lineno,
            depth,
            current_subelement_name: None,
        }
    }
}

impl Accumulator for SchemaAccumulator {
    type Value = ();  // Schema doesn't return meaningful data

    fn start_subelement(&mut self, _element_info: &ElementInfo) {
        // Nothing special needed
    }
    
    fn add_subelement(&mut self, _subelement: ()) {
        // For echo, subelements have already been printed
        // We don't need to do anything with the () value
    }
    
    fn end_subelement(&mut self) {
        if let Some(name) = &self.current_subelement_name {
            print!("{}</{}>", nl_indent(self.depth + 1), name);
        }
        self.current_subelement_name = None;
    }
    
    fn has_open_subelement(&self) -> bool {
        self.current_subelement_name.is_some()
    }
    
    fn current_subelement_name(&self) -> &str {
        self.current_subelement_name.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("")
    }
    
    fn finish(self) -> () {
        print!("{}</{}>", nl_indent(self.depth), self.element_name);
        ()
    }
    
    fn element_name(&self) -> &str {
        &self.element_name
    }
    
    fn element_lineno(&self) -> LineNumber {
        self.element_lineno
    }
}

#[cfg(test)]
mod tests {
    use stdext::function_name;
    use std::io::{BufReader, Cursor};

    use crate::parse_doc::ParseDoc;

    use super::{SchemaLevelInfo, ParseSchema};

    #[test]
    fn testit() {
        println!("Running test {}", function_name!());

        let input_str = 
            "<!--  \n".to_owned() +
            "\n" +
            "Just supply a few elements. This will only work for non-checking code.\n" +
            " -->\n" +
            "<schema xmlns:xtce=\"http://www.omg.org/spec/XTCE/20180204\" xmlns=\"http://www.w3.org/2001/XMLSchema\" targetNamespace=\"http://www.omg.org/spec/XTCE/20180204\" elementFormDefault=\"qualified\" attributeFormDefault=\"unqualified\" version=\"1.2\">\n" +
            "    <one>\n" +
            "       <two>\n" +
            "          <three>\n" +
            "          </three>\n" +
            "       </two>\n" +
            "    </one>\n" +
            "    <four>\n" +
            "    </four>\n" +
            "</schema>\n";
        for (lineno, line) in input_str.split('\n').enumerate() {
            println!("{} {}", lineno, line);
        }

        let cursor = Cursor::new((&input_str).as_bytes());
        let reader = BufReader::new(cursor);

        let echo_level_info = SchemaLevelInfo::new();

        // FIXME: Handle returned error
        let _ = ParseSchema::parse(reader, &echo_level_info);
        println!();
    }
}
*/
