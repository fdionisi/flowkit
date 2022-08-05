use std::{collections::HashMap, path::PathBuf};

use chrono::NaiveDateTime;

pub type PanelId = usize;

/// Defines channel/marker mapping. Each document will contain a single value for channel and a
/// single value for marker, these two values are treated as a pair within the panel.
pub struct ChannelMap {
    /// Name of channel (e.g. fluorochrome)
    pub channel: String,
    /// Name of channel (e.g. fluorochrome)
    pub marker: String,
}

/// Defines a standardised name for a channel or marker and provides method for testing if a
/// channel/marker should be associated to standard
pub struct NormalisedName {
    /// The "standard" name i.e. the nomenclature we used for a channel/marker in this panel
    pub standard: String,
    /// Regular expression used to test if a term corresponds to this standard
    pub regex_str: String,
    /// String values that have direct association to this standard (comma seperated values)
    pub permutations: String,
    /// Is the nomenclature case sensitive? This would be false for something like 'CD3' for
    /// example, where 'cd3' and 'CD3' are synonymous
    pub case_sensitive: bool,
}

/// Document representation of channel/marker definition for an experiment. A panel, once
/// associated to an experiment will standardise data upon input; when an fcs file is created in
/// the database, it will be associated to an experiment and the channel/marker definitions in the
/// fcs file will be mapped to the associated panel.
pub struct Panel {
    /// Unique identifier for the panel
    name: PanelId,
    /// List of marker names; see NormalisedName
    markers: Vec<NormalisedName>,
    /// List of channels; see NormalisedName
    channels: Vec<NormalisedName>,
    /// List of channel/marker mappings; see ChannelMap
    mappings: Vec<ChannelMap>,
    /// Date of creation
    initiation_date: NaiveDateTime,
}

impl ChannelMap {
    /// Check a channel/marker pair for resemblance
    pub fn check_matched_pair(&self, channel: String, marker: String) -> bool {
        (self.channel.to_owned(), self.marker.to_owned()) == (channel, marker)
    }
}

impl NormalisedName {
    /// Given a term 'x', determine if 'x' is synonymous to this standard. If so, return the
    /// standardised name.
    pub fn query(&self, x: String) -> Option<String> {
        todo!()
    }
}

impl Panel {
    /// Check excel template and if valid return pandas dataframes
    pub fn check_excel_template(path: PathBuf) -> Option<(u8, u8)> {
        todo!()
    }

    /// Populate panel attributes from an excel template
    pub fn create_from_excel(self, path: PathBuf) -> bool {
        todo!()
    }

    /// Populate panel attributes from a dictionary
    pub fn create_from_dict(self, x: HashMap<String, String>) -> bool {
        todo!()
    }

    // Yields list of channels associated to panel
    pub fn markers(&self) -> impl Iterator<Item = String> {
        vec![].into_iter()
    }

    // Yields list of markers associated to panel
    pub fn channels(&self) -> impl Iterator<Item = String> {
        vec![].into_iter()
    }
}
