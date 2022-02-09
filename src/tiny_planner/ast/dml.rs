use super::*;

/// Select statement
pub struct SelectStmtNode {
    /// Whether the select has distinct option
    pub distinct: bool,
    pub from: Box<TableRef>,
    /// Where clause
    pub whe: Option<ExpressionNode>,
    /// Select expression list
    pub fields: FieldList,
    pub group_by: Option<GroupByClause>,
    pub having: Option<HavingClause>,
    pub order_by: Option<OrderByClause>,
    pub limit: Option<Limit>,
}

pub struct Join {
    pub left: Box<TableRef>,
    // todo: in tinysql, right could be null, why?
    pub right: Box<TableRef>,
    pub join_type: JoinType,
    pub on: Option<OnCondition>,
}

pub enum JoinType {
    Left,
    Right,
    Cross,
}

pub struct OnCondition {
    pub expr: ExpressionNode,
}

pub enum TableRef {
    SelectStmt(SelectStmtNode),
    TableSource(TableSource),
    TableName(TableName),
    Join(Join),
}

/// TableSource represents table source with a name
pub struct TableSource {
    pub source: Box<TableRef>,
    pub as_name: String,
}

pub struct FieldList {
    pub fields: Vec<SelectField>,
}

/// SelectField represents fields in select statements. There are two types of select field: wildcard
/// and expression with optional alias name
pub enum SelectField {
    WildCard(WildCardField),
    AliasExpr(AliasExprField),
}

/// WildCardField is a special type of select select field content
pub struct WildCardField {
    schema: String,
    table: String,
}

pub struct AliasExprField {
    pub expr: ExpressionNode,
    pub as_name: String,
}

pub struct OrderByClause {
    pub items: Vec<ByItem>,
}

pub struct ByItem {
    pub expr: ExpressionNode,
    pub desc: bool,
}

pub struct Limit {
    pub count: Option<ExpressionNode>,
    pub offset: Option<ExpressionNode>,
}

pub struct GroupByClause {
    pub items: Vec<ByItem>,
}

pub struct HavingClause {
    pub expr: ExpressionNode,
}
