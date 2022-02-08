use super::*;

pub enum ExpressionNode {
    /// Test only stuff
    Check(CheckExprNode)
}

#[derive(Default)]
pub struct CheckExprNode {
    enter_count: i32,
    leave_count: i32,
}

impl CheckExprNode {
    pub fn leave(&mut self) {
        self.leave_count += 1;
    }

    pub fn enter(&mut self) {
        self.enter_count += 1;
    }

    pub fn leave_count(&self) -> i32 {
        self.leave_count
    }

    pub fn enter_count(&self) -> i32 {
        self.enter_count
    }
}

impl<V: Visitor> Node<V> for CheckExprNode {
    fn accept(self, visitor: &mut V) -> (AstNode, bool) {
        let (node, _) = visitor.enter(AstNode::Expression(ExpressionNode::Check(self)));
        visitor.leave(node)
    }
}
