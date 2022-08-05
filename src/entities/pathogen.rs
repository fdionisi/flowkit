use std::convert::TryFrom;

use chrono::NaiveDateTime;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

use super::serialization;

#[derive(Clone)]
pub enum PathogenStatus {
    Unknown = 0,
    Pve = 1,
    Nve = 2,
}

#[derive(Clone)]
pub enum OrganismType {
    Unknown = 0,
    Bacteria = 1,
    Fungi = 2,
    Virus = 3,
}

/// Document representation of isolated pathogen. Single document instance represents one pathogen.
pub struct Pathogen {
    /// value of organisms gram status
    pub gram_status: Option<PathogenStatus>,
    /// value of hmbpp status
    pub hmbpp_status: Option<PathogenStatus>,
    /// value of organisms ribo status
    pub ribo_status: Option<PathogenStatus>,
    /// name of the organism
    pub org_name: Option<String>,
    /// method used to identify organism
    pub id_method: Option<String>,
    /// site of isolated organism
    pub culture_source: Option<String>,
    /// type of organism isolated
    pub organism_type: Option<OrganismType>,
    /// date that organism was reported
    pub report_date: Option<NaiveDateTime>,
    /// string value for free text notes
    pub notes: Option<String>,
}

impl TryFrom<u8> for PathogenStatus {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PathogenStatus::Unknown),
            1 => Ok(PathogenStatus::Pve),
            2 => Ok(PathogenStatus::Nve),
            _ => Err("Parse error"),
        }
    }
}

impl TryFrom<u8> for OrganismType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OrganismType::Unknown),
            1 => Ok(OrganismType::Bacteria),
            2 => Ok(OrganismType::Fungi),
            3 => Ok(OrganismType::Virus),
            _ => Err("Parse error"),
        }
    }
}

impl Pathogen {
    pub fn to_flatbuf<'fbb>(
        &self,
        builder: &mut FlatBufferBuilder<'fbb>,
    ) -> WIPOffset<serialization::Pathogen<'fbb>> {
        use super::serialization::{Pathogen as PathogenFlatBuff, PathogenArgs};
        let org_name = self
            .org_name
            .as_ref()
            .map(|org_name| builder.create_string(&org_name));
        let id_method = self
            .id_method
            .as_ref()
            .map(|id_method| builder.create_string(&id_method));
        let culture_source = self
            .culture_source
            .as_ref()
            .map(|culture_source| builder.create_string(&culture_source));
        let notes = self
            .notes
            .as_ref()
            .map(|notes| builder.create_string(&notes));

        PathogenFlatBuff::create(
            builder,
            &PathogenArgs {
                gram_status: self
                    .gram_status
                    .clone()
                    .or(Some(PathogenStatus::Unknown))
                    .unwrap() as u8,
                hmbpp_status: self
                    .hmbpp_status
                    .clone()
                    .or(Some(PathogenStatus::Unknown))
                    .unwrap() as u8,
                ribo_status: self
                    .ribo_status
                    .clone()
                    .or(Some(PathogenStatus::Unknown))
                    .unwrap() as u8,
                organism_type: self
                    .organism_type
                    .clone()
                    .or(Some(OrganismType::Unknown))
                    .unwrap() as u8,
                report_date: self
                    .report_date
                    .map(|report_date| report_date.timestamp())
                    .unwrap_or(-1),
                org_name,
                id_method,
                culture_source,
                notes,
            },
        )
    }

    pub fn from_flatbuf(message: &serialization::Pathogen) -> Self {
        let report_date = message.report_date();

        Pathogen {
            gram_status: Some(PathogenStatus::try_from(message.gram_status()).unwrap()),
            hmbpp_status: Some(PathogenStatus::try_from(message.hmbpp_status()).unwrap()),
            ribo_status: Some(PathogenStatus::try_from(message.ribo_status()).unwrap()),
            organism_type: Some(OrganismType::try_from(message.organism_type()).unwrap()),
            org_name: message.org_name().map(|id| id.to_string()),
            report_date: if report_date > 0 {
                Some(NaiveDateTime::from_timestamp(report_date, 0))
            } else {
                None
            },
            id_method: message.id_method().map(|id| id.to_string()),
            culture_source: message.culture_source().map(|id| id.to_string()),
            notes: message.notes().map(|id| id.to_string()),
        }
    }
}
