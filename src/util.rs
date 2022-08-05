pub mod compensate {
    /// Extracts spillover matrix from FCS text entry.
    /// Returns (spillover matrix new_spill, column headers)
    pub fn get_spill(text: String) {}

    /// Compensate numpy data 'npy' given spillover matrix 'spill'
    /// and marker indices to compensate
    pub fn compensate() {}

    /// Generates spillover matrix for one FCS file (presumably from beads)
    /// npy: the numpy array of the bead data
    /// stain_index: index of the stained channel
    pub fn gen_spill_matrix() {}
}

pub mod transforms {
    /// return the lower nth quantile
    pub fn quantile(data: Vec<i32>, n: usize) {}

    /// Product logarithm or LambertW function computes principal solution
    /// for w in f(w) = w*exp(w).
    pub fn product_log(data: Vec<i32>, n: usize) {}
}
