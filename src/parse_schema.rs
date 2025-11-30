/**
 * Parse XML text input and produce Rust Schema code.
 */

//use std::fs::File;
use std::fmt;
use std::io::{BufReader, Read, Write};
use std::ops::{ControlFlow, FromResidual, Try};
//use xml::attribute::OwnedAttribute;
//use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::banner::write_banner_file;
use crate::element::{Element, ElementInfo};
use crate::misc::{nl_indent, path_string, write_vec, rust_xml_event};
use crate::ParseLoc;
pub use crate::xml_document_error::XmlDocumentError;
use crate::parse_xml::{Accumulator, LevelInfo, ParseXml};
use crate::document::DocumentInfo;

use crate::{ELEMENT_INDENTS, TREE_DEPTH};

/*
 * Parse an input stream of XSD code and generate Rust code. That code is
 * then used to guide the parsing of XML code. The XSD is actually XML.
 */
pub struct ParseSchema<'a> {
    pub document_info:  DocumentInfo,
    pub root:           Box<dyn Element>,
    pub output:         Option<&'a mut dyn Write>,
    pub identifiers:    Vec<Vec<String>>,
}

impl<'a> ParseSchema<'a> {
    pub fn new(document_info: DocumentInfo, root: Box<dyn Element>) -> ParseSchema<'a> {
        ParseSchema {
            document_info,
            root,
            output:         None,
            identifiers: vec!(),
        }
    }

    pub fn parse_path (
        &mut self,
        params:             &ParseSchemaParams,
        path:               &'a str,
        element_level_info: &<ParseSchema<'a> as ParseXml<'a>>::LI,
        output:             &'a mut dyn Write,
    ) -> Result<(DocumentInfo, <<<ParseSchema<'_> as ParseXml<'_>>::LI as LevelInfo<'_>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    {
eprintln!("ParseSchema::parse_path: output is Some");
        self.output = Some(output);

        // FIXME: check for error
//        let _ = writeln!(output, "<!-- in parse_path -->");
        let _ = self.write_start(&params);
        let res = self.parse_path_base(path, element_level_info)?;
        self.write_end();
//        let _ = writeln!(output, "<!-- exiting parse_path -->");
        Ok(res)
    }

    pub fn parse<R>(
        &mut self,
        params:             &ParseSchemaParams,
        buf_reader:         BufReader<R>,
        element_level_info: &<ParseSchema<'a> as ParseXml<'a>>::LI,
        output:             &'a mut dyn Write,
    ) -> Result<(DocumentInfo, <<<ParseSchema<'a> as ParseXml<'a>>::LI as LevelInfo<'a>>::AccumulatorType as Accumulator>::Value), XmlDocumentError>
    where
        R: Read,
    {
eprintln!("ParseSchema::parse: output is Some");
        self.output = Some(output);
        // Be sure to restore before returning
//        println!("<-- In parse -->");
        // FIXME: check for error
        let _ = self.write_start(&params);

        let res = self.parse_base(buf_reader, element_level_info);
        let res = res?;

        self.write_end();

        Ok(res)
    }

    fn write_start(&mut self, params: &ParseSchemaParams) -> fmt::Result {
        let depth = 0;
        self.write_front_matter(params.schema_crate, depth)?;

        self.static_parse_schema_display(depth, params.const_name, params.schema_type,
            params.schema_name)?;

        Ok(())
    }

    /*
     * Generate one-time content for the beginning of the output file
     */
    fn write_front_matter(&mut self, schema_crate: &str, depth: usize) -> fmt::Result {
        let front_matter: Vec::<&str> = vec!(
            "// FIXME: insert banner",
            "// Auto-generated file",
//            "use lazy_static::lazy_static;", 
            "use std::collections::BTreeMap;",
            "", 
            "use xml::attribute::OwnedAttribute;",
            "use xml::name::OwnedName;",
            "use xml::namespace::Namespace;",
            "",
            "use xml_tree::{ElementInfo};",
            "use xml_tree::SchemaElement;",
            "use xml_tree::ParseLoc;",
            "",
            "use ", schema_crate, ";",
            "", 
        );

        let output = self.output.as_mut().expect("output should be Some");

        write_banner_file(output)?;

        let indent_str = nl_indent(depth);

        for front in front_matter {
            // FIXME: check for error
            let _ = write!(output, "{}{}", indent_str, front);
        }

        Ok(())
    }

    /*
     * Generate the constant first part of the schema structure
     */
//FIXME: clean this up
    pub fn static_parse_schema_display(&mut self, depth: usize, _const_name: &str,
        _schema_type: &str, _schema_name: &str) -> fmt::Result {
        let output = self.output.as_mut().expect("output should be Some");

/* FIXME: remove this
        let indent_str = nl_indent(depth);
        // FIXME: check for error
//        let _ = write!(output, "{}pub static ref {const_name}: {schema_type}<'static> = {schema_name}::new(", indent_str);

        let indent_str = nl_indent(depth + 1);
        for name in [const_name, schema_type, schema_name] {
        // FIXME: check for error
            let _ = write!(output, "{}{:?},", indent_str, name);
        }
*/
        let depth1 = depth + 1;
        // FIXME: generate function name properly
        let _ = write!(output, "{}pub fn get_xtce_schema<'a>() -> XtceSchema<'a> {{",
            nl_indent(depth));
        let _ = write!(output, "{}XtceSchema::new(", nl_indent(depth1));

        Ok(())
    }

    /*
     * Generate the constant end of schema structure
     */
    pub fn write_end(&mut self) {
        // FIXME: check for error
        let _ = self.write_back_matter(1);
    }

    fn write_back_matter(&mut self, depth: usize) -> fmt::Result {
        let output = self.output.as_mut().expect("output should be Some");
        // FIXME: check for error
        let _ = write!(output, "{})", nl_indent(depth));
        let _ = write!(output, "{}}}", nl_indent(depth - 1));
        let _ = write!(output, "\n");
//        let _ = writeln!(output, "<!-- write back matter");
        Ok(())
    }
}

