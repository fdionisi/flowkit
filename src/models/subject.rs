use std::path::PathBuf;

use chrono::NaiveDateTime;

pub type SubjectId = usize;

pub enum BugStatus {
    Pve,
    Nve,
    Unknown,
}

pub enum OrganismType {
    Bacteria,
    Fungi,
    Virus,
}

pub enum SubjectGender {
    Male,
    Female,
}

/// Model for a custom dictionary that can be used for given descriptions to meta-data.
/// Helpful when exploring single cell data that has been associated to meta-data in the Explorer object;
/// see flow.clustering.main.Explorer)
pub struct MetaDataDictionary {
    /// name of meta-data (column name)
    pub key: String,
    /// string value of writen description
    pub desc: String,
}

/// Document representation of drug administration. Single document instance represents one event.
pub struct Drug {
    /// name of therapy/drug
    pub name: String,
    /// date that therapy/drug started
    pub init_date: Option<NaiveDateTime>,
    /// date that therapy/drug started
    pub end_date: Option<NaiveDateTime>,
}

/// Document representation of isolated pathogen. Single document instance represents one pathogen.
pub struct Bug {
    /// value of organisms gram status
    pub gram_status: Option<BugStatus>,
    /// value of hmbpp status
    pub hmbpp_status: Option<BugStatus>,
    /// value of organisms ribo status
    pub ribo_status: Option<BugStatus>,
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

/// Document representation of biological test (blood pathology). Single document instance represents one test.
pub struct Biology {
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

/// Document based representation of subject meta-data. Subjects are stored in a dynamic document,
/// meaning new properties can be added ad-hoc.
pub struct Subject {
    /// Unique identifier for subject
    pub subject_id: SubjectId,
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
    pub infection_data: Vec<Bug>,
    /// Associated biological data
    pub patient_biology: Vec<Biology>,
    /// Additional notes
    pub notes: Option<String>,
}

pub enum GramStatus {
    Mixed,
    Unknown,
}

impl Subject {
    /// Given an instance of Subject, return the gram status of isolated organisms.
    /// Where multiple organisms are found, if gram status differs amongst orgs, returns 'mixed'
    pub fn gram_status(&self) {
        todo!()
    }

    /// Fetch the name of isolated organisms for each patient.
    pub fn bugs(&self) {
        todo!()
    }

    /// Parse all infectious isolates for each patient and return the organism type isolated, one
    /// of either: 'gram positive', 'gram negative', 'virus', 'mixed' or 'fungal'
    pub fn org_type(&self) {
        todo!()
    }

    /// Given a value of either 'hmbpp' or 'ribo' for 'field' argument, return True if any Bug has
    /// a positive status for the given patient ID.
    pub fn hmbpp_ribo(&self, field: String) {
        todo!()
    }

    ///  Given some test name, return a summary statistic of all results for a given patient ID
    pub fn biology(subject_id: SubjectId, test_name: String, method: String) {
        todo!()
    }
}
