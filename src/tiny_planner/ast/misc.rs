use super::*;

pub struct AnalyzeTableStmtNode {}

/// ExplainStmt is a statement to provide information about how a SQL statement is executed or get
/// column information in a table
pub struct ExplainStmtNode {
    pub stmt: Box<AstNode>,
    pub format: String,
}