pub struct ParseSchemaParams<'a> {
    pub const_name:     &'a str,
    pub schema_type:    &'a str,
    pub schema_name:    &'a str,
    pub schema_crate:   &'a str,
}

impl<'a> ParseXml<'a> for ParseSchema<'a> {
    type LI = SchemaLevelInfo;
    type AC = SchemaAccumulator;
}

impl<'a> Try for ParseSchema<'a> 
{
    type Output = <<ParseSchema<'a> as ParseXml<'a>>::AC as Accumulator>::Value;
    type Residual = XmlDocumentError;
    fn from_output(_: <Self as Try>::Output) -> Self
    { todo!() }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output>
    { todo!() }
}

impl<'a> FromResidual for ParseSchema<'a> {
    fn from_residual(_: <ParseSchema<'a> as Try>::Residual) -> Self
    { todo!() }
}

// LevelInfo<'_> that tracks depth for indented output
// depth:  The number of nested SchemaElements
#[derive(Debug, Clone)]
pub struct SchemaLevelInfo {
    depth:  usize,
    path:   Vec<String>,
}

impl SchemaLevelInfo {
    pub fn new(_schema: &Box<dyn Element>) -> Self {
        SchemaLevelInfo {
            depth:  0,
            path:   vec!(),
        }
    }
}

impl<'a> LevelInfo<'a> for SchemaLevelInfo {
    type ParseXmlType = ParseSchema<'a>;
    type AccumulatorType = SchemaAccumulator;

    fn next_level(&self, element_info: &ElementInfo) -> Self {
        let mut path = self.path.clone();
//        path.push(element_info.owned_name.local_name.clone());
        path.push(element_info.owned_name.local_name.clone());
eprintln!("next_level name {} path {:?}", element_info.owned_name.local_name, &path);
        SchemaLevelInfo {
            depth:  self.depth + 1,
            path:   path,
        }
    }

    fn create_accumulator(&self, parse_xml: &mut Self::ParseXmlType,
        element_info: ElementInfo) -> Result<SchemaAccumulator, XmlDocumentError>
    {
        Ok(SchemaAccumulator::new(element_info, self.depth, parse_xml, &self.path))
    }
}

/// Accumulator that just echoes structure (doesn't build elements)
pub struct SchemaAccumulator {
    element:                    SchemaElement,
    element_name:               String,
    parse_loc:                  ParseLoc,
    depth:                      usize,
    current_subelement_name:    Option<String>,
    path:                       Vec<String>,
}

impl SchemaAccumulator {
    pub fn new(element_info: ElementInfo, depth: usize,
            parse_schema: &mut ParseSchema<'_>, path: &Vec<String>) -> Self {
        let output = parse_schema.output.as_mut().expect("output should be Some");
        let ei = element_info.clone();
        let depth1 = depth + 1;
//        let depth2 = depth + 2;
        let element = SchemaElement::new(ei, depth1, vec![], vec![], vec![], vec![]);
        // FIXME: check for errors
        let _ = element.write_start(output, depth,
            "SchemaElement".to_string());
eprintln!("SchemaAccumulator::new: path {:?}", path);
        parse_schema.identifiers.push(path.clone());

        SchemaAccumulator {
            element,
            // FIXME: should use element.name()
            element_name:               element_info.owned_name.local_name.clone(),
            parse_loc:                  element_info.parse_loc,
            depth:                      depth,
            current_subelement_name:    None,
            path:                       path.clone(),
        }
    }
}

impl Accumulator for SchemaAccumulator {
    type Value = ();  // Schema doesn't return meaningful data
    type DocType<'a> = ParseSchema<'a>;

    /*
     * Note that we have started a sublement
     */
    fn start_subelement(&mut self, parse_schema: &mut ParseSchema<'_>, element_info: &ElementInfo) {
eprintln!("start_subelement name {}", element_info.owned_name.local_name);
/*
        // FIXME: probably needs to be fully qualified
        // FIXME: propagate to other parse_.*() code
        self.current_subelement_name = Some(element_info.owned_name.local_name.clone());

        parse_schema.identifiers.push(path);
eprintln!("start_subelement identifiers {:?}", parse_schema.identifiers);
*/
    }
    
