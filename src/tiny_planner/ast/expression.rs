use super::*;

pub enum ExpressionNode {
    Between(BetweenExpr),
    BinaryOperation(BinaryOperationExpr),
    Parentheses(ParenthesesExpr),
    PatternIn(PatternInExpr),
    UnaryOperation(UnaryOperationExpr),
    Values,
    Variable(VariableExpr),
    ColumnName,
    Func(FuncExpr),
    Check(CheckExpr),
}

/// "between and" or "not between and"
pub struct BetweenExpr {
    /// The expression to be checked
    pub expr: Box<ExpressionNode>,
    /// The minimal value in the range
    pub left: Box<ExpressionNode>,
    /// The maximum value in the range
    pub right: Box<ExpressionNode>,
    /// Not is true, the expression is "not between and"
    pub not: bool,
}

pub struct BinaryOperationExpr {
    /// Op is the operator code for BinaryOperation
    pub op: Op,
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

pub struct ParenthesesExpr {
    pub expr: Box<ExpressionNode>,
}

pub struct PatternInExpr {
    /// Expr is the value to be compared
    pub expr: Box<ExpressionNode>,
    /// List is the list expression in compare list
    pub list: Vec<ExpressionNode>,
    /// Not is true, the expression is "not in"
    pub not: bool,
}

pub struct UnaryOperationExpr {
    pub op: Op,
    pub expr: Box<ExpressionNode>,
}

pub struct VariableExpr {
    /// Name is the variable name
    pub name: String,
    /// Indicates whether this variable is global
    pub is_global: bool,
    /// Indicates whether this variable is a system variable in current session
    pub is_system: bool,
    /// The value of this variable
    pub value: Box<ExpressionNode>,
}

pub enum FuncExpr {
    FuncCall(FuncCallExpr),
    AggregateFunc(AggregateFuncExpr),
}

pub struct FuncCallExpr {
    pub func: Func,
    pub args: Vec<ExpressionNode>,
}

pub enum Func {
    IsNull,
    Length,
    Strcmp,
    If,
    IfNull,
}

pub enum AggregateFunc {
    Count,
    Sum,
    Avg,
    FirstRow,
    Min,
    Max,
}

pub struct AggregateFuncExpr {
    pub func: AggregateFunc,
    pub args: Vec<ExpressionNode>,
}

pub struct CheckExpr {
    pub count: u32,
}
