mod aggregation;
mod column;
mod constant;
mod scalar;
mod schema;

pub use aggregation::*;
pub use column::*;
pub use constant::*;
pub use scalar::*;
pub use schema::*;

/// Expression represents all scalar expression in SQL, not including aggregation function. There are
/// two ways to evaluate a expression: row-based and chunk-based (vectorized). In a row-based manner,
/// the expression actually transforms a input row into a single column of value. In a chunk-based
/// manner, the expression transforms the whole chunk into another chunk of column.
pub enum Expression {
    Column(Column),
    Constant(Constant),
    ScalarFunc(ScalarFunc),
}
