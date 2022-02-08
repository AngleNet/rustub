use super::*;

pub trait Node<V: Visitor> {
    /// Accepts a visitor to visit itself.
    /// The returned node should replace the original node. It also returns whether to stop visiting
    /// or not. It's actually modifying the node(actually a tree) according to the visitor.
    fn accept(self, visitor: &mut V) -> (AstNode, bool);
}

pub enum AstNode {
    /// DDL
    CreateDatabaseStmt(CreateDatabaseStmtNode),
    DropDatabaseStmt(DropDatabaseStmtNode),
    CreateTableStmt(CreateTableStmtNode),
    AlterTableStmt(AlterTableStmtNode),
    DropTableStmt(DropTableStmtNode),
    CreateIndexStmt(CreateIndexStmtNode),
    AlterIndexStmt(AlterTableStmtNode),
    DropIndexStmt(DropIndexStmtNode),

    /// DDL options
    IndexOption(IndexOptionNode),
    ColumnOption(ColumnOptionNode),

    Expression(ExpressionNode),
}

impl<V> Node<V> for AstNode where V: Visitor {
    fn accept(self, visitor: &mut V) -> (AstNode, bool) {
        match self {
            AstNode::CreateDatabaseStmt(s) => s.accept(visitor),
            _ => { panic!("") }
        }
    }
}
