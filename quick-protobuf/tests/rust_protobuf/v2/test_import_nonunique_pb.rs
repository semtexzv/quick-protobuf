use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestImportNonunque {
    pub n1: Option<nonunique_1::Nonunique>,
    pub n2: Option<nonunique_2::Nonunique>,
}

impl<'a> MessageRead<'a> for TestImportNonunque {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.n1 = Some(r.read_message::<nonunique_1::Nonunique>(bytes)?),
                Ok(18) => msg.n2 = Some(r.read_message::<nonunique_2::Nonunique>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestImportNonunque {
    fn get_size(&self) -> usize {
        0 + self
            .n1
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
            + self
                .n2
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.n1 {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        if let Some(ref s) = self.n2 {
            w.write_with_tag(18, |w| w.write_message(s))?;
        }
        Ok(())
    }
}
