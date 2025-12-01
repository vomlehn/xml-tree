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
    let mut result = format!("OwnedName {{");
    result += &format!("{}local_name: {},", nl_indent(depth1),
        &rust_to_string(&owned_name.local_name));

    result += &match &owned_name.namespace {
        None => format!("{}namespace: None,", nl_indent(depth1)),
        Some(namespace) => format!("{}namespace: Some({}),",
            nl_indent(depth1), rust_to_string(&namespace)),
    };
    result += &match &owned_name.prefix {
        None => format!("{}prefix: None", nl_indent(depth1)),
        Some(prefix) => format!("{}prefix: Some({})",
            nl_indent(depth1), rust_to_string(prefix)),
    };
    result += &format!("{}}}", nl_indent(depth));

    result
}

pub fn rust_xml_event<'a>(xml_event: &'a XmlEvent, _depth: usize) -> String {
    format!("{}", DisplayXmlEvent(xml_event))
}

pub fn rust_owned_attribute<'a>(attribute: &'a OwnedAttribute, depth: usize) -> String {
    // FIXME: have to print element by element
    let _depth1 = depth + 1;
    let depth2 = depth + 2;
    let depth3 = depth + 3;
    "OwnedAttribute {".to_string() +
        &nl_indent(depth3) + "name: " + &rust_owned_name(&attribute.name, depth3) + ", " +
        &nl_indent(depth3) + "value: " + &rust_to_string(&attribute.value) +
        &nl_indent(depth2) +
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

pub fn path_string(strings: &[String]) -> Option<String> {
    for s in strings {
        if s.len() == 0 {
            return None;
        }
    }

    Some(strings.iter()
        .map(|s| {
            s.replace("_", "__")
        })
        .collect::<Vec<String>>()
        .join("_"))
}

pub fn string_path(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '_' {
            // Check if next char is also '_'
            if chars.peek() == Some(&'_') {
                // Double underscore: add single underscore to current string
                current.push('_');
                chars.next(); // consume the second underscore
            } else {
                // Single underscore: it's a separator
                result.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }

    // Don't forget the last element
    result.push(current);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_string_no_underscores() {
        let v = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        let actual = path_string(&v).expect("Bad path item");
        assert_eq!(actual, "hello_world_rust");
    }

    #[test]
    fn test_path_string_single_underscore() {
        let v = vec!["hello".to_string(), "my_world".to_string(), "rust".to_string()];
        let actual = path_string(&v).expect("Bad path item");
        assert_eq!(actual, "hello_my__world_rust");
    }

    #[test]
    fn test_path_string_multiple_underscores() {
        let v = vec!["hello".to_string(), "my_big_world".to_string(), "rust".to_string()];
        let actual = path_string(&v).expect("Bad path item");
        assert_eq!(actual, "hello_my__big__world_rust");
    }

    #[test]
    fn test_path_string_empty_vector() {
        let v: Vec<String> = vec![];
        let actual = path_string(&v).expect("Bad path item");
        assert_eq!(actual, "");
    }

    #[test]
    fn test_path_string_single_element() {
        let v = vec!["hello_world".to_string()];
        let actual = path_string(&v).expect("Bad path item");
        assert_eq!(actual, "hello__world");
    }

    #[test]
    fn test_path_string_consecutive_underscores() {
        let v = vec!["hello__world".to_string(), "rust".to_string()];
        let actual = path_string(&v).expect("Bad path item");
        assert_eq!(actual, "hello____world_rust");
    }

    #[test]
    fn test_path_string_just_underscore() {
        let v = vec!["hello".to_string(), "_".to_string(), "world".to_string()];
        let actual = path_string(&v).expect("Bad path item");
        assert_eq!(actual, "hello____world");
    }

    #[test]
    fn test_string_path_no_underscores() {
        let s = "hello";
        assert_eq!(
            string_path(s),
            vec!["hello"]
        );
    }

    #[test]
    fn test_string_path_simple_split() {
        let s = "hello_world_rust";
        assert_eq!(
            string_path(s),
            vec!["hello", "world", "rust"]
        );
    }

    #[test]
    fn test_string_path_single_escaped_underscore() {
        let s = "hello_my__world_rust";
        assert_eq!(
            string_path(s),
            vec!["hello", "my_world", "rust"]
        );
    }

    #[test]
    fn test_string_path_multiple_escaped_underscores_in_one_element() {
        let s = "hello_my__big__world_rust";
        assert_eq!(
            string_path(s),
            vec!["hello", "my_big_world", "rust"]
        );
    }

    #[test]
    fn test_string_path_escaped_underscores_in_multiple_elements() {
        let s = "hello__there_my__world_rust__lang";
        assert_eq!(
            string_path(s),
            vec!["hello_there", "my_world", "rust_lang"]
        );
    }

    #[test]
    fn test_string_path_empty_string() {
        let s = "";
        assert_eq!(
            string_path(s),
            vec![""]
        );
    }

    #[test]
    fn test_string_path_only_escaped_underscore() {
        let s = "__";
        assert_eq!(
            string_path(s),
            vec!["_"]
        );
    }

    #[test]
    fn test_string_path_only_separator() {
        let s = "_";
        assert_eq!(
            string_path(s),
            vec!["", ""]
        );
    }

    #[test]
    fn test_string_path_four_consecutive_underscores() {
        let s = "hello____world";
        assert_eq!(
            string_path(s),
            vec!["hello__world"]
        );
    }

    #[test]
    fn test_string_path_six_consecutive_underscores() {
        let s = "hello______world";
        assert_eq!(
            string_path(s),
            vec!["hello___world"]
        );
    }

    #[test]
    fn test_string_path_escaped_underscore_at_start() {
        let s = "__hello_world";
        assert_eq!(
            string_path(s),
            vec!["_hello", "world"]
        );
    }

    #[test]
    fn test_string_path_escaped_underscore_at_end() {
        let s = "hello_world__";
        assert_eq!(
            string_path(s),
            vec!["hello", "world_"]
        );
    }

    /* FIXME: Invalid input
    #[test]
    fn test_string_path_separator_then_escaped() {
        let s = "hello___world";
        assert_eq!(
            string_path(s),
            vec!["hello", "_", "world"]
        );
    }
    */

    #[test]
    fn test_string_path_empty_elements() {
        let s = "hello__world";
        assert_eq!(
            string_path(s),
            vec!["hello_world"]
        );
    }

    #[test]
    fn test_string_path_multiple_separators() {
        let s = "a_b_c_d";
        assert_eq!(
            string_path(s),
            vec!["a", "b", "c", "d"]
        );
    }

    #[test]
    fn test_string_path_leading_separator() {
        let s = "_hello_world";
        assert_eq!(
            string_path(s),
            vec!["", "hello", "world"]
        );
    }

    #[test]
    fn test_string_path_trailing_separator() {
        let s = "hello_world_";
        assert_eq!(
            string_path(s),
            vec!["hello", "world", ""]
        );
    }

    #[test]
    fn test_string_path_both_leading_and_trailing_separators() {
        let s = "_hello_world_";
        assert_eq!(
            string_path(s),
            vec!["", "hello", "world", ""]
        );
    }

    #[test]
    fn test_string_path_complex_pattern() {
        let s = "a__b_c____d_e__f__g";
        assert_eq!(
            string_path(s),
            vec!["a_b", "c__d", "e_f_g"]
        );
    }

    /* FIXME: Invalid input
    #[test]
    fn test_string_path_only_underscores_odd() {
        let s = "___";
println!("string_path {:?}", string_path(s));
        assert_eq!(
            string_path(s),
            vec!["", "_", ""]
        );
    }
    */

    #[test]
    fn test_string_path_only_underscores_even() {
        let s = "____";
        assert_eq!(
            string_path(s),
            vec!["__"]
        );
    }

    // Roundtrip tests - verify join and split are inverses
    #[test]
    fn test_string_path_roundtrip_simple() {
        let original = vec![
            "hello".to_string(),
            "world".to_string(),
            "rust".to_string(),
        ];

        let joined = path_string(&original).expect("Bad path item");
        let split = string_path(&joined);

        assert_eq!(original, split);
    }

    #[test]
    fn test_string_path_roundtrip_with_underscores() {
        let original = vec![
            "hello".to_string(),
            "my_world".to_string(),
            "rust_lang".to_string(),
        ];

        let joined = path_string(&original).expect("Bad path item");
        let split = string_path(&joined);

        assert_eq!(original, split);
    }

    /* FIXME: Invalid input
    #[test]
    fn test_string_path_roundtrip_complex() {
        let original = vec![
            "a_b_c".to_string(),
            "d__e".to_string(),
            "f".to_string(),
            "_".to_string(),
            "g__h__i".to_string(),
        ];

        let joined = path_string(&original).expect("Bad path item");
        let split = string_path(&joined);

        assert_eq!(original, split);
    }
    */

    #[test]
    fn test_string_path_roundtrip_single_element() {
        let original = vec!["hello_world".to_string()];

        let joined = path_string(&original).expect("Bad path item");
        let split = string_path(&joined);

        assert_eq!(original, split);
    }

    /* FIXME: Invalid input
    #[test]
    fn test_string_path_roundtrip_empty_strings() {
        let original = vec![
            "".to_string(),
            "hello".to_string(),
            "".to_string(),
            "world".to_string(),
            "".to_string(),
        ];

        let joined = path_string(&original).expect("Bad path item");
        let split = string_path(&joined);

        assert_eq!(original, split);
    }
    */

    /* FIXME: Invalid input
    #[test]
    fn test_string_path_roundtrip_all_underscores() {
        let original = vec![
            "_".to_string(),
            "__".to_string(),
            "___".to_string(),
        ];

        let joined = path_string(&original).expect("Bad path item");
        let split = string_path(&joined);

        assert_eq!(original, split);
    }
    */

    #[test]
    fn test_string_path_unicode_characters() {
        let s = "hello_世界_rust";
        assert_eq!(
            string_path(s),
            vec!["hello", "世界", "rust"]
        );
    }

    #[test]
    fn test_string_path_unicode_with_escaped_underscores() {
        let s = "hello__世界_rust__语言";
        assert_eq!(
            string_path(s),
            vec!["hello_世界", "rust_语言"]
        );
    }

    #[test]
    fn test_string_path_special_characters() {
        let s = "hello-world_foo@bar_baz#qux";
        assert_eq!(
            string_path(s),
            vec!["hello-world", "foo@bar", "baz#qux"]
        );
    }
}
