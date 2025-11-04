/*
 * Miscellaneous small functions
 */

use std::fmt;
use std::io::Write;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::XmlEvent;

/*
 * Both indent() and nl_indent() produce strings used for indentation whose lengths
 * are a multiple of their arguments. Rust outputs have a header, which is followed
 * by the XML schema tree.
 * TREE_DEPTH       The minimum indentation of the schema tree
 * ELEMENT_INDENTS  The indentation from on ElementSchema to the next nested
 *                  ElementSchema.
 * INDENT           String corresponding to indenting once.
 */
pub const TREE_DEPTH: usize = 2;
pub const ELEMENT_INDENTS: usize = 2;
const INDENT: &str = "    ";

pub fn nl_indent(n: usize) -> String {
    "\n".to_owned() + &indent(n)
}

pub fn indent(n: usize) -> String {
    INDENT.repeat(n)
}

/**
 * Print a vector of elements of the given type
 * T:       Type of vector elements
 * f:       Formatter
 * depth:   Indentation
 */
// FIXME: uses of this need to be cleaned up and consolidated
pub fn write_vec<T, F>(output: &mut dyn Write, depth: usize, vec: &Vec<T>,
    to_string: F) -> fmt::Result
    where
        F: for<'a> Fn(&'a T, usize) -> String,
{
    let depth1 = depth + 1;
    let depth2 = depth + 2;
    // FIXME: check for errors
    if vec.is_empty() {
        let _ = write!(output, "vec!()");
    } else {
        let _ = write!(output, "{}vec!(", nl_indent(depth1));
        let mut this_indent = "".to_string();
        for elem in vec {
            let e = to_string(elem, depth);
            let _ = writeln!(output, "{}{},", this_indent, e);
            this_indent = indent(depth2);
        }
        let _ = write!(output, "{})", indent(depth1));
    }

    Ok(())
}

pub struct DisplayXmlEvent<'a>(pub &'a XmlEvent);

impl<'a> fmt::Display for DisplayXmlEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use a match statement to handle each variant of XmlEvent
        match self.0 {
            XmlEvent::StartDocument { version, encoding, standalone } => {
                write!(f, "StartDocument(version: {:?}, encoding: {:?}, standalone: {:?})", version, encoding, standalone)
            }
            XmlEvent::EndDocument => {
                write!(f, "EndDocument")
            }
            XmlEvent::StartElement { name, attributes, .. /*namespace*/ } => {
                write!(f, "StartElement(name: {}, attrs: {} total)",
                    name.local_name, attributes.len())
            }
            XmlEvent::EndElement { name } => {
                write!(f, "EndElement(name: {})", name.local_name)
            }
            XmlEvent::Characters(s) => {
                // Truncate long strings for cleaner display
                let s_display = format!("{}...", &s[..20].trim());
                write!(f, "Characters(\"{}\")", s_display)
            }
            XmlEvent::Comment(s) => {
                write!(f, "Comment(\"{}\")", s.trim())
            }
            // FIXME: do this right
            // Use Debug format ({:?}) for all other variants for a full representation
            _ => write!(f, "{:?}", self.0),
        }
    }
}

pub fn rust_owned_name(owned_name: &OwnedName, depth: usize) -> String {
    let depth1 = depth + 1;

    // FIXME: check for errors
    let mut result = format!("{}OwnedName{{local_name: {},",
        nl_indent(depth), rust_to_string(&owned_name.local_name)).to_string();
    result += &match &owned_name.namespace {
        None => format!("{}namespace: None,", nl_indent(depth1)),
        Some(namespace) => format!("{}namespace: Some({}),",
            nl_indent(depth1), rust_to_string(&namespace)),
    };
    result += &match &owned_name.prefix {
        None => format!("{}prefix: None", nl_indent(depth1)),
        Some(prefix) => format!("{}prefix: Some({})}}",
            nl_indent(depth1), rust_to_string(prefix)),
    };
    result += "}";

    result
}

pub fn rust_xml_event<'a>(xml_event: &'a XmlEvent, _depth: usize) -> String {
    format!("{}", DisplayXmlEvent(xml_event))
}

pub fn rust_owned_attribute<'a>(attribute: &'a OwnedAttribute, depth: usize) -> String {
    // FIXME: have to print element by element
    let _depth1 = depth + 1;
    let _depth2 = depth + 2;
    let depth3 = depth + 3;
    "OwnedAttribute {".to_string() +
        "name: " + &rust_owned_name(&attribute.name, depth3) + ", " +
        &nl_indent(depth3) +
        "value: " + &rust_to_string(&attribute.value) +
    "}"
}

/*
 * Given an str, returns the value that, when parsed by Rust, will be that string
 * plus to_string().
 */
pub fn rust_to_string(s: &str) -> String {
    rust_str(s) + ".to_string()"
}

/*
 * Returns a value that, when parsed by Rust, will be a valid string. That is,
 * given the String:
 *
 *      abc\"de
 *
 * the result will be the String:
 *
 *      "abc\\\"de
 */
pub fn rust_string(s: &String) -> String {
    format!("{:?}", s)
}

/*
 * Returns a value that, when parsed by Rust, will be a valid string. That is,
 * given the str:
 *
 *      abc\"de
 *
 * the result will be the String:
 *
 *      "abc\\\"de
 */
pub fn rust_str(s: &str) -> String {
    format!("{:?}", s)
}
