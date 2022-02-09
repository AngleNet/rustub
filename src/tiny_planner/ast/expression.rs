use super::*;

pub enum ExpressionNode {
    Between,
    BinaryOperation,
    Parentheses,
    PatternIn,
    Row,
    UnaryExpression,
    Values,
    Variable,
    ColumnName,
}
