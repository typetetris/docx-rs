use std::io::Read;
use std::str::FromStr;

use crate::HeightRule;

use super::attributes::*;
use super::*;

impl ElementReader for TableRowProperty {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut property = TableRowProperty::new();
        loop {
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    ignore::ignore_element(e.clone(), XMLElement::TableRowPropertyChange, r);

                    match e {
                        XMLElement::GridAfter => {
                            if let Some(v) = read_val(&attributes) {
                                property = property.grid_after(u32::from_str(&v)?);
                            }
                        }
                        XMLElement::WidthAfter => {
                            if let Ok(v) = read_width(&attributes) {
                                property = property.width_after(v.0 as f32);
                            }
                        }
                        XMLElement::GridBefore => {
                            if let Some(v) = read_val(&attributes) {
                                property = property.grid_before(u32::from_str(&v)?);
                            }
                        }
                        XMLElement::WidthBefore => {
                            if let Ok(v) = read_width(&attributes) {
                                property = property.width_before(v.0 as f32);
                            }
                        }
                        XMLElement::TableRowHeight => {
                            if let Some(v) = read_val(&attributes) {
                                if let Ok(h) = f32::from_str(&v) {
                                    property = property.row_height(h);
                                }
                            }

                            if let Some(v) = read(&attributes, "hRule") {
                                if let Ok(h) = HeightRule::from_str(&v) {
                                    property = property.height_rule(h);
                                }
                            }
                        }
                        XMLElement::Delete => {
                            if let Ok(d) = Delete::read(r, &attributes) {
                                property = property.delete(d);
                            }
                        }
                        XMLElement::Insert => {
                            if let Ok(i) = Insert::read(r, &attributes) {
                                property = property.insert(i);
                            }
                        }
                        XMLElement::CantSplit => {
                            if read_bool(&attributes) {
                                property = property.cant_split();
                            }
                        }
                        XMLElement::TableHeader => {
                            if read_bool(&attributes) {
                                property = property.header();
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableRowProperty {
                        return Ok(property);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_all_supported_properties() {
        let xml = r#"<w:trPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:gridAfter w:val="2" />
    <w:wAfter w:w="120" w:type="dxa" />
    <w:gridBefore w:val="1" />
    <w:wBefore w:w="80" w:type="dxa" />
    <w:trHeight w:val="240" w:hRule="exact" />
    <w:cantSplit />
    <w:tblHeader />
</w:trPr>"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let property = TableRowProperty::read(&mut parser, &[]).unwrap();

        assert_eq!(
            property,
            TableRowProperty::new()
                .grid_after(2)
                .width_after(120.0)
                .grid_before(1)
                .width_before(80.0)
                .row_height(240.0)
                .height_rule(HeightRule::Exact)
                .cant_split()
                .header()
        );
    }

    #[test]
    fn test_read_disabled_on_off_properties() {
        let xml = r#"<w:trPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:cantSplit w:val="false" />
    <w:tblHeader w:val="off" />
</w:trPr>"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let property = TableRowProperty::read(&mut parser, &[]).unwrap();

        assert_eq!(property, TableRowProperty::new());
    }
}
