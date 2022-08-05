use std::{collections::HashMap, mem, path::PathBuf};

use async_std::{
    fs::File,
    io::{BufReader, Cursor, SeekFrom},
    prelude::*,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::error::FlowError;

#[derive(Debug)]
pub enum ByteOrd {
    LowerThen(String),
    GreaterThen(String),
    Default(String),
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Double,
    String,
}

#[derive(Debug)]
pub struct FlowIo(FlowFile, Option<FlowHeader>);

#[derive(Debug)]
pub struct FlowData {
    raw: Vec<u8>,
    data_type: DataType,
    byte_ord: ByteOrd,
}

#[derive(Debug)]
pub struct FlowFile(File);

#[derive(Debug, Clone)]
pub struct FlowHeader {
    version: f64,
    text_start: u64,
    text_end: u64,
    data_start: u64,
    data_end: u64,
    analysis_start: Option<u64>,
    analysis_end: Option<u64>,
}

impl From<&String> for ByteOrd {
    fn from(byte_ord: &String) -> Self {
        match byte_ord.as_str() {
            "4,3,2,1" | "2,1" => ByteOrd::GreaterThen(byte_ord.to_string()),
            "1,2,3,4" | "1,2" => ByteOrd::LowerThen(byte_ord.to_string()),
            _ => ByteOrd::Default(byte_ord.to_string()),
        }
    }
}

impl From<&String> for DataType {
    fn from(data_type: &String) -> Self {
        match data_type.as_str() {
            "I" => DataType::Int,
            "F" => DataType::Float,
            "D" => DataType::Double,
            _ => DataType::String,
        }
    }
}

impl FlowIo {
    pub async fn new(path: PathBuf) -> Result<FlowIo, FlowError> {
        Ok(FlowIo(FlowFile::new(path).await?, None))
    }

    pub async fn header(&mut self) -> Result<FlowHeader, FlowError> {
        if let Some(h) = self.1.clone() {
            return Ok(h);
        }

        let header = FlowHeader::new(&mut self.0, 0).await?;
        self.1 = Some(header.clone());

        Ok(header)
    }

    pub async fn text(&mut self) -> Result<HashMap<String, String>, FlowError> {
        let header = self.header().await?;

        let text = {
            let mut rdr = Cursor::new(
                self.0
                    .read_bytes(0, header.text_start, header.text_end)
                    .await?,
            );
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            buf
        };

        Ok(FlowIo::parse_pairs(text))
    }

    pub async fn analysis(&mut self) -> Result<Option<HashMap<String, String>>, FlowError> {
        let header = self.header().await?;

        if let None = header.analysis_start {
            return Ok(None);
        } else if let None = header.analysis_end {
            return Ok(None);
        }

        let text = {
            let mut rdr = Cursor::new(
                self.0
                    .read_bytes(
                        0,
                        header.analysis_start.unwrap(),
                        header.analysis_end.unwrap(),
                    )
                    .await?,
            );
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            buf
        };

        Ok(Some(FlowIo::parse_pairs(text)))
    }

    pub async fn data(&mut self) -> Result<FlowData, FlowError> {
        let header = self.header().await?;
        let text = self.text().await?;

        let data_type: DataType = text.get("$DATATYPE").expect("$DATATYPE is invalid").into();
        // let mode = text.get("$MODE").expect("$MODE is invalid");

        let byte_ord: ByteOrd = text.get("$BYTEORD").expect("$BYTEORD is invalid").into();

        let par: usize = text.get("$PAR").unwrap().parse()?;
        let mut bit_width: Vec<usize> = vec![];
        let mut data_range: Vec<usize> = vec![];

        for index in 1..par + 1 {
            bit_width.push(text.get(&format!("$P{}B", index)).unwrap().parse()?);
            data_range.push(text.get(&format!("$P{}R", index)).unwrap().parse()?);
        }

        Ok(FlowData {
            raw: self
                .0
                .read_bytes(0, header.data_start, header.data_end)
                .await?,
            byte_ord,
            data_type,
        })

        // match data_type {
        //     DataType::Int => todo!(),
        //     DataType::Float => {
        //         let bytes = self
        //             .0
        //             .read_bytes(0, header.data_start, header.data_end)
        //             .await?;
        //
        //         let chunks = bytes.chunks(mem::size_of::<f32>());
        //
        //         let mut wrt: Vec<f32> = vec![];
        //         for chunk in chunks {
        //             let mut rdr = std::io::Cursor::new(chunk);
        //             wrt.push(match byte_ord {
        //                 ByteOrd::GreaterThen(_) => rdr.read_f32::<BigEndian>().unwrap(),
        //                 ByteOrd::LowerThen(_) => rdr.read_f32::<LittleEndian>().unwrap(),
        //                 _ => todo!(),
        //             });
        //         }
        //
        //         FlowData {
        //             row: wrt
        //         }
        //     }
        //     DataType::Double => {
        //         let bytes = self
        //             .0
        //             .read_bytes(0, header.data_start, header.data_end)
        //             .await?;
        //
        //         let chunks = bytes.chunks(mem::size_of::<f64>());
        //
        //         let mut wrt: Vec<f64> = vec![];
        //         for chunk in chunks {
        //             let mut rdr = std::io::Cursor::new(chunk);
        //             wrt.push(match byte_ord {
        //                 ByteOrd::GreaterThen(_) => rdr.read_f64::<BigEndian>().unwrap(),
        //                 ByteOrd::LowerThen(_) => rdr.read_f64::<LittleEndian>().unwrap(),
        //                 _ => todo!(),
        //             });
        //         }
        //     }
        //     DataType::String => todo!(),
        // }
        //
        // println!(
        //     "{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        //     data_type, mode, byte_ord, bit_width, data_range
        // );
        //
        // Ok(String::new())
    }

    fn parse_pairs(text: String) -> HashMap<String, String> {
        text.trim_start_matches("\u{c}")
            .trim_end_matches("\u{c}")
            .split("\u{c}")
            .collect::<Vec<&str>>()
            .chunks(2)
            .fold(HashMap::new(), |mut result, chunk| {
                if chunk.len() != 2 {
                    return result;
                }

                result.insert(String::from(chunk[0]), String::from(chunk[1]));

                result
            })
    }
}

impl FlowData {
    pub fn parse_float(&self) -> Result<Vec<f32>, FlowError> {
        if self.data_type != DataType::Float {
            return Err(FlowError::InvalidDataType);
        }

        let chunks = self.raw.chunks(mem::size_of::<f32>());

        let mut wrt: Vec<f32> = vec![];
        for chunk in chunks {
            let mut rdr = std::io::Cursor::new(chunk);
            wrt.push(match self.byte_ord {
                ByteOrd::GreaterThen(_) => rdr.read_f32::<BigEndian>().unwrap(),
                ByteOrd::LowerThen(_) => rdr.read_f32::<LittleEndian>().unwrap(),
                _ => todo!(),
            });
        }

        Ok(wrt)
    }

    pub fn parse_double(&self) -> Result<Vec<f64>, FlowError> {
        if self.data_type != DataType::Double {
            return Err(FlowError::InvalidDataType);
        }

        let chunks = self.raw.chunks(mem::size_of::<f64>());

        let mut wrt: Vec<f64> = vec![];
        for chunk in chunks {
            let mut rdr = std::io::Cursor::new(chunk);
            wrt.push(match self.byte_ord {
                ByteOrd::GreaterThen(_) => rdr.read_f64::<BigEndian>().unwrap(),
                ByteOrd::LowerThen(_) => rdr.read_f64::<LittleEndian>().unwrap(),
                _ => todo!(),
            });
        }

        Ok(wrt)
    }
}

impl FlowFile {
    pub async fn new(path: PathBuf) -> Result<FlowFile, FlowError> {
        Ok(FlowFile(File::open(path).await?))
    }

    /// Read in bytes from start to stop inclusive.
    async fn read_bytes(
        &mut self,
        offset: u64,
        start: u64,
        stop: u64,
    ) -> Result<Vec<u8>, FlowError> {
        debug!("read_bytes({}, {}, {})", offset, start, stop);
        let reference = &self.0;
        let mut f = BufReader::new(reference);
        let first_byte = offset + start;
        let c = f.seek(SeekFrom::Start(first_byte)).await?;
        debug!("Cursor set to {}", c);

        let bytes_to_read = stop - first_byte + 1;
        let mut result = vec![0; bytes_to_read as usize];

        let mut take = f.take(bytes_to_read);
        take.read(&mut result).await?;
        debug!("Result is {:?}", result);

        Ok(result)
    }
}

impl FlowHeader {
    /// Parse the FlowIo FCS file at the offset (supporting multiple data segments in a file.
    pub async fn new(file: &mut FlowFile, offset: u64) -> Result<FlowHeader, FlowError> {
        let version: f64 = {
            let mut rdr = Cursor::new(file.read_bytes(offset, 3, 5).await?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            buf.parse()?
        };

        let text_start: u64 = {
            let mut rdr = Cursor::new(file.read_bytes(offset, 10, 17).await?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            buf.trim().parse()?
        };

        let text_end: u64 = {
            let mut rdr = Cursor::new(file.read_bytes(offset, 18, 25).await?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            buf.trim().parse()?
        };

        let data_start: u64 = {
            let mut rdr = Cursor::new(file.read_bytes(offset, 26, 33).await?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            buf.trim().parse()?
        };

        let data_end: u64 = {
            let mut rdr = Cursor::new(file.read_bytes(offset, 34, 41).await?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            buf.trim().parse()?
        };

        let analysis_start: Option<u64> = {
            let mut rdr = Cursor::new(file.read_bytes(offset, 42, 49).await?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            let analysis_start = buf.trim().parse::<u64>()?;
            if analysis_start == 0 {
                None
            } else {
                Some(analysis_start)
            }
        };

        let analysis_end: Option<u64> = {
            let mut rdr = Cursor::new(file.read_bytes(offset, 50, 57).await?);
            let mut buf = String::new();
            rdr.read_to_string(&mut buf).await?;
            let analysis_end = buf.trim().parse::<u64>()?;
            if analysis_end == 0 {
                None
            } else {
                Some(analysis_end)
            }
        };

        Ok(FlowHeader {
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
