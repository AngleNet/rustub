use crate::tiny_planner::catalog::{Table, TableRef};
use crate::tiny_planner::expression::*;
use crate::tiny_planner::property::StatsInfo;
use crate::tiny_planner::util::AccessPath;
use std::sync::{Arc, RwLock};

pub struct PlanSchema {
    pub schema: Schema,
    pub fields: Vec<FieldName>,
}

/// Represents a table scan.
///
/// It should include:
///     1. simple data location(which database, table)
///     2. handle of the underlying table,
///     3. table meta,
///     4. filters which should be pushed down
///     5. possible access paths, table statistics
///     6. plan statistics
pub struct DataSource {
    schema: PlanSchema,
    database: String,
    table: String,
    instance: TableRef,
    /// Filters should be pushed down over this data source
    push_down_expr: Vec<Expression>,
    /// All of filters should be applied over this data source
    filter_expr: Vec<Expression>,
    statistics: StatsInfo,
    possible_access_path: Vec<AccessPath>,
}

impl DataSource {
    pub fn schema(&mut self) -> &mut PlanSchema {
        &mut self.schema
    }
}
