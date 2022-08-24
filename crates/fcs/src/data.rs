use std::{io, mem};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub enum Byteord {
    LittleEndian(String),
    BigEndian(String),
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Double,
}

#[derive(Debug)]
pub enum Data {
    Int(Vec<i32>),
    Float(Vec<f32>),
    Double(Vec<f64>),
}

impl<S> From<S> for Byteord
where
    S: AsRef<str>,
{
    fn from(byteord: S) -> Self {
        let byteord = byteord.as_ref();
        match byteord {
            "4,3,2,1" | "2,1" => Byteord::BigEndian(byteord.to_string()),
            "1,2,3,4" | "1,2" => Byteord::LittleEndian(byteord.to_string()),
            _ => panic!("invalid byteorder"),
        }
    }
}

impl<S> From<S> for DataType
where
    S: AsRef<str>,
{
    fn from(data_type: S) -> Self {
        let data_type = data_type.as_ref();
        match data_type {
            "I" => DataType::Int,
            "F" => DataType::Float,
            "D" => DataType::Double,
            _ => panic!("invalid format"),
        }
    }
}

impl Data {
    pub fn new(bytes: &Vec<u8>, data_type: DataType, byteord: Byteord) -> io::Result<Self> {
        match data_type {
            DataType::Int => {
                let chunks = bytes.chunks(mem::size_of::<u32>());

                let mut wrt: Vec<i32> = vec![];
                for chunk in chunks {
                    let mut rdr = std::io::Cursor::new(chunk);
                    wrt.push(match byteord {
                        Byteord::BigEndian(_) => rdr.read_i32::<BigEndian>()?,
                        Byteord::LittleEndian(_) => rdr.read_i32::<LittleEndian>()?,
                    });
                }

                Ok(Data::Int(wrt))
            }
            DataType::Float => {
                let chunks = bytes.chunks(mem::size_of::<f32>());

                let mut wrt: Vec<f32> = vec![];
                for chunk in chunks {
                    let mut rdr = std::io::Cursor::new(chunk);
                    wrt.push(match byteord {
                        Byteord::BigEndian(_) => rdr.read_f32::<BigEndian>()?,
                        Byteord::LittleEndian(_) => rdr.read_f32::<LittleEndian>()?,
                    });
                }

                Ok(Data::Float(wrt))
            }
            DataType::Double => {
                let chunks = bytes.chunks(mem::size_of::<f64>());

                let mut wrt: Vec<f64> = vec![];
                for chunk in chunks {
                    let mut rdr = std::io::Cursor::new(chunk);
                    wrt.push(match byteord {
                        Byteord::BigEndian(_) => rdr.read_f64::<BigEndian>()?,
                        Byteord::LittleEndian(_) => rdr.read_f64::<LittleEndian>()?,
                    });
                }

                Ok(Data::Double(wrt))
            }
        }
    }
}
