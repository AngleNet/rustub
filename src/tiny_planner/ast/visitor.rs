use crate::tiny_planner::ast::node::AstNode;

pub trait AstVisitor {
    fn visit(&mut self, node: &AstNode) {
        match node {
            AstNode::CreateDatabaseStmt => {}
            AstNode::DropDatabaseStmt => {}
            AstNode::CreateTableStmt => {}
            AstNode::DropTableStmt => {}
            AstNode::AlterTableStmt => {}
        }
    }
}
