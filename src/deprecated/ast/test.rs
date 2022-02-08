use crate::deprecated::ast::*;

pub(crate) struct CheckVisitor {}

impl Visitor for CheckVisitor {
    fn enter(&self, mut node: AstNode) -> (AstNode, bool) {
        match node {
            AstNode::Expression(ExpressionNode::Check(ref mut e)) => {
                e.enter();
                // skip the children
                (node, true)
            }
            // keep searching
            _ => (node, false),
        }
    }

    fn leave(&self, mut node: AstNode) -> (AstNode, bool) {
        match node {
            AstNode::Expression(ExpressionNode::Check(ref mut e)) => {
                e.leave();
            }
            _ => {}
        }
        // stop the visit
        (node, true)
    }
}
