use crate::common::error::Result;
use crate::tiny_planner::ast::{AstNode, AstVisitor, SelectStmtNode};

use super::*;

pub struct PlanBuilder {}

impl PlanBuilder {
    pub fn build(&mut self, node: &AstNode) -> Result<LogicalPlan> {
        match node {
            AstNode::SelectStmt(s) => self.build_select(s),
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn build_select(&mut self, stmt: &SelectStmtNode) -> Result<LogicalPlan> {
        todo!()
    }
}
