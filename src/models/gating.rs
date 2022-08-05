use chrono::NaiveDateTime;

/// Document representation of a Gate
pub struct Gate {
    /// Unique identifier for this gate (within the scope of a GatingStrategy)
    pub name: String,
    /// List of population names; populations derived from application of this gate
    pub children: Vec<String>,
    /// Name of parent population; the population this gate acts upon
    pub parent: String,
    /// Name of gating class used to generate gate (see flow.gating.actions)
    pub class: String,
    /// Name of class method used to generate gate (see flow.gating.actions)
    pub method: String,
    /// list of keyword arguments (list of tuples; first element = key, second element = value)
    /// passed to class/method to generate gate
    pub keywords: Vec<(String, String)>,
}

/// Document representation of a gating template; a gating template is a collection of gating
/// objects that can be applied to multiple fcs files or an entire experiment in bulk.
pub struct GatingStrategy {
    /// Unique identifier for template
    name: String,
    /// List of Gate documents; see Gate
    gates: Vec<Gate>,
    /// Date of creation
    creation_date: NaiveDateTime,
    /// Date of last edit
    last_edit: NaiveDateTime,
    /// Warnings associated to this gating template
    flags: Option<String>,
    /// free text comments
    notes: Option<String>,
}