    fn add_subelement(&mut self, _parse_schema: &mut ParseSchema<'_>, _subelement: ()) {
        // We don't need to do anything with the () value
        self.element.has_subelements = true;
    }
    
    fn end_subelement(&mut self, _parse_schema: &mut ParseSchema<'_>) {
        // FIXME: what's this for? I think I should be verifying the names are the same
        if let Some(_name) = &self.current_subelement_name {
        }
        self.current_subelement_name = None;
    }
    
    fn finish(self, parse_schema: &mut ParseSchema<'_>) -> Self::Value {
        let output = parse_schema.output.as_mut().expect("output should be Some");

        // FIXME: return error
        let _ = self.element.write_end(output, self.depth);
        ()
    }
    
    fn has_open_subelement(&self) -> bool {
        self.current_subelement_name.is_some()
    }
    
    fn current_subelement_name(&self) -> &str {
        self.current_subelement_name.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("")
    }
    
    fn element_name(&self) -> &str {
        &self.element_name
    }
    
    fn parse_loc(&self) -> ParseLoc {
        self.parse_loc.clone()
    }
}

#[derive(Clone)]
pub struct SchemaElement {
    pub element_info:       ElementInfo,
    pub depth:              usize,
    pub before_element:     Vec<XmlEvent>,
    pub content:            Vec<XmlEvent>,
    pub after_element:      Vec<XmlEvent>,
    pub subelements:        Vec<Box<dyn Element>>,
    pub has_subelements:    bool,
}

impl SchemaElement {
    pub fn new(element_info: ElementInfo,
        depth:          usize,
        before_element: Vec::<XmlEvent>,
        content: Vec::<XmlEvent>,
        after_element: Vec::<XmlEvent>,
        subelements: Vec<Box<dyn Element>>) -> SchemaElement {
        SchemaElement {
            element_info,
            depth,
            subelements,
            before_element,
            content,
            after_element,
            has_subelements:    false,
        }
    }

    /*
     * Print the first part of the SchemaElement
     * self:    self
     * output:  Where to write the text
     * depth:   Number of nested SchemaElement
     */
    pub fn write_start(&self, output: &mut dyn Write, depth: usize, name: String) ->
        fmt::Result {
        let depth0 = TREE_DEPTH + ELEMENT_INDENTS * depth;
        let depth1 = depth0 + 1;

        // FIXME: return error code
        let _ = write!(output, "{}Box::new({}::new(",
            nl_indent(depth0), name);

        // FIXME: check for errors
        let _ = self.element_info.write(output, depth1);
        let _ = write!(output, "{}", nl_indent(depth1));

        let _ = write!(output, "{}, ", depth);

        let _ = write_vec::<XmlEvent, fn (&XmlEvent, usize) -> String>(output, depth1,
            &self.before_element, rust_xml_event as fn(&XmlEvent, usize) -> String);
        let _ = write!(output, ", ");

        let _ = write_vec::<XmlEvent, fn (&XmlEvent, usize) -> String>(output, depth1,
            &self.content, rust_xml_event as fn(&XmlEvent, usize) -> String);
        let _ = write!(output, ", ");

        let _ = write_vec::<XmlEvent, fn (&XmlEvent, usize) -> String>(output, depth1,
            &self.after_element, rust_xml_event as fn(&XmlEvent, usize) -> String);
        let _ = write!(output, ",");

        // This defines the start of the SchemaElement subelements
        let _ = write!(output, " vec!(");
        Ok(())
    }

    // FIXME: remove _element
    pub fn write_end(&self, output: &mut dyn Write, depth: usize) -> fmt::Result {
        let depth0 = TREE_DEPTH + ELEMENT_INDENTS * depth;
        let depth1 = depth0 + 1;
        let _depth2 = depth0 + 2;

        // FIXME: check for errors
        // Close off the list of subelements
        if !self.has_subelements {
            let _ = write!(output, ") /* Close subelement list 0 */");
        } else {
            let _ = write!(output, "{}) /* Close subelement list 1 */", nl_indent(depth1));
        }

        let _ = write!(output, "{})), /* Close vec!>Box::new>SchemaElement::new> */",
            nl_indent(depth0));

        Ok(())
    }
}

impl Element for SchemaElement {

    /**
     * Find a subelement (one level deeper) with the given name
     */
    fn get(&self, name: &str) -> Option<&dyn Element> {
/*
println!("get: looking for {} in {}", name, self.name());
println!("...");
for x in self.subelements() {
    println!(" {}", x);
}
*/
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

    fn parse_loc(&self) -> ParseLoc {
        self.element_info.parse_loc.clone()
    }

    /**
     * Return a vector of all subelements.
     */
    fn subelements(&self) -> &Vec<Box<dyn Element>> {
        &self.subelements
    }

    /**
     * Return a mutable vector of all subelements.
     */
    fn subelements_mut(&mut self) -> &mut Vec<Box<dyn Element>> {
        &mut self.subelements
    }
}
