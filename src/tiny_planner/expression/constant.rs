use crate::tiny_planner::types::{DataBox, FieldType};

/// Constant stands for a constant value.
pub struct Constant {
    pub return_type: FieldType,
    pub value: DataBox,
}
