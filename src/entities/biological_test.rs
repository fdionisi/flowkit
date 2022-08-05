use chrono::NaiveDateTime;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

use super::serialization;

/// Document representation of biological test (blood pathology). Single document instance represents one test.
pub struct BiologicalTest {
    /// date that test was performed
    pub test_date: Option<NaiveDateTime>,
    /// name of pathology test
    pub test: Option<String>,
    /// value of pathology test
    pub result: Option<f64>,
    /// units reported
    pub unit: Option<String>,
    /// reported reference range
    pub ref_range: Option<String>,
    /// category of test
    pub test_category: Option<String>,
}

impl BiologicalTest {
    pub fn to_flatbuf<'fbb>(
        &self,
        builder: &mut FlatBufferBuilder<'fbb>,
    ) -> WIPOffset<serialization::BiologicalTest<'fbb>> {
        use super::serialization::{BiologicalTest as BiologicalTestFlatBuff, BiologicalTestArgs};
        let test = self.test.as_ref().map(|test| builder.create_string(&test));
        let unit = self.unit.as_ref().map(|unit| builder.create_string(&unit));
        let ref_range = self
            .ref_range
            .as_ref()
            .map(|ref_range| builder.create_string(&ref_range));
        let test_category = self
            .test_category
            .as_ref()
            .map(|test_category| builder.create_string(&test_category));

        BiologicalTestFlatBuff::create(
            builder,
            &BiologicalTestArgs {
                test_date: self
                    .test_date
                    .map(|test_date| test_date.timestamp())
                    .unwrap_or(-1),
                test,
                result: self.result.or(Some(0.0)).unwrap(),
                unit,
                ref_range,
                test_category,
            },
        )
    }

    pub fn from_flatbuf(message: &serialization::BiologicalTest) -> Self {
        let test_date = message.test_date();

        BiologicalTest {
            test_date: if test_date > 0 {
                Some(NaiveDateTime::from_timestamp(test_date, 0))
            } else {
                None
            },
            test: message.test().map(|test| test.to_string()),
            result: Some(message.result()),
            unit: message.unit().map(|unit| unit.to_string()),
            ref_range: message.ref_range().map(|ref_range| ref_range.to_string()),
            test_category: message
                .test_category()
                .map(|test_category| test_category.to_string()),
        }
    }
}
