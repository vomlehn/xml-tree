/*
 * Definitions for XSD-specific schemas
 */

use std::marker::PhantomData;

struct Dummy<'a> {
    _s:  &'a String
}

pub struct XsdSchema<'a> {
    marker1:    PhantomData<Dummy<'a>>
}

impl<'a> XsdSchema<'a> {
    pub fn new() -> XsdSchema<'a> {
        XsdSchema {
            marker1:    PhantomData,
        }
    }
}
