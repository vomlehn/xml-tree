use lazy_static::lazy_static;

use crate::xml_schema::{SchemaElement, XmlSchema};

lazy_static! {
    pub static ref XSD_SCHEMA: XmlSchema =
        XmlSchema::new("XsdSchema", SchemaElement::new("XTCE", vec!(
            SchemaElement::new("schema", vec!(
                SchemaElement::new("schema", vec!(
                    SchemaElement::new("import", vec!()),
                    SchemaElement::new("annotation", vec!(
                        SchemaElement::new("documentation", vec!()),
                    )),
                    SchemaElement::new("element", vec!(
                        SchemaElement::new("annotation", vec!(
                            SchemaElement::new("documentation", vec!()),
                        )),
                        SchemaElement::new("key", vec!(
                            SchemaElement::new("annotation", vec!(
                                SchemaElement::new("key", vec!()),
                            )),
                            SchemaElement::new("selector", vec!()),
                            SchemaElement::new("field", vec!()),
                        )),
                    )),
                )),
/*
                SchemaElement::new("attribute", vec!(
                    SchemaElement::new("annotation", vec!(
                        SchemaElement::new("documentation", vec!()),
                    )),
                )),
*/
                SchemaElement::new("complexType", vec!(
                    SchemaElement::new("annotation", vec!(
                        SchemaElement::new("documentation", vec!()),
                    )),
                    SchemaElement::new("simpleContent", vec!(
                        SchemaElement::new("extension", vec!(
                            SchemaElement::new("attribute", vec!()),
                        )),
                    )),
                    SchemaElement::new("complexContent", vec!(
                        SchemaElement::new("extension", vec!(
                            SchemaElement::new("sequence", vec!(
                                SchemaElement::new("annotation", vec!(
                                    SchemaElement::new("documentation", vec!()),
                                    SchemaElement::new("appinfo", vec!()),
                                )),
                                SchemaElement::new("element", vec!(
                                    SchemaElement::new("annotation", vec!(
                                        SchemaElement::new("documentation", vec!()),
                                    )),
                                )),
                            )),
                        )),
                    )),
                    SchemaElement::new("choice", vec!(),),
                )),
                SchemaElement::new("simpleType", vec!(
                    SchemaElement::new("restriction", vec!(
                        SchemaElement::new("maxInclusive", vec!()),
                        SchemaElement::new("minInclusive", vec!()),
                        SchemaElement::new("pattern", vec!()),
                    )),
                    SchemaElement::new("union", vec!()),
                )),
                SchemaElement::new("enumeration", vec!(
                    SchemaElement::new("annotation", vec!(
                        SchemaElement::new("documentation", vec!()),
                    )),
                )),
            )),
        )),
    );

}
