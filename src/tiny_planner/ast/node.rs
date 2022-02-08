use super::*;

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
    /// DML
    /// Expressions
}
