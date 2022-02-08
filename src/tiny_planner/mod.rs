use std::collections::HashMap;
use crate::tiny_planner::expression::{Column, Schema};
use crate::tiny_planner::property::StatsInfo;

mod plan;
mod expression;
mod property;
mod statistics;
mod physical;
mod logical;
mod ast;


/// Plan is the description of an execution flow. It is created from Ast Node first, then optimized
/// by the optimizer, finally used by the executor to create a Cursor which executes the statement.
pub trait Plan {
    fn schema(&self) -> &Schema;

    fn id(&self) -> i32;

    fn plan_type(&self) -> &str;

    fn explain_info(&self) -> &str;

    fn replace_expr_columns(&mut self, replace: HashMap<String, Column>);

    fn stats_info(&self) -> &StatsInfo;

    fn output_names(&self) -> &[&str];

    fn set_output_names(&self, names: &[&str]);
}

/// LogicalPlan is a tree of logical operators.
/// We can do a lot of logical optimizations to it, like predicate push down and column pruning.
pub trait LogicalPlan: Plan {}

/// PhysicalPlan is tree of physical operators.
pub trait PhysicalPlan: Plan {}
