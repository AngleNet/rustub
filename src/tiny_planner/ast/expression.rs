use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
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
    /// todo: why need this?
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
    pub count: Rc<RefCell<u32>>,
}

impl CheckExpr {
    pub fn reset(&mut self) {
        let mut v = self.count.borrow_mut();
        *v = 0;
    }

    pub fn increment(&mut self) {
        let mut v = self.count.borrow_mut();
        *v += 1;
    }
}

impl Clone for CheckExpr {
    fn clone(&self) -> Self {
        CheckExpr {
            count: self.count.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct CheckVisitor {}

    impl AstVisitor for CheckVisitor {}

    struct TestCase {
        node: ExpressionNode,
        expected_count: u32,
    }

    impl TestCase {
        fn new(node: ExpressionNode, expected_count: u32) -> Self {
            TestCase {
                node,
                expected_count,
            }
        }
    }

    #[inline]
    fn wrap_check(check: &CheckExpr) -> Box<ExpressionNode> {
        Box::new(ExpressionNode::Check(check.clone()))
    }

    #[test]
    pub fn expression_visitor_cover() {
        let mut check = CheckExpr {
            count: Rc::new(RefCell::new(0))
        };
        let mut cases = vec![
            TestCase {
                node: ExpressionNode::Between(
                    BetweenExpr {
                        expr: wrap_check(&check),
                        left: wrap_check(&check),
                        right: wrap_check(&check),
                        not: false,
                    }),
                expected_count: 3,
            },
            TestCase {
                node: ExpressionNode::BinaryOperation(
                    BinaryOperationExpr {
                        op: Op::And,
                        left: wrap_check(&check),
                        right: wrap_check(&check),
                    }
                ),
                expected_count: 2,
            },
            TestCase {
                node: ExpressionNode::Parentheses(
                    ParenthesesExpr {
                        expr: wrap_check(&check)
                    }
                ),
                expected_count: 1,
            },
            TestCase {
                node: ExpressionNode::UnaryOperation(
                    UnaryOperationExpr {
                        op: Op::LogicAnd,
                        expr: wrap_check(&check),
                    }
                ),
                expected_count: 1,
            },
            TestCase {
                node: ExpressionNode::Variable(
                    VariableExpr {
                        name: "".to_string(),
                        is_global: false,
                        is_system: false,
                        value: wrap_check(&check),
                    }
                ),
                expected_count: 1,
            },
        ];
        let mut visitor = CheckVisitor {};
        for case in &mut cases {
            check.reset();
            assert!(visitor.visit_expression(&mut case.node).is_ok());
            assert_eq!(case.expected_count, *check.count.borrow());
        }
    }
}
