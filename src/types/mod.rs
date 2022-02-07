mod boolean;
mod integer;
/// Rustub supports ordinary types, such as boolean, tiny int, small int, int, big int, varchar,
/// timestamp, decimal. Every value has a specific type and a bunch of compatible types which can be
/// casted to.
mod value;
mod varchar;

pub use crate::types::boolean::BooleanType;
use crate::types::value::Value;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TypeId {
    Invalid,
    Boolean,
    TinyInt,
    SmallInt,
    Integer,
    BigInt,
    Decimal,
    VarChar,
    Timestamp,
}

impl ToString for TypeId {
    fn to_string(&self) -> String {
        match self {
            TypeId::Invalid => "INVALID".to_string(),
            TypeId::Boolean => "BOOLEAN".to_string(),
            TypeId::TinyInt => "TINYINT".to_string(),
            TypeId::SmallInt => "SMALLINT".to_string(),
            TypeId::Integer => "INTEGER".to_string(),
            TypeId::BigInt => "BIGINT".to_string(),
            TypeId::Decimal => "DECIMAL".to_string(),
            TypeId::VarChar => "VARCHAR".to_string(),
            TypeId::Timestamp => "TIMESTAMP".to_string(),
        }
    }
}

impl From<u8> for TypeId {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            1 => TypeId::Boolean,
            2 => TypeId::TinyInt,
            3 => TypeId::SmallInt,
            4 => TypeId::Integer,
            5 => TypeId::BigInt,
            6 => TypeId::Decimal,
            7 => TypeId::VarChar,
            8 => TypeId::Timestamp,
            _ => TypeId::Invalid,
        }
    }
}

impl From<TypeId> for u8 {
    #[inline]
    fn from(t: TypeId) -> Self {
        match t {
            TypeId::Boolean => 1,
            TypeId::TinyInt => 2,
            TypeId::SmallInt => 3,
            TypeId::Integer => 4,
            TypeId::BigInt => 5,
            TypeId::Decimal => 6,
            TypeId::VarChar => 7,
            TypeId::Timestamp => 8,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CmpBool {
    False,
    True,
    Null,
}

impl From<bool> for CmpBool {
    #[inline]
    fn from(v: bool) -> Self {
        if v {
            CmpBool::True
        } else {
            CmpBool::False
        }
    }
}

lazy_static! {
    // Cache for type instances
    static ref TYPE_INSTANCES: [Box<dyn Type + Sync>; 1] = [Box::new(BooleanType::new())];
}

pub trait Type {
    // Comparison functions
    //
    // NOTE:
    // We could get away with only CompareLessThan() being purely virtual, since
    // the remaining comparison functions can derive their logic from
    // CompareLessThan(). For example:
    //
    //    CompareEquals(o) = !CompareLessThan(o) && !o.CompareLessThan(this)
    //    CompareNotEquals(o) = !CompareEquals(o)
    //    CompareLessThanEquals(o) = CompareLessThan(o) || CompareEquals(o)
    //    CompareGreaterThan(o) = !CompareLessThanEquals(o)
    //    ... etc. ...
    //
    // We don't do this for two reasons:
    // (1) The redundant calls to CompareLessThan() may be a performance problem,
    //     and since Value is a core component of the execution engine, we want to
    //     make it as performant as possible.
    // (2) Keep the interface consistent by making all functions purely virtual.
    fn compare_equal(&self, left: &Value, right: &Value) -> CmpBool {
        unimplemented!()
    }

    fn compare_not_equal(&self, left: &Value, right: &Value) -> CmpBool {
        unimplemented!()
    }

    fn compare_less_than(&self, left: &Value, right: &Value) -> CmpBool {
        unimplemented!()
    }

    fn compare_less_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        unimplemented!()
    }

    fn compare_greater_than(&self, left: &Value, right: &Value) -> CmpBool {
        unimplemented!()
    }

    fn compare_greater_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        unimplemented!()
    }

    // Other mathematical functions

    fn add(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn subtract(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn multiply(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn divide(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn modulo(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn min(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn max(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn sqrt(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    fn operate_null(&self, left: &Value, right: &Value) -> Value {
        unimplemented!()
    }

    #[inline]
    fn is_zero(&self, val: &Value) -> bool {
        unimplemented!()
    }

    #[inline]
    fn is_inlined(&self, val: &Value) -> bool {
        unimplemented!()
    }

    fn to_string(&self, val: &Value) -> String {
        unimplemented!()
    }

    fn serialize(&self, val: &Value, buf: &mut [u8]) {
        unimplemented!()
    }

    fn deserialize(&self, buf: &[u8]) -> Value {
        unimplemented!()
    }

    fn cast_as(&self, val: &Value, typ: TypeId) -> Value {
        unimplemented!()
    }

    fn copy(&self, v: &Value) -> Value {
        unimplemented!()
    }
}

pub fn type_instance<'a>(typ: TypeId) -> &'a Box<dyn Type + Sync> {
    let idx: u8 = typ.into();
    &TYPE_INSTANCES[idx as usize]
}
