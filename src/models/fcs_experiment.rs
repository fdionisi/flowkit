use super::{ClusterId, FileField, GatingStrategy, PanelId};

pub type FcsExperimentId = usize;

pub struct FcsExperiment {
    /// Unique identifier for experiment
    pub experiment_id: FcsExperimentId,
    /// Panel object describing associated channel/marker pairs
    pub panel: PanelId,
    /// Reference field for associated files
    pub fcs_files: Vec<FileField>,
    /// Warnings associated to experiment
    pub flags: Option<String>,
    /// Additional free text comments
    pub notes: Option<String>,
    /// Reference to gating templates associated to this experiment
    pub gating_strategies: Vec<GatingStrategy>,
    /// List of IDs for meta clusters belonging to this experiment
    pub meta_cluster_ids: Vec<ClusterId>,
}
