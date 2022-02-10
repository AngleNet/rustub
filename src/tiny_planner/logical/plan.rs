use crate::tiny_planner::catalog::{Table, TableRef};
use crate::tiny_planner::expression::*;
use crate::tiny_planner::logical::Plan;
use crate::tiny_planner::property::StatsInfo;
use crate::tiny_planner::util::AccessPath;
use std::sync::{Arc, RwLock};

/// Logical operators include: data source, table scan, selection, projection, sort, limit,
/// aggregation, join, index scan

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

impl Plan for DataSource {
    fn schema(&mut self) -> &mut PlanSchema {
        &mut self.schema
    }
}

/// Represents a where or having predicate.
pub struct LogicalSelection {
    pub schema: PlanSchema,
    pub conditions: Vec<Expression>,
}

impl Plan for LogicalSelection {
    fn schema(&mut self) -> &mut PlanSchema {
        &mut self.schema
    }
}

/// LogicalTableScan is the logical table scan operator for a KV store
pub struct LogicalTableScan {
    pub schema: PlanSchema,
    pub source: DataSource,
}

pub struct LogicalProjection {
    pub schema: PlanSchema,
    pub exprs: Vec<Expression>,
}

pub struct LogicalAggregation {
    pub schema: PlanSchema,
    pub agg_funcs: Vec<AggFuncDesc>,
    pub group_by_items: Vec<Expression>,
    pub group_by_cols: Vec<Column>,
}

pub struct LogicalIndexScan {}

pub struct LogicalSort {}

pub struct LogicalLimit {}

pub struct LogicalJoin {}
