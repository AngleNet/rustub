use super::*;
use crate::common::error::Result;

pub trait AstVisitor {
    /// todo: how to do pre/in/post order traversing?
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
            AstNode::Expression(s) => { self.visit_expression(s) }
            AstNode::SelectStmt(s) => { self.visit_select_stmt(s) }
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

    /// Expressions
    fn visit_expression(&mut self, exp: &mut ExpressionNode) -> Result<()> {
        // what if we want to rewrite the expression?
        match exp {
            ExpressionNode::Between(e) => {
                self.visit_between_and_expr(e)
            }
            ExpressionNode::BinaryOperation(e) => {
                self.visit_binary_operation_expr(e)
            }
            ExpressionNode::Parentheses(e) => {
                self.visit_parentheses_expr(e)
            }
            ExpressionNode::PatternIn(e) => {
                self.visit_pattern_in_expr(e)
            }
            ExpressionNode::UnaryOperation(e) => {
                self.visit_unary_operation_expr(e)
            }
            ExpressionNode::Values => { unimplemented!() }
            ExpressionNode::Variable(e) => {
                self.visit_variable_expr(e)
            }
            ExpressionNode::ColumnName => { unimplemented!() }
            ExpressionNode::Func(f) => {
                self.visit_func_expr(f)
            }
            ExpressionNode::Check(c) => {
                self.visit_check_expr(c)
            }
        }
    }

    fn visit_between_and_expr(&mut self, expr: &mut BetweenExpr) -> Result<()> {
        self.visit_expression(&mut expr.expr)?;
        self.visit_expression(&mut expr.left)?;
        self.visit_expression(&mut expr.right)
    }

    fn visit_binary_operation_expr(&mut self, expr: &mut BinaryOperationExpr) -> Result<()> {
        self.visit_expression(&mut expr.left)?;
        self.visit_expression(&mut expr.right)
    }

    fn visit_parentheses_expr(&mut self, expr: &mut ParenthesesExpr) -> Result<()> {
        self.visit_expression(&mut expr.expr)
    }

    fn visit_pattern_in_expr(&mut self, expr: &mut PatternInExpr) -> Result<()> {
        self.visit_expression(&mut expr.expr)?;
        for e in &mut expr.list {
            self.visit_expression(e)?;
        }
        Ok(())
    }

    fn visit_unary_operation_expr(&mut self, expr: &mut UnaryOperationExpr) -> Result<()> {
        self.visit_expression(&mut expr.expr)
    }

    fn visit_variable_expr(&mut self, expr: &mut VariableExpr) -> Result<()> {
        self.visit_expression(&mut expr.value)
    }

    fn visit_func_expr(&mut self, func: &mut FuncExpr) -> Result<()> {
        match func {
            FuncExpr::FuncCall(f) => {
                self.visit_func_call_expr(f)
            }
            FuncExpr::AggregateFunc(f) => {
                self.visit_agg_func_call_expr(f)
            }
        }
    }

    fn visit_func_call_expr(&mut self, func: &mut FuncCallExpr) -> Result<()> {
        for arg in &mut func.args {
            self.visit_expression(arg)?;
        }
        Ok(())
    }

    fn visit_agg_func_call_expr(&mut self, func: &mut AggregateFuncExpr) -> Result<()> {
        for arg in &mut func.args {
            self.visit_expression(arg)?;
        }
        Ok(())
    }

    fn visit_check_expr(&mut self, check: &mut CheckExpr) -> Result<()> {
        check.count += 1;
        Ok(())
    }
    /// Select statement
    fn visit_select_stmt(&mut self, stmt: &mut SelectStmtNode) -> Result<()> {
        // fields -> from -> where -> group by -> having -> order by -> limit
        self.visit_select_field_list(&mut stmt.fields)?;
        self.visit_table_ref(&mut stmt.from)?;
        if let Some(w) = &mut stmt.whe {
            self.visit_expression(w)?;
        }
        if let Some(g) = &mut stmt.group_by {
            self.visit_group_by(g)?;
        }

        if let Some(h) = &mut stmt.having {
            self.visit_having(h)?;
        }

        if let Some(o) = &mut stmt.order_by {
            self.visit_order_by(o)?;
        }
        if let Some(limit) = &mut stmt.limit {
            self.visit_limit(limit)?;
        }
        Ok(())
    }

    fn visit_select_field_list(&mut self, fields: &mut FieldList) -> Result<()> {
        for field in &mut fields.fields {
            self.visit_select_field(field)?;
        }
        Ok(())
    }

    fn visit_select_field(&mut self, field: &mut SelectField) -> Result<()> {
        match field {
            SelectField::WildCard(w) => {
                self.visit_select_wildcard_field(w)
            }
            SelectField::AliasExpr(e) => {
                self.visit_select_alias_expr_field(e)
            }
        }
    }

    fn visit_select_alias_expr_field(&mut self, field: &mut AliasExprField) -> Result<()> {
        self.visit_expression(&mut field.expr)
    }

    fn visit_select_wildcard_field(&mut self, field: &mut WildCardField) -> Result<()> {
        Ok(())
    }

    fn visit_table_ref(&mut self, table: &mut TableRef) -> Result<()> {
        match table {
            TableRef::SelectStmt(s) => {
                self.visit_select_stmt(s)
            }
            TableRef::TableSource(s) => {
                self.visit_table_source(s)
            }
            TableRef::TableName(n) => {
                self.visit_table_name(n)
            }
            TableRef::Join(j) => {
                self.visit_table_join(j)
            }
        }
    }

    fn visit_table_source(&mut self, source: &mut TableSource) -> Result<()> {
        self.visit_table_ref(&mut source.source)
    }

    fn visit_table_join(&mut self, join: &mut Join) -> Result<()> {
        self.visit_table_ref(&mut join.left)?;
        self.visit_table_ref(&mut join.right)?;
        if let Some(on) = &mut join.on {
            self.visit_join_on_condition(on)?;
        }
        Ok(())
    }

    fn visit_join_on_condition(&mut self, on: &mut OnCondition) -> Result<()> {
        self.visit_expression(&mut on.expr)
    }

    fn visit_group_by(&mut self, group_by: &mut GroupByClause) -> Result<()> {
        for item in &mut group_by.items {
            self.visit_group_or_order_by_item(item)?;
        }
        Ok(())
    }

    fn visit_having(&mut self, having: &mut HavingClause) -> Result<()> {
        self.visit_expression(&mut having.expr)
    }

    fn visit_order_by(&mut self, order_by: &mut OrderByClause) -> Result<()> {
        for item in &mut order_by.items {
            self.visit_group_or_order_by_item(item)?;
        }
        Ok(())
    }

    fn visit_group_or_order_by_item(&mut self, item: &mut ByItem) -> Result<()> {
        self.visit_expression(&mut item.expr)
    }

    fn visit_limit(&mut self, limit: &mut Limit) -> Result<()> {
        if let Some(e) = &mut limit.count {
            self.visit_expression(e)?;
        }
        if let Some(e) = &mut limit.offset {
            self.visit_expression(e)?;
        }
        Ok(())
    }
}
