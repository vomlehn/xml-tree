/*
 * Miscellaneous small functions
 */

use std::fmt;
use xml::name::OwnedName;

pub fn owned_name_display(f: &mut fmt::Formatter<'_>, depth: usize, owned_name: &OwnedName) -> fmt::Result {
    write!(f, "{}OwnedName{{local_name: \"{}\".to_string(),", nl_indent(depth), owned_name.local_name)?;
    write!(f, "{}namespace: {:?}, prefix: {:?}}},", nl_indent(depth + 1), owned_name.namespace, owned_name.prefix)
}

const INDENT: &str = "    ";

pub fn nl_indent(n: usize) -> String {
    "\n".to_owned() + &indent(n)
}

pub fn indent(n: usize) -> String {
    INDENT.repeat(n)
}

/**
 * Print a descriptor of the given type.
 * f:       Formatter
 * depth:   Indentation
 */
// FIXME: uses of this need to be cleaned up and consolidated
pub fn vec_display<T>(f: &mut fmt::Formatter, depth: usize, vec: &Vec<T>) -> fmt::Result
where
    T:  XmlDisplay
{
    if vec.is_empty() {
        write!(f, "vec!()")?;
    } else {
        write!(f, "{}vec!(", nl_indent(depth + 1))?;
        for elem in vec {
                elem.print(f, depth)?;
        }
        write!(f, "{})", nl_indent(depth))?;
    }

    Ok(())
}

pub trait XmlDisplay
{
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
}
