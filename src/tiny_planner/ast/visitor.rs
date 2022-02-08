use crate::tiny_planner::ast::AstNode;

pub trait Visitor {
    /// Enter is called before children nodes are visited.
    fn enter(&self, node: AstNode) -> (AstNode, bool);

    fn leave(&self, node: AstNode) -> AstNode;
}
