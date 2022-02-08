pub struct Column {}

/// KeyInfo stores the columns of one unique key or primary key.
pub type KeyInfo = Vec<Column>;

/// Schema stands for the row schema and unique key information get from input.
pub struct Schema {
    columns: Vec<Column>,
    // Including unique keys and primary keys
    keys: Vec<KeyInfo>,
}
