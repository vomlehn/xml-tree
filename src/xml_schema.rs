/*
 * Define the data structures used to describe the XML used for parsing.
 */
// FIXME: make sure errors returned are appropriate

use std::fmt;
// FIXME: implement some more iterators

use crate::banner::write_banner_file;
use crate::xml_document_tree::XmlDocumentTree;
use crate::xml_document_tree::XmlTreeFactory;
use crate::walk_and_print::{nl_indent, print_walk, XmlDisplay};

pub struct XmlSchema<'a> {
    pub inner: XmlSchemaInner<'a>,
}

// FIXME: remove unsafe
unsafe impl<'a> Sync for XmlSchema<'a> {
}

impl<'a> XmlSchema<'a> {
    pub fn display(&self) {
        println!("{}", self.inner);
    }

    pub fn new(const_name: &'a str, schema_type: &'a str, schema_name: &'a str, xml_document: XmlTreeFactory) -> XmlSchema<'a> {
        XmlSchema {
            inner:  XmlSchemaInner {
                const_name,
                schema_type,
                schema_name,
                xml_document,
            }
        }
    }
}

impl<'a> XmlDisplay for XmlSchema<'a> {
    fn print(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        write!(f, "{}{}", nl_indent(depth), self.inner)
    }
}

pub struct XmlSchemaPrint {
}

impl<'a> fmt::Display for XmlSchema<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)?;
        writeln!(f, "schema_name: {}", self.inner.schema_name)?;
        Ok(())
    }
}

/*
 * Top-level definition of the schema
 * schema_name:     Name of the schema
 * const_name:      Const name. This is how the code refers to the schema
 * xml_document:    XML document
 */
//#[derive(Clone)]
pub struct XmlSchemaInner<'a> {
    pub const_name:     &'a str,
    pub schema_type:    &'a str,
    pub schema_name:    &'a str,
    pub xml_document:   XmlTreeFactory,
}

impl<'a> XmlSchemaInner<'a> {
    pub fn display(&self) {
        println!("XmlSchemaInner::display");
    }
}

impl fmt::Display for XmlSchemaInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = 0;
        front_matter_display(f, depth)?;

        let indent_str = nl_indent(depth);
        write!(f, "{}lazy_static! {{", indent_str)?;

        static_xml_schema_display(f, depth + 1, self.const_name, self.schema_type, self.schema_name)?;

        print_walk(f, depth + 2, &self.xml_document)?;

        back_matter_display(f, 1)?;
        Ok(())
    }
}

impl fmt::Debug for XmlSchemaInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "inner \"{}\" (\"{}\")", self.const_name, self.schema_name)?;
        writeln!(f, "xml_document {:?}", self.xml_document)
    }
}

impl XmlDisplay for XmlSchemaInner<'_> {
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
        "use crate::xml_document::DirectElement;", 
        "use crate::xml_document_factory::{DocumentInfo, ElementInfo};",
        "use crate::xml_schema::XmlSchema;", 
        "use crate::XmlDocument;",
        "", 
    );

    write_banner_file(f)?;

    let indent_str = nl_indent(depth);

    for front in front_matter {
        write!(f, "{}{}", indent_str, front)?;
    }

    Ok(())
}

fn static_xml_schema_display(f: &mut fmt::Formatter, depth: usize, const_name: &str, schema_type: &str, schema_name: &str) -> fmt::Result {
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
