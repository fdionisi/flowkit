use chrono::NaiveDateTime;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

use super::serialization;

/// Document representation of drug administration. Single document instance represents one event.
pub struct Drug {
    /// name of therapy/drug
    pub name: String,
    /// date that therapy/drug started
    pub init_at: Option<NaiveDateTime>,
    /// date that therapy/drug started
    pub end_at: Option<NaiveDateTime>,
}

impl Drug {
    pub fn to_flatbuf<'fbb>(
        &self,
        builder: &mut FlatBufferBuilder<'fbb>,
    ) -> WIPOffset<serialization::Drug<'fbb>> {
        use super::serialization::{Drug as DrugFlatBuff, DrugArgs};

        let name = Some(builder.create_string(&self.name));
        let init_at = self
            .init_at
            .map(|init_at| init_at.timestamp())
            .unwrap_or(-1);
        let end_at = self.end_at.map(|end_at| end_at.timestamp()).unwrap_or(-1);

        DrugFlatBuff::create(
            builder,
            &DrugArgs {
                name,
                init_at,
                end_at,
            },
        )
    }

    pub fn from_flatbuf(message: &serialization::Drug) -> Self {
        let init_at = message.init_at();
        let end_at = message.end_at();
        Drug {
            name: message
                .name()
                .map(|id| id.to_string())
                .expect("Customer must have an id"),
            init_at: if init_at > 0 {
                Some(NaiveDateTime::from_timestamp(init_at, 0))
            } else {
                None
            },
            end_at: if end_at > 0 {
                Some(NaiveDateTime::from_timestamp(end_at, 0))
            } else {
                None
            },
        }
    }
}
