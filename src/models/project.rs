use chrono::NaiveDateTime;

use super::{FcsExperiment, Subject};

pub type ProjectId = usize;

/// Document representation of Project
pub struct Project {
    /// unique identifier for project
    pub id: ProjectId,
    /// List of references for associated subjects; see Subject
    pub subjects: Vec<Subject>,
    /// date of creation
    pub start_date: NaiveDateTime,
    /// user name of owner
    pub owner: String,
    /// List of references for associated fcs files
    pub fcs_experiments: Vec<FcsExperiment>,
}
