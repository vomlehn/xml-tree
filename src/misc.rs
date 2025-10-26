/*
 * Miscellaneous small functions
 */

use std::fmt;
use std::io::Write;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

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
pub fn display_vec<T, F>(output: &mut dyn Write, depth: usize, vec: &Vec<T>,
    to_string: F) -> fmt::Result
    where
        F: for<'a> Fn(&'a T) -> String,
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
            let e = to_string(elem);
            let _ = writeln!(output, "{}{}", this_indent, e);
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

pub fn xml_event_to_string<'a>(xml_event: &'a XmlEvent) -> String {
    format!("{}", DisplayXmlEvent(xml_event))
}

pub fn owned_attribute_to_string<'a>(attribute: &'a OwnedAttribute) -> String {
    format!("{:?}", attribute)
}
