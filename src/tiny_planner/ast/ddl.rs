pub struct DatabaseOption {}

/// Statement which creates a database
pub struct CreateDatabaseStmtNode {
    if_not_exists: bool,
    name: String,
    options: Vec<DatabaseOption>,
}

/// Statement which drops a database
pub struct DropDatabaseStmtNode {}

pub struct CreateTableStmtNode {}

pub struct AlterTableStmtNode {}

pub struct DropTableStmtNode {}

pub struct CreateIndexStmtNode {}

pub struct AlterIndexStmtNode {}

pub struct DropIndexStmtNode {}
