use super::*;

///(enter) |  ^
///        v  | (leave)
///        Node
///       / | ... \
///      x  x      x
///
/// A visitor can be used to visit an AST whose procedure can be customized enter and leave callbacks
/// which are called before and after the AST is visited. Let the specific node handling how to walk
/// it correctly.
pub trait Visitor {
    /// Enter is called before children nodes are visited. It also returns whether its children should
    /// be visited or not.
    fn enter(&self, node: AstNode) -> (AstNode, bool);

    /// Leave is called after children nodes have been visited. It also returns whether the visitor
    /// should be stopped or not.
    fn leave(&self, node: AstNode) -> (AstNode, bool);
}
