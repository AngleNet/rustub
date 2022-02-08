use crate::tiny_planner::statistics::HistColl;

/// StatsInfo stores the basic information of statistics for the plan's output. It's used for cost
/// estimation.
pub struct StatsInfo {
    row_count: f64,
    cardinality: Vec<f64>,
    histograms: HistColl,
    /// Indicates the statistics version of a table. If the StatsInfo is calculated using the pseudo
    /// statistics on a table, stats_version will be PseudoVersion
    /// todo: what is PseudoVersion?
    stats_version: u64
}
