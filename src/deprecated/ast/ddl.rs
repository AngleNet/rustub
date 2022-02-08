use std::ops::Deref;
use super::*;

#[derive(Default)]
pub struct DatabaseOption {
    option_type: i32,
    value: String,
}

/// Statement which creates a database
#[derive(Default)]
pub struct CreateDatabaseStmtNode {
    if_not_exists: bool,
    name: String,
    options: Vec<DatabaseOption>,
}

impl<V: Visitor> Node<V> for CreateDatabaseStmtNode {
    fn accept(self, v: &mut V) -> (AstNode, bool) {
        let (node, _) = v.enter(AstNode::CreateDatabaseStmt(self));
        v.leave(node)
    }
}

/// Statement which drops a database and all tables in the database
#[derive(Default)]
pub struct DropDatabaseStmtNode {
    if_exists: bool,
    name: String,
}

impl<V: Visitor> Node<V> for DropDatabaseStmtNode {
    fn accept(self, v: &mut V) -> (AstNode, bool) {
        let (node, _) = v.enter(AstNode::DropDatabaseStmt(self));
        v.leave(node)
    }
}

pub struct CreateTableStmtNode {
    if_not_exists: bool,
    is_temporary: bool,
    table: TableName,
    refer_table: TableName,
}

pub struct AlterTableStmtNode {}

pub struct DropTableStmtNode {}

pub struct CreateIndexStmtNode {}

pub struct AlterIndexStmtNode {}

pub struct DropIndexStmtNode {}

pub enum OptionNode {
    Index(IndexOption),
    Column(ColumnOption),
}


impl<V: Visitor> Node<V> for OptionNode {
    fn accept(self, v: &mut V) -> (AstNode, bool) {
        match self {
            OptionNode::Index(o) => o.accept(v),
            OptionNode::Column(o) => o.accept(v),
        }
    }
}

pub enum DefNode {
    Column(ColumnDef),
}

///////////////////////// misc

pub struct FieldType {}

pub struct TableName {
    schema: String,
    name: String,
    partitions: Vec<String>,
}

pub struct ColumnName {
    schema: String,
    table: String,
    name: String,
}

pub struct ColumnDef {
    name: TableName,
    field_type: FieldType,
    options: Vec<ColumnOption>,
}

impl<V> Node<V> for ColumnDef where V: Visitor {
    fn accept(self, v: &mut V) -> (AstNode, bool) {
        let (mut node, skip) = v.enter(AstNode::Def(DefNode::Column(self)));
        if skip {
            return v.leave(node);
        }

        match &mut node {
            AstNode::Def(DefNode::Column(ref mut c)) => {
                for i in 0..c.options.len() {
                    // todo: we can not do this.
                    // let (n, ok) = c.options[i].accept(v);
                }
            }
            _ => { panic!() }
        }
        v.leave(node)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IndexType {
    Invalid,
    BTree,
    Hash,
    RTree,
}

impl Into<String> for IndexType {
    fn into(self) -> String {
        match self {
            IndexType::Invalid => "".to_string(),
            IndexType::BTree => "BTREE".to_string(),
            IndexType::Hash => "HASH".to_string(),
            IndexType::RTree => "RTREE".to_string(),
        }
    }
}

pub enum IndexVisibility {
    Default,
    Visible,
    Invisible,
}

/// IndexOption is the index options.
///
///     KEY_BLOCK_SIZE [=] value
/// |   index_type
/// |   WITH PARSER parser_name
/// |   COMMENT 'string'
pub struct IndexOption {
    key_block_size: u64,
    index_type: IndexType,
    comment: String,
    parser_name: String,
    visibility: IndexVisibility,
}

impl<V: Visitor> Node<V> for IndexOption {
    fn accept(self, v: &mut V) -> (AstNode, bool) {
        let (node, _) = v.enter(AstNode::Option(OptionNode::Index(self)));
        v.leave(node)
    }
}

pub enum ColumnOptionType {
    NoOption,
    PrimaryKey,
    NotNull,
    AutoIncrement,
    DefaultValue,
    UniqKey,
    Null,
    OnUpdate,
    FullText,
    Comment,
    Generated,
    Reference,
    Collate,
    Check,
    ColumnFormat,
    Storage,
    AutoRandom,
}

pub struct ColumnOption {
    option_type: ColumnOptionType,
    expression: ExpressionNode,
    stored: bool,
    value: String,
    auto_random_bit_length: i32,
    enforced: bool,
}

impl<V> Node<V> for ColumnOption
    where
        V: Visitor,
{
    fn accept(self, v: &mut V) -> (AstNode, bool) {
        let (node, skip) = v.enter(AstNode::Option(OptionNode::Column(self)));
        if skip {
            return v.leave(node);
        }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::deprecated::ast::{
        AstNode, CheckExprNode, CreateDatabaseStmtNode, DropDatabaseStmtNode, Node,
    };
    use crate::tiny_planner::test::CheckVisitor;

    struct TestCase {
        node: AstNode,
        expected_enter_count: i32,
        expected_leave_count: i32,
    }

    impl TestCase {
        fn new(node: AstNode, enter: i32, leave: i32) -> Self {
            TestCase {
                node,
                expected_enter_count: enter,
                expected_leave_count: leave,
            }
        }
    }

    #[test]
    fn ddl_visitor_cover() {
        let mut ce = CheckExprNode::default();
        let cases = vec![
            TestCase::new(
                AstNode::CreateDatabaseStmt(CreateDatabaseStmtNode::default()),
                0,
                0,
            ),
            TestCase::new(
                AstNode::DropDatabaseStmt(DropDatabaseStmtNode::default()),
                0,
                0,
            ),
        ];
        let mut v = CheckVisitor {};
        for case in cases {
            ce.reset();
            case.node.accept(&mut v);
            assert_eq!(case.expected_enter_count, ce.enter_count());
            assert_eq!(case.expected_leave_count, ce.leave_count());
        }
    }
}
