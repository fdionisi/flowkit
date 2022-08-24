use std::io::{self, Cursor, Read};

use crate::traits::ByteRead;

#[derive(Debug)]
pub struct Header {
    pub version: f64,
    pub text_start: u64,
    pub text_end: u64,
    pub data_start: u64,
    pub data_end: u64,
    pub analysis_start: Option<u64>,
    pub analysis_end: Option<u64>,
}

impl Header {
    /// Parse the FlowIo FCS file at the offset (supporting multiple data segments in a file.
    pub(crate) fn new<R>(reader: &mut R) -> io::Result<Self>
    where
        R: ByteRead,
    {
        let offset: u64 = 0;
        let version: f64 = {
            let mut rdr = Cursor::new(reader.read_bytes(offset, 3, 5)?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf)?;
            buf.parse().expect("version to be parsed")
        };

        let text_start: u64 = {
            let mut rdr = Cursor::new(reader.read_bytes(offset, 10, 17)?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf)?;
            buf.trim().parse().expect("text_start to be parsed")
        };

        let text_end: u64 = {
            let mut rdr = Cursor::new(reader.read_bytes(offset, 18, 25)?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf)?;
            buf.trim().parse().expect("text_end to be parsed")
        };

        let data_start: u64 = {
            let mut rdr = Cursor::new(reader.read_bytes(offset, 26, 33)?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf)?;
            buf.trim().parse().expect("data_start to be parsed")
        };

        let data_end: u64 = {
            let mut rdr = Cursor::new(reader.read_bytes(offset, 34, 41)?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf)?;
            buf.trim().parse().expect("data_end to be parsed")
        };

        let analysis_start: Option<u64> = {
            let mut rdr = Cursor::new(reader.read_bytes(offset, 42, 49)?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf)?;
            let analysis_start: u64 = buf.trim().parse().expect("analysis_start to be parsed");
            if analysis_start == 0 {
                None
            } else {
                Some(analysis_start)
            }
        };

        let analysis_end: Option<u64> = {
            let mut rdr = Cursor::new(reader.read_bytes(offset, 50, 57)?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf)?;
            let analysis_end: u64 = buf.trim().parse().expect("analysis_end to be parsed");
            if analysis_end == 0 {
                None
            } else {
                Some(analysis_end)
            }
        };

        Ok(Header {
            version,
            text_start,
            text_end,
            data_start,
            data_end,
            analysis_start,
            analysis_end,
        })
    }
}
