use std::{
    collections::BTreeMap,
    io::{self, Cursor, Read},
    ops::Range,
};

use crate::{
    data::{Byteord, DataType},
    keywords::{OptionalKeyword, RequiredKeyword},
};

#[derive(Debug)]
pub struct Text {
    pub pairs: BTreeMap<String, String>,
}

#[derive(Debug)]
pub struct Parameter {
    /// index
    pub index: u32,
    /// $PnB
    pub bits: u32,
    /// $PnE - should not be optional
    pub amplyficatioin_type: Option<String>,
    /// $PnN
    pub short_name: String,
    /// $PnR
    pub range: Range<u32>,
    /// $PnD
    pub visualization_scale: Option<u32>,
    /// $PnF
    pub optical_filter_name: Option<String>,
    /// $PnG
    pub amplifier_gain: Option<u32>,
    /// $PnL
    pub excitation_wavelengths: Option<String>,
    /// $PnO
    pub excitation_power: Option<String>,
    /// $PnP
    pub emitted_light_collected: Option<String>,
    /// $PnS
    pub name: Option<String>,
    /// $PnT
    pub detector_type: Option<String>,
    /// $PnV
    pub detector_voltage: Option<String>,
}

#[derive(Debug)]
pub struct Gate {
    /// index
    pub index: u32,
    /// $GnS
    pub name: Option<String>,
}

// TODO(@fdionisi): add dates and times
#[derive(Debug)]
pub struct Metadata {
    /// $OP
    pub operator: Option<String>,
    /// $EXP
    pub expermiment_director: Option<String>,
    /// $FIL
    pub file_name: Option<String>,
    /// $CYT
    pub fc_name: Option<String>,
    /// $CYTSN
    pub fc_serial: Option<String>,
    /// $INST
    pub institute: Option<String>,
    /// $SRC
    pub source: Option<String>,
    /// $SYS
    pub system: Option<String>,
}

impl Text {
    pub fn new(bytes: &Vec<u8>) -> io::Result<Self> {
        let mut rdr = Cursor::new(bytes);
        let mut buf = String::new();
        rdr.read_to_string(&mut buf)?;

        Ok(Text {
            pairs: parse_pairs(&buf),
        })
    }

    pub fn get<K>(&self, key: K) -> Option<&String>
    where
        K: ToString,
    {
        self.pairs.get(&key.to_string())
    }

    pub fn metadata(&self) -> Metadata {
        Metadata {
            operator: self.get(OptionalKeyword::Op).map(|s| s.to_owned()),
            expermiment_director: self.get(OptionalKeyword::Exp).map(|s| s.to_owned()),
            file_name: self.get(OptionalKeyword::Fil).map(|s| s.to_owned()),
            fc_name: self.get(OptionalKeyword::Cyt).map(|s| s.to_owned()),
            fc_serial: self.get(OptionalKeyword::Cytsn).map(|s| s.to_owned()),
            institute: self.get(OptionalKeyword::Inst).map(|s| s.to_owned()),
            source: self.get(OptionalKeyword::Src).map(|s| s.to_owned()),
            system: self.get(OptionalKeyword::Sys).map(|s| s.to_owned()),
        }
    }

    pub fn total_events(&self) -> u32 {
        self.get(RequiredKeyword::Tot)
            .expect("$TOT is invalid")
            .parse()
            .expect("$TOT to be valid number")
    }

    pub fn gates_number(&self) -> Option<u32> {
        self.get(OptionalKeyword::Gate)
            .map(|g| g.parse().expect("$GATE to be valid number"))
    }

    pub fn gates(&self) -> Vec<Gate> {
        (1..self.gates_number().unwrap_or(0) + 1)
            .map(|index| Gate {
                index,
                name: self
                    .pairs
                    .get(&format!("$G{}S", index))
                    .map(|s| s.to_owned()),
            })
            .collect()
    }

    /// $PAR: Number of parameters in an event.
    pub fn parameters_number(&self) -> u32 {
        self.get(RequiredKeyword::Par)
            .expect("$PAR is invalid")
            .parse()
            .expect("$PAR to be valid number")
    }

    pub fn parameters(&self) -> Vec<Parameter> {
        (1..(self.parameters_number() + 1))
            .map(|i| Parameter {
                index: i,
                bits: self
                    .get(&format!("$P{}B", i))
                    .expect("to have value")
                    .parse()
                    .expect("to be u32"),
                amplyficatioin_type: self.pairs.get(&format!("$P{}E", i)).map(|s| s.to_owned()),
                short_name: self
                    .get(&format!("$P{}N", i))
                    .expect("to have value")
                    .to_owned(),
                range: 0..self
                    .get(&format!("$P{}R", i))
                    .expect("to have value")
                    .parse()
                    .expect("to be a u32"),
                visualization_scale: self
                    .get(&format!("$P{}R", i))
                    .map(|s| s.parse().expect("to be a u32")),
                optical_filter_name: self.pairs.get(&format!("$P{}F", i)).map(|s| s.to_owned()),
                amplifier_gain: self
                    .get(&format!("$P{}G", i))
                    .map(|s| s.parse().expect("to be u32")),
                excitation_wavelengths: self.get(&format!("$P{}L", i)).map(|s| s.to_owned()),
                excitation_power: self.get(&format!("$P{}O", i)).map(|s| s.to_owned()),
                emitted_light_collected: self.get(&format!("$P{}P", i)).map(|s| s.to_owned()),
                name: self.get(&format!("$P{}S", i)).map(|s| s.to_owned()),
                detector_type: self.get(&format!("$P{}T", i)).map(|s| s.to_owned()),
                detector_voltage: self.get(&format!("$P{}V", i)).map(|s| s.to_owned()),
            })
            .collect()
    }

    /// Type of data in DATA segment (ASCII, integer, floating point).
    pub fn data_type(&self) -> DataType {
        self.get(RequiredKeyword::DataType)
            .expect("$DATATYPE is invalid")
            .into()
    }

    /// Byte order for data acquisition computer.
    pub fn byteord(&self) -> Byteord {
        self.get(RequiredKeyword::Byteord)
            .expect("$BYTEORD is invalid")
            .into()
    }
}

fn parse_pairs(text: &str) -> BTreeMap<String, String> {
    BTreeMap::from_iter(
        text.trim_start_matches("\u{005C}")
            .trim_end_matches("\u{005C}")
            .split("\u{005C}")
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|chunk| (chunk[0].into(), chunk[1].into())),
    )
}
