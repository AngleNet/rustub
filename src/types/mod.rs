mod value;
mod numeric;

pub use numeric::*;
pub use value::*;

/// Supported types
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ColTypeId {
    Invalid,

    Boolean,

    TinyInt,

    BigInt,

    Decimal,

    VarChar,

    Timestamp,
}

impl From<usize> for ColTypeId {
    fn from(_: usize) -> Self {
        return ColTypeId::Invalid;
    }
}

impl From<ColTypeId> for usize {
    fn from(_: ColTypeId) -> Self {
        return 0;
    }
}

impl ColTypeId {
    pub fn type_size(&self) -> u32 {
        todo!()
    }

    pub fn min_value(&self) -> u32 {
        todo!()
    }

    pub fn max_value(&self) -> u32 {
        todo!()
    }
}

/// Static methods
pub fn new(id: ColTypeId) -> Box<dyn ColType> {
    todo!()
}


pub trait NumericColType {}

pub trait ColType: NumericColType {}


#[cfg(test)]
mod test {
    use crate::types::ColTypeId;

    #[test]
    fn test_col_type_id_converter() {
        assert_eq!(ColTypeId::from(0), ColTypeId::Invalid);
        assert_eq!(usize::from(ColTypeId::Invalid), 0);
    }
}
