/*
 * Miscellaneous small functions
 */

use std::fmt;
use std::io::Write;
use xml::name::OwnedName;

pub fn owned_name_display(output: &mut dyn Write, depth: usize, owned_name: &OwnedName) -> fmt::Result {
    // FIXME: check for errors
    let _ = write!(output, "{}OwnedName{{local_name: \"{}\".to_string(),", nl_indent(depth), owned_name.local_name);
    let _ = write!(output, "{}namespace: {:?}, prefix: {:?}}},", nl_indent(depth + 1), owned_name.namespace, owned_name.prefix);
    Ok(())
}

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
pub fn vec_display<T: fmt::Debug>(output: &mut dyn Write, depth: usize, vec: &Vec<T>) -> fmt::Result
{
//let _ = write!(output, "Z+");
    // FIXME: check for errors
    if vec.is_empty() {
        let _ = write!(output, "vec!()");
    } else {
        let _ = write!(output, "{}vec!(", nl_indent(depth + 1));
        for elem in vec {
//            let e: String = format!("{}{}", indent(depth), elem);
//                elem.print(output, depth);
            // FIXME: switch to non-Debug format
            let e = format_args!("{:?}", elem);
            let _ = writeln!(output, "{}{}", indent(depth), e);
        }
        let _ = write!(output, "{})", nl_indent(depth));
    }
//    let _ = write!(output, "Z-");

    Ok(())
}

/*
pub trait XmlDisplay
{
    fn print(&self, output: &mut dyn Write, depth: usize) -> fmt::Result;
}
*/
