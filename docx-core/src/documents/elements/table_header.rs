use serde::{Serialize, Serializer};
use std::io::Write;

use crate::{xml_builder::XMLBuilder, BuildXML};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableHeader {}
impl BuildXML for TableHeader {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).table_header()?.into_inner()
    }
}

impl Serialize for TableHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("tblHeader")
    }
}
