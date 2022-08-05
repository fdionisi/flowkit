use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

use chrono::NaiveDateTime;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

use super::{serialization, BiologicalTest, Drug, Pathogen};

pub type SubjectId = usize;

##[derive(Clone)]
pub enum SubjectGender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}

impl TryFrom<u8> for SubjectGender {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubjectGender::Unknown),
            1 => Ok(SubjectGender::Male),
            2 => Ok(SubjectGender::Female),
            _ => Err("Parse error"),
        }
    }
}

/// Document based representation of subject meta-data. Subjects are stored in a dynamic document,
/// meaning new properties can be added ad-hoc.
pub struct Subject {
    /// Unique identifier for subject
    pub id: SubjectId,
    /// Age of subject
    pub age: Option<u8>,
    /// Date of birth
    pub date_of_birth: Option<NaiveDateTime>,
    /// Gender of subject; 1 = Female, 0 = Male
    pub gender: Option<SubjectGender>,
    /// List of references to files associated to subject
    pub files: Vec<PathBuf>,
    /// Associated drug data
    pub drug_data: Vec<Drug>,
    /// Associated infection data
    pub infection_data: Vec<Pathogen>,
    /// Associated biological data
    pub patient_biology: Vec<BiologicalTest>,
    /// Additional notes
    pub notes: Option<String>,
}

impl Subject {
    pub fn to_flatbuf<'fbb>(
        &self,
        builder: &mut FlatBufferBuilder<'fbb>,
    ) -> WIPOffset<serialization::Subject<'fbb>> {
        use super::serialization::{Subject as SubjectFlatBuff, SubjectArgs};
        let id: u8 = self.id.try_into().unwrap();
        let age: i8 = self
            .age
            .map(|age| age.try_into().unwrap())
            .or(Some(-1))
            .unwrap();

        let files = Some(
            builder.create_vector(
                &self
                    .files
                    .iter()
                    .map(|file| builder.create_string(file.to_str().unwrap()))
                    .collect::<Vec<_>>(),
            ),
        );
        let drug_data = Some(
            builder.create_vector(
                &self
                    .drug_data
                    .iter()
                    .map(|drug| drug.to_flatbuf(&mut builder))
                    .collect::<Vec<_>>(),
            ),
        );
        let infection_data = Some(
            builder.create_vector(
                &self
                    .infection_data
                    .iter()
                    .map(|pathogen| pathogen.to_flatbuf(&mut builder))
                    .collect::<Vec<_>>(),
            ),
        );
        let patient_biology = Some(
            builder.create_vector(
                &self
                    .patient_biology
                    .iter()
                    .map(|biological_test| biological_test.to_flatbuf(&mut builder))
                    .collect::<Vec<_>>(),
            ),
        );

        let notes = self
            .notes
            .as_ref()
            .map(|notes| builder.create_string(&notes));

        SubjectFlatBuff::create(
            builder,
            &SubjectArgs {
                id,
                age,
                gender: self.gender.clone().or(Some(SubjectGender::Unknown)).unwrap() as u8,
                date_of_birth: self
                    .date_of_birth
                    .map(|date_of_birth| date_of_birth.timestamp())
                    .unwrap_or(-1),
                files,
                drug_data,
                infection_data,
                patient_biology,
                notes,
            },
        )
    }

    // pub fn from_flatbuf(message: &serialization::Subject) -> Self {
    //     let report_date = message.report_date();

    //     Subject {
    //         gram_status: Some(SubjectStatus::try_from(message.gram_status()).unwrap()),
    //         hmbpp_status: Some(SubjectStatus::try_from(message.hmbpp_status()).unwrap()),
    //         ribo_status: Some(SubjectStatus::try_from(message.ribo_status()).unwrap()),
    //         organism_type: Some(OrganismType::try_from(message.organism_type()).unwrap()),
    //         org_name: message.org_name().map(|id| id.to_string()),
    //         report_date: if report_date > 0 {
    //             Some(NaiveDateTime::from_timestamp(report_date, 0))
    //         } else {
    //             None
    //         },
    //         id_method: message.id_method().map(|id| id.to_string()),
    //         culture_source: message.culture_source().map(|id| id.to_string()),
    //         notes: message.notes().map(|id| id.to_string()),
    //     }
    // }
}
