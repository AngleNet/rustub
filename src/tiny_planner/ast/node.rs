use super::*;

pub enum AstNode {
    CreateDatabaseStmt(CreateDatabaseStmtNode),
    DropDatabaseStmt(DropDatabaseStmtNode),
    CreateTableStmt(CreateTableStmtNode),
    DropTableStmt(DropTableStmtNode),
    AlterTableStmt(AlterTableStmtNode),
    TruncateTableStmt(TruncateTableStmtNode),
    CreateIndexStmt(CreateIndexStmtNode),
    DropIndexStmt(DropIndexStmtNode),
}
