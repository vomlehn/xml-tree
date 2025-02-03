/*
 * This is the parsed scheme for XSD files.
 */
use lazy_static::lazy_static;

/*
lazy_static! {
    pub static ref XSD_SCHEMA: XmlSchema<'static> =
        XmlSchema::new("XsdSchema", DirectElement::new("schema", vec!(
            DirectElement::new("import", vec!()),
            DirectElement::new("annotation", vec!(
                DirectElement::new("documentation", vec!()),
            )),
            DirectElement::new("element", vec!(
                DirectElement::new("annotation", vec!(
                    DirectElement::new("documentation", vec!()),
                )),
                DirectElement::new("key", vec!(
                    DirectElement::new("annotation", vec!(
                        DirectElement::new("documentation", vec!()),
                    )),
                    DirectElement::new("selector", vec!()),
                    DirectElement::new("field", vec!()),
                )),
            )),
            DirectElement::new("complexType", vec!(
                DirectElement::new("annotation", vec!(
                    DirectElement::new("documentation", vec!()),
                    DirectElement::new("appinfo", vec!()),
                )),
                DirectElement::new("attribute", vec!(
                    DirectElement::new("annotation", vec!(
                        DirectElement::new("documentation", vec!()),
                        DirectElement::new("appinfo", vec!()),
                    )),
                    DirectElement::new("simpleType", vec!(
                        DirectElement::new("restriction", vec!(
                            DirectElement::new("enumeration", vec!(
                                DirectElement::new("annotation", vec!(
                                    DirectElement::new("documentation", vec!()),
                                )),
                            )),
                        )),
                    )),
                )),
                DirectElement::new("choice", vec!(
                    DirectElement::new("annotation", vec!(
                        DirectElement::new("documentation", vec!()),
                    )),
                    DirectElement::new("element", vec!(
                        DirectElement::new("annotation", vec!(
                            DirectElement::new("documentation", vec!()),
                        )),
                        DirectElement::new("key", vec!(
                            DirectElement::new("annotation", vec!(
                                DirectElement::new("documentation", vec!()),
                            )),
                            DirectElement::new("selector", vec!()),
                            DirectElement::new("field", vec!()),
                        )),
                    )),
                )),
                DirectElement::new("sequence", vec!(
                    DirectElement::new("annotation", vec!(
                        DirectElement::new("documentation", vec!()),
                        DirectElement::new("appinfo", vec!()),
                    )),
                    DirectElement::new("element", vec!(
                        DirectElement::new("annotation", vec!(
                            DirectElement::new("documentation", vec!()),
                            DirectElement::new("appinfo", vec!()),
                        )),
                    )),
                    DirectElement::new("choice", vec!(
                        DirectElement::new("annotation", vec!(
                            DirectElement::new("documentation", vec!()),
                        )),
                        DirectElement::new("element", vec!(
                            DirectElement::new("annotation", vec!(
                                DirectElement::new("documentation", vec!()),
                                DirectElement::new("appinfo", vec!()),
                            )),
                        )),
                    )),
                )),
                DirectElement::new("simpleContent", vec!(
                    DirectElement::new("extension", vec!(
                        DirectElement::new("attribute", vec!()),
                    )),
                )),
                DirectElement::new("sequence", vec!(
                    DirectElement::new("element", vec!(
                        DirectElement::new("annotation", vec!(
                            DirectElement::new("documentation", vec!()),
                            DirectElement::new("appinfo", vec!()),
                        )),
                    )),
                    DirectElement::new("choice", vec!(
                        DirectElement::new("annotation", vec!(
                            DirectElement::new("documentation", vec!()),
                        )),
                        DirectElement::new("element", vec!(
                            DirectElement::new("annotation", vec!(
                                DirectElement::new("documentation", vec!()),
                                DirectElement::new("appinfo", vec!()),
                            )),
                        )),
                    )),
                )),
                DirectElement::new("complexContent", vec!(
                    DirectElement::new("extension", vec!(
                        DirectElement::new("attribute", vec!(
                            DirectElement::new("annotation", vec!(
                                DirectElement::new("documentation", vec!()),
                                DirectElement::new("appinfo", vec!()),
                            )),
                        )),
                        DirectElement::new("choice", vec!(
                            DirectElement::new("annotation", vec!(
                                DirectElement::new("documentation", vec!()),
                            )),
                            DirectElement::new("element", vec!(
                                DirectElement::new("annotation", vec!(
                                    DirectElement::new("documentation", vec!()),
                                )),
                            )),
                        )),
                        DirectElement::new("sequence", vec!(
                            DirectElement::new("annotation", vec!(
                                DirectElement::new("documentation", vec!()),
                                DirectElement::new("appinfo", vec!()),
                            )),
                            DirectElement::new("choice", vec!(
                                DirectElement::new("annotation", vec!(
                                    DirectElement::new("documentation", vec!()),
                                )),
                                DirectElement::new("choice", vec!(
                                    DirectElement::new("annotation", vec!(
                                        DirectElement::new("documentation", vec!()),
                                    )),
                                    DirectElement::new("element", vec!(
                                        DirectElement::new("annotation", vec!(
                                            DirectElement::new("documentation", vec!()),
                                        )),
                                    )),
                                )),
                                DirectElement::new("element", vec!(
                                    DirectElement::new("annotation", vec!(
                                        DirectElement::new("documentation", vec!()),
                                    )),
                                )),
                            )),
                            DirectElement::new("element", vec!(
                                DirectElement::new("annotation", vec!(
                                    DirectElement::new("documentation", vec!()),
                                    DirectElement::new("appinfo", vec!()),
                                )),
                                DirectElement::new("complexType", vec!(
                                    DirectElement::new("complexContent", vec!()),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
            DirectElement::new("simpleType", vec!(
                DirectElement::new("annotation", vec!(
                    DirectElement::new("documentation", vec!()),
                )),
                DirectElement::new("enumeration", vec!(
                    DirectElement::new("annotation", vec!(
                        DirectElement::new("documentation", vec!()),
                    )),
                )),
                DirectElement::new("restriction", vec!(
                    DirectElement::new("maxInclusive", vec!()),
                    DirectElement::new("minInclusive", vec!()),
                    DirectElement::new("pattern", vec!()),
                    DirectElement::new("enumeration", vec!(
                        DirectElement::new("annotation", vec!(
                            DirectElement::new("documentation", vec!()),
                            DirectElement::new("appinfo", vec!()),
                        )),
                    )),
                )),
                DirectElement::new("union", vec!()),
            )),
        )),
    );

}
*/
