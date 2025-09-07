use lazy_static::lazy_static;
use std::fs;
use std::fmt;
use std::sync::Mutex;

lazy_static! {
    static ref BANNER_FILE_NAME: Mutex<Option<String>> = Mutex::new(None);
}

/* FIXME: implement this
static banner: [&str] = [
    "FIXME: finish out the banner code",
    "This file was created automatically by {}. Modifications to this file will",
    "be lost when the next build is done.",
];

struct decoration {
    at_top:     &str,
    at_bottom:  &str,
    at_right:   &str,
    at_left:    &str,
    for_line:   &str,
};
*/

pub fn set_banner_file_name(name: Option<String>) {
    {
        let mut banner_file_name = BANNER_FILE_NAME.lock().unwrap();
        *banner_file_name = name;
    }
}

pub fn _write_banner_file(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let banner_file_name = BANNER_FILE_NAME.lock().unwrap().clone();

    if let Some(name) = banner_file_name {
        match fs::read_to_string(&name) {
            Ok(contents) => write!(f, "{}", contents)?,
            Err(e) => write!(f, "Error reading file '{}': {}", &name, e)?,
        }
    }

    Ok(())
}
