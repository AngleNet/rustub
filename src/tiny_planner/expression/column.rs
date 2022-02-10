use crate::tiny_planner::expression::Expression;
use crate::tiny_planner::types::FieldType;

/// Column represents a column
pub struct Column {
    pub return_type: FieldType,
    pub original_name: String,
    /// The unique id of this column
    pub unique_id: i64,
}

impl Column {
    /// Returns whether this column is equal to another expression or not. If the column id is same
    /// with another column, they are the same.
    pub fn eq_expr(&self, expr: &Expression) -> bool {
        if let Expression::Column(c) = expr {
            return self.unique_id == c.unique_id;
        }
        return false;
    }
}
