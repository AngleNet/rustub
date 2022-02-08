use super::*;

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
    fn accept(self, visitor: &mut V) -> (AstNode, bool) {
        let (node, _) = visitor.enter(AstNode::CreateDatabaseStmt(self));
        visitor.leave(node)
    }
}

/// Statement which drops a database and all tables in the database
pub struct DropDatabaseStmtNode {
    if_exists: bool,
    name: String,
}

impl<V: Visitor> Node<V> for DropDatabaseStmtNode {
    fn accept(self, visitor: &mut V) -> (AstNode, bool) {
        let (node, _) = visitor.enter(AstNode::DropDatabaseStmt(self));
        visitor.leave(node)
    }
}

pub struct CreateTableStmtNode {
    if_not_exists: bool,
    is_temporary: bool,
}

pub struct AlterTableStmtNode {}

pub struct DropTableStmtNode {}

pub struct CreateIndexStmtNode {}

pub struct AlterIndexStmtNode {}

pub struct DropIndexStmtNode {}


///////////////////////// misc

pub struct TableName {}

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
            IndexType::Invalid => { "".to_string() }
            IndexType::BTree => { "BTREE".to_string() }
            IndexType::Hash => { "HASH".to_string() }
            IndexType::RTree => { "RTREE".to_string() }
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
pub struct IndexOptionNode {
    key_block_size: u64,
    index_type: IndexType,
    comment: String,
    parser_name: String,
    visibility: IndexVisibility,
}

impl<V: Visitor> Node<V> for IndexOptionNode {
    fn accept(self, visitor: &mut V) -> (AstNode, bool) {
        let (node, _) = visitor.enter(AstNode::IndexOption(self));
        visitor.leave(node)
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

pub struct ColumnOptionNode {
    option_type: ColumnOptionType,
    expression: ExpressionNode,
    stored: bool,
    value: String,
    auto_random_bit_length: i32,
    enforced: bool,
}

impl<V> Node<V> for ColumnOptionNode where V: Visitor {
    fn accept(self, visitor: &mut V) -> (AstNode, bool) {
        let (node, skip) = visitor.enter(AstNode::ColumnOption(self));
        if skip {
            return visitor.leave(node);
        }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::tiny_planner::ast::{AstNode, CheckExprNode, CreateDatabaseStmtNode, Node};
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
        let ce = CheckExprNode::default();
        let cases = vec![
            TestCase::new(AstNode::CreateDatabaseStmt(CreateDatabaseStmtNode::default()), 0, 0)
        ];
        let mut v = CheckVisitor {};
        for case in cases {
            case.node.accept(&mut v);
            assert_eq!(case.expected_enter_count, ce.enter_count());
            assert_eq!(case.expected_leave_count, ce.leave_count());
        }
    }
}
