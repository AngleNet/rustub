use crate::tiny_planner::ast::ExpressionNode;
use crate::tiny_planner::expression::Column;

/// Create database statement
#[derive(Default)]
pub struct CreateDatabaseStmtNode {
    pub if_not_exists: bool,
    pub name: String,
    pub options: Vec<DatabaseOption>,
}

pub enum DatabaseOption {
    Charset,
    Collate,
    Encryption,
}

/// Drop database statement
#[derive(Default)]
pub struct DropDatabaseStmtNode {
    pub if_exists: bool,
    pub name: String,
}

/// Create table statement
#[derive(Default)]
pub struct CreateTableStmtNode {
    pub if_not_exists: bool,
    pub is_temporary: bool,
    pub table: TableName,
    pub refer_table: Option<TableName>,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
}

impl Clone for CreateTableStmtNode {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[derive(Default)]
pub struct TableName {
    pub schema: String,
    pub name: String,
    pub partition_names: Vec<String>,
}

impl Clone for TableName {
    fn clone(&self) -> Self {
        TableName {
            schema: self.schema.clone(),
            name: self.name.clone(),
            partition_names: self.partition_names.clone(),
        }
    }
}

#[derive(Default)]
pub struct ColumnDef {
    pub name: ColumnName,
    pub field_type: FieldType,
    /// A column could have multiple options such as not null, default ...
    pub options: Vec<ColumnOption>,
}

impl Clone for ColumnDef {
    fn clone(&self) -> Self {
        ColumnDef {
            name: self.name.clone(),
            field_type: self.field_type.clone(),
            options: self.options.clone(),
        }
    }
}

#[derive(Default)]
pub struct ColumnName {
    pub schema: String,
    pub table: String,
    pub name: String,
}

impl Clone for ColumnName {
    fn clone(&self) -> Self {
        ColumnName {
            schema: self.schema.clone(),
            table: self.table.clone(),
            name: self.name.clone(),
        }
    }
}

#[derive(Clone)]
pub enum ColumnOption {
    PrimaryKey,
    NotNull,
    AutoIncrement,
    DefaultValue(ExpressionNode),
    UniqKey,
    Null,
    OnUpdate(ExpressionNode),
    Fulltext,
    Comment,
    Generated(ExpressionNode),
    Reference,
    Check(bool),
    ColumnFormat,
    Storage,
    AutoRandom,
}

#[derive(Default, Clone, Copy)]
pub struct FieldType {}

/// fixme: missing index options and specifications
pub enum TableConstraint {
    NoConstraint,
    PrimaryKey,
    Key,
    Index,
    Uniq,
    UniqKey,
    UniqIndex,
    ForeignKey,
    Fulltext,
    Check,
}

/// Drop table statement
pub struct DropTableStmtNode {
    pub if_exists: bool,
    pub tables: Vec<TableName>,
    pub is_view: bool,
    pub is_temporary: bool,
}

impl Clone for DropTableStmtNode {
    fn clone(&self) -> Self {
        DropTableStmtNode {
            if_exists: self.if_exists,
            tables: self.tables.clone(),
            is_view: self.is_view,
            is_temporary: self.is_temporary,
        }
    }
}

/// Alter table statement
pub struct AlterTableStmtNode {}

/// Truncate table statement
pub struct TruncateTableStmtNode{}

/// Create index statement
pub struct CreateIndexStmtNode{}
/// Drop index statement
pub struct DropIndexStmtNode{}
