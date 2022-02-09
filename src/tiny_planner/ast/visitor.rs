use super::*;
use crate::common::error::Result;

pub trait AstVisitor {
    fn visit(&mut self, mut node: &mut AstNode) -> Result<()> {
        match node {
            AstNode::CreateDatabaseStmt(s) => {
                self.visit_create_database_stmt(s)
            }
            AstNode::DropDatabaseStmt(s) => {
                self.visit_drop_database_stmt(s)
            }
            AstNode::CreateTableStmt(s) => {
                self.visit_create_table_stmt(s)
            }
            AstNode::DropTableStmt(s) => {
                self.visit_drop_table_stmt(s)
            }
            AstNode::AlterTableStmt(s) => {
                self.visit_alter_table_stmt(s)
            }
            AstNode::TruncateTableStmt(s) => {
                self.visit_truncate_table_stmt(s)
            }
            AstNode::CreateIndexStmt(s) => {
                self.visit_create_index_stmt(s)
            }
            AstNode::DropIndexStmt(s) => {
                self.visit_drop_index_stmt(s)
            }
        }
    }

    fn visit_create_database_stmt(&mut self, node: &CreateDatabaseStmtNode) -> Result<()> {
        Ok(())
    }

    fn visit_drop_database_stmt(&mut self, node: &DropDatabaseStmtNode) -> Result<()> {
        Ok(())
    }

    fn visit_create_table_stmt(&mut self, node: &mut CreateTableStmtNode) -> Result<()> {
        node.table = self.visit_table_name(&node.table).unwrap();
        if let Some(ref n) = node.refer_table {
            node.refer_table = Some(self.visit_table_name(n).unwrap());
        }
        for idx in 0..node.columns.len() {
            node.columns[idx] = self.visit_table_column(&mut node.columns[idx]).unwrap();
        }
        for idx in 0..node.constraints.len() {
            node.constraints[idx] = self.visit_table_constraint(&node.constraints[idx]).unwrap();
        }
        Ok(())
    }

    fn visit_table_name(&mut self, name: &TableName) -> Result<TableName> {
        Ok(name.clone())
    }

    fn visit_table_column(&mut self, col: &mut ColumnDef) -> Result<ColumnDef> {
        let mut new = col.clone();
        new.name = self.visit_column_name(&new.name).unwrap();
        for idx in 0..new.options.len() {
            new.options[idx] = self.visit_column_option(&new.options[idx]).unwrap();
        }
        Ok(new)
    }

    fn visit_column_name(&mut self, col: &ColumnName) -> Result<ColumnName> {
        Ok(col.clone())
    }

    fn visit_column_option(&mut self, opt: &ColumnOption) -> Result<ColumnOption> {
        let new = match opt {
            ColumnOption::DefaultValue(e) => {
                ColumnOption::DefaultValue(self.visit_expression(e).unwrap())
            }
            ColumnOption::OnUpdate(e) => {
                ColumnOption::OnUpdate(self.visit_expression(e).unwrap())
            }
            ColumnOption::Generated(e) => {
                ColumnOption::Generated(self.visit_expression(e).unwrap())
            }
            ColumnOption::PrimaryKey => { ColumnOption::PrimaryKey }
            ColumnOption::NotNull => { ColumnOption::NotNull }
            ColumnOption::AutoIncrement => { ColumnOption::AutoIncrement }
            ColumnOption::UniqKey => { ColumnOption::UniqKey }
            ColumnOption::Null => { ColumnOption::Null }
            ColumnOption::Fulltext => { ColumnOption::Fulltext }
            ColumnOption::Comment => { ColumnOption::Comment }
            ColumnOption::Reference => { ColumnOption::Reference }
            ColumnOption::Check(b) => { ColumnOption::Check(*b) }
            ColumnOption::ColumnFormat => { ColumnOption::ColumnFormat }
            ColumnOption::Storage => { ColumnOption::Storage }
            ColumnOption::AutoRandom => { ColumnOption::AutoRandom }
        };
        Ok(new)
    }

    fn visit_table_constraint(&mut self, constraint: &TableConstraint) -> Result<TableConstraint> {
        let new = match constraint {
            TableConstraint::NoConstraint => { TableConstraint::NoConstraint }
            TableConstraint::PrimaryKey => { TableConstraint::PrimaryKey }
            TableConstraint::Key => { TableConstraint::Key }
            TableConstraint::Index => { TableConstraint::Index }
            TableConstraint::Uniq => { TableConstraint::Uniq }
            TableConstraint::UniqKey => { TableConstraint::UniqKey }
            TableConstraint::UniqIndex => { TableConstraint::UniqIndex }
            TableConstraint::ForeignKey => { TableConstraint::ForeignKey }
            TableConstraint::Fulltext => { TableConstraint::Fulltext }
            TableConstraint::Check => { TableConstraint::Check }
        };
        Ok(new)
    }

    fn visit_expression(&mut self, exp: &ExpressionNode) -> Result<ExpressionNode> {
        todo!()
    }

    fn visit_drop_table_stmt(&mut self, stmt: &mut DropTableStmtNode) -> Result<()> {
        for idx in 0..stmt.tables.len() {
            stmt.tables[idx] = self.visit_table_name(&stmt.tables[idx]).unwrap();
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
