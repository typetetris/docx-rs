use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for TableRow {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut cells = vec![];
        let mut property = None;
        loop {
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    ignore::ignore_element(e.clone(), XMLElement::TableRowPropertyChange, r);

                    match e {
                        XMLElement::TableCell => {
                            cells.push(TableCell::read(r, &attributes)?);
                            continue;
                        }
                        XMLElement::TableRowProperty => {
                            property = Some(TableRowProperty::read(r, &attributes)?);
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableRow {
                        let mut row = TableRow::new(cells);

                        if let Some(property) = property {
                            row.property = property;
                        }

                        return Ok(row);
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
    use crate::BuildXML;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_header_and_rebuild() {
        let xml = r#"<w:tr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:trPr>
        <w:tblHeader />
    </w:trPr>
    <w:tc />
</w:tr>"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let row = TableRow::read(&mut parser, &[]).unwrap();

        assert_eq!(row, TableRow::new(vec![TableCell::new()]).header());

        let rebuilt = String::from_utf8(row.build()).unwrap();
        assert!(rebuilt.contains("<w:tblHeader />"));
    }

    #[test]
    fn test_read_disabled_header() {
        let xml = r#"<w:tr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:trPr>
        <w:tblHeader w:val="false" />
    </w:trPr>
    <w:tc />
</w:tr>"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let row = TableRow::read(&mut parser, &[]).unwrap();

        assert_eq!(row, TableRow::new(vec![TableCell::new()]));
    }

    #[test]
    fn test_read_cant_split_and_rebuild() {
        let xml = r#"<w:tr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:trPr>
        <w:cantSplit />
    </w:trPr>
    <w:tc />
</w:tr>"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let row = TableRow::read(&mut parser, &[]).unwrap();

        assert_eq!(row, TableRow::new(vec![TableCell::new()]).cant_split());

        let rebuilt = String::from_utf8(row.build()).unwrap();
        assert!(rebuilt.contains("<w:cantSplit />"));
    }

    #[test]
    fn test_read_disabled_cant_split() {
        let xml = r#"<w:tr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:trPr>
        <w:cantSplit w:val="false" />
    </w:trPr>
    <w:tc />
</w:tr>"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let row = TableRow::read(&mut parser, &[]).unwrap();

        assert_eq!(row, TableRow::new(vec![TableCell::new()]));
    }
}
