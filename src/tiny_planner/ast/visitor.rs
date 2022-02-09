use super::*;
use crate::common::error::Result;

pub trait AstVisitor {
    fn visit(&mut self, mut node: &mut AstNode) -> Result<()> {
        match node {
            AstNode::CreateDatabaseStmt(s) => self.visit_create_database_stmt(s),
            AstNode::DropDatabaseStmt(s) => self.visit_drop_database_stmt(s),
            AstNode::CreateTableStmt(s) => self.visit_create_table_stmt(s),
            AstNode::DropTableStmt(s) => self.visit_drop_table_stmt(s),
            AstNode::AlterTableStmt(s) => self.visit_alter_table_stmt(s),
            AstNode::TruncateTableStmt(s) => self.visit_truncate_table_stmt(s),
            AstNode::CreateIndexStmt(s) => self.visit_create_index_stmt(s),
            AstNode::DropIndexStmt(s) => self.visit_drop_index_stmt(s),
        }
    }

    fn visit_create_database_stmt(&mut self, node: &CreateDatabaseStmtNode) -> Result<()> {
        Ok(())
    }

    fn visit_drop_database_stmt(&mut self, node: &DropDatabaseStmtNode) -> Result<()> {
        Ok(())
    }

    fn visit_create_table_stmt(&mut self, node: &mut CreateTableStmtNode) -> Result<()> {
        self.visit_table_name(&mut node.table)?;
        if let Some(ref mut n) = node.refer_table {
            self.visit_table_name(n)?;
        }
        for idx in 0..node.columns.len() {
            self.visit_table_column(&mut node.columns[idx])?;
        }
        for idx in 0..node.constraints.len() {
            self.visit_table_constraint(&mut node.constraints[idx])?;
        }
        Ok(())
    }

    fn visit_table_name(&mut self, name: &mut TableName) -> Result<()> {
        Ok(())
    }

    fn visit_table_column(&mut self, col: &mut ColumnDef) -> Result<()> {
        self.visit_column_name(&mut col.name)?;
        for idx in 0..col.options.len() {
            self.visit_column_option(&mut col.options[idx])?;
        }
        Ok(())
    }

    fn visit_column_name(&mut self, col: &ColumnName) -> Result<()> {
        Ok(())
    }

    fn visit_column_option(&mut self, mut opt: &mut ColumnOption) -> Result<()> {
        match opt {
            ColumnOption::DefaultValue(e)
            | ColumnOption::OnUpdate(e)
            | ColumnOption::Generated(e) => {
                self.visit_expression(e)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_table_constraint(&mut self, constraint: &mut TableConstraint) -> Result<()> {
        Ok(())
    }

    fn visit_expression(&mut self, exp: &mut ExpressionNode) -> Result<()> {
        // what if we want to rewrite the expression?
        todo!()
    }

    fn visit_drop_table_stmt(&mut self, stmt: &mut DropTableStmtNode) -> Result<()> {
        for idx in 0..stmt.tables.len() {
            self.visit_table_name(&mut stmt.tables[idx])?;
        }
        Ok(())
    }

    fn visit_alter_table_stmt(&mut self, stmt: &mut AlterTableStmtNode) -> Result<()> {
        todo!()
    }

    fn visit_truncate_table_stmt(&mut self, stmt: &mut TruncateTableStmtNode) -> Result<()> {
        todo!()
    }

    fn visit_create_index_stmt(&mut self, stmt: &mut CreateIndexStmtNode) -> Result<()> {
        todo!()
    }

    fn visit_drop_index_stmt(&mut self, stmt: &mut DropIndexStmtNode) -> Result<()> {
        todo!()
    }
}
