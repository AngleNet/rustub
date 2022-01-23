use crate::types::ColTypeId;

/// A value represents a view over SQL data stored in some materialized state. All values have a type
/// and comparison functions, but subclasses implement other type-specific functionality

pub struct ColValue {}

impl ColValue {
    pub fn new(typ: ColTypeId) -> ColValue {
        todo!()
    }

    pub fn with_tiny_int_or_boolean(typ: ColTypeId, i: u8) -> ColValue {
        todo!()
    }

    pub fn with_decimal(typ: ColTypeId, d: f64) -> ColValue {
        todo!()
    }
}
