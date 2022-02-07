use crate::catalog::schema::Schema;
use crate::common::error::*;
use crate::concurrency::Transaction;
use crate::storage::index::Index;
use crate::storage::table::TableHeap;
use std::error::Error;

mod column;
mod schema;

pub type TableOid = u32;
pub type ColumnOid = u32;
pub type IndexOid = u32;

/// The TableInfo maintains metadata about a table
pub struct TableInfo {
    /// The table schema
    schema: Schema,
    /// The table name
    name: String,
    /// The table heap
    table: Box<TableHeap>,
    /// The table OID
    oid: TableOid,
}

/// The IndexInfo maintains metadata about an index.
pub struct IndexInfo {
    /// The schema for the index key
    key_schema: Schema,
    /// The index name
    name: String,
    /// The index itself
    index: Box<dyn Index>,
    oid: IndexOid,
    /// The name of the table on which the index is created
    table_name: String,
    /// The size of the index key, in bytes
    key_size: usize,
}

/// The Catalog is a non-persistent catalog that is designed for use by executors within the DBMS
/// execution engine. It handles table creation, table lookup, index creation and index lookup
pub struct Catalog {}

impl Catalog {
    pub fn create_table(txn: Transaction, name: String, schema: Schema) -> Result<TableInfo> {
        todo!()
    }

    pub fn get_table(name: String) -> Option<TableInfo> {
        todo!()
    }

    // todo: create index, get index
}
