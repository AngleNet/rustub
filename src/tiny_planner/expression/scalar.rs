use crate::tiny_planner::ast::{Func, FuncCallExpr};
use crate::tiny_planner::expression::BuiltinFunc::IsNull;
use crate::tiny_planner::types::FieldType;

/// ScalarFunc is the function that returns a value
pub struct ScalarFunc {
    pub return_type: FieldType,
    pub func: BuiltinFunc,
}

pub enum BuiltinFunc {
    IsNull(IsNullFunc),
}

impl ScalarFunc {
    pub fn new(return_type: FieldType, func: BuiltinFunc) -> Self {
        ScalarFunc { return_type, func }
    }

    pub fn with_func_expr(return_type: FieldType, func: FuncCallExpr) -> Self {
        match &func.func {
            Func::IsNull => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }
}

pub struct IsNullFunc {}
