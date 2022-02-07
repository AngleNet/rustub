use crate::types::{type_instance, CmpBool, TypeId};
use std::env::var;

/// Comparable matrix
/// ---------------------------------------------------------------------------
/// |      right|tiny | small | integer | big | decimal | timestamp | varchar |
/// | left      | int |  int  |         | int |         |           |         |
/// ---------------------------------------------------------------------------
/// | tiny int  |  Y  |   Y   |    Y    |  Y  |    Y    |     Y     |
/// | -----------------------------------------------------------------
/// | small int |
/// |------------------------------------------------------------------
/// | integer   |
/// |-----------------------------------------------------------------
/// | big int   |
/// |------------------------------------------------------------
/// | decimal   |
/// |-------------------------------------------------
/// | timestamp |
/// |-------------------
/// | varchar   |
/// |---------------------------------------------------------------
/// Casting matrix
///
///

// todo: what if a value is null?

#[derive(Debug)]
enum Val {
    Boolean(i8),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Decimal(f64),
    Timestamp(u64),
    Varlen(Vec<u8>),
    Null,
}

pub struct Value {
    value: Val,
    type_id: TypeId,
}

impl Value {
    #[inline]
    pub fn with_tinyint(i: i8) -> Self {
        Value {
            value: Val::TinyInt(i),
            type_id: TypeId::TinyInt,
        }
    }

    #[inline]
    pub fn with_boolean(i: i8) -> Self {
        Value {
            value: Val::Boolean(i),
            type_id: TypeId::Boolean,
        }
    }

    #[inline]
    pub fn with_null(typ: TypeId) -> Self {
        Value {
            value: Val::Null,
            type_id: typ,
        }
    }

    #[inline]
    pub fn with_varchar(var: &[u8]) -> Self {
        Value {
            value: Val::Varlen(Vec::from(var)),
            type_id: TypeId::VarChar,
        }
    }

    #[inline]
    pub fn as_boolean(&self) -> i8 {
        match &self.value {
            Val::Boolean(v) => *v,
            _ => panic!(""),
        }
    }

    pub fn as_tinyint(&self) -> i8 {
        match &self.value {
            Val::TinyInt(v) => *v,
            _ => panic!(""),
        }
    }

    pub fn as_integer(&self) -> i32 {
        match &self.value {
            Val::Int(v) => *v,
            _ => panic!(""),
        }
    }

    pub fn as_bigint(&self) -> i64 {
        match &self.value {
            Val::BigInt(v) => *v,
            _ => panic!(""),
        }
    }

    #[inline]
    pub fn as_varchar(&self) -> &[u8] {
        match &self.value {
            Val::Varlen(ref v) => &v[..],
            _ => panic!(""),
        }
    }

    #[inline]
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn check_integer(&self) -> bool {
        match self.type_id() {
            TypeId::TinyInt | TypeId::SmallInt | TypeId::Integer | TypeId::BigInt => true,
            _ => false,
        }
    }

    pub fn check_comparable(&self, o: &Value) -> bool {
        match self.type_id() {
            TypeId::Boolean => o.type_id() == TypeId::Boolean || o.type_id() == TypeId::VarChar,
            TypeId::TinyInt
            | TypeId::SmallInt
            | TypeId::Integer
            | TypeId::BigInt
            | TypeId::Decimal => match o.type_id() {
                TypeId::TinyInt
                | TypeId::SmallInt
                | TypeId::Integer
                | TypeId::BigInt
                | TypeId::Decimal
                | TypeId::VarChar => true,
                _ => false,
            },
            TypeId::VarChar => {
                // Anything can be cast to a string!
                true
            }
            // todo: what happens with timestamp?
            _ => false,
        }
    }

    #[inline]
    pub fn cast_as(&self, typ: TypeId) -> Value {
        type_instance(self.type_id()).cast_as(self, typ)
    }

    #[inline]
    pub fn compare_equal(&self, v: &Value) -> CmpBool {
        type_instance(self.type_id()).compare_equal(self, v)
    }

    #[inline]
    pub fn compare_not_equal(&self, v: &Value) -> CmpBool {
        type_instance(self.type_id()).compare_not_equal(self, v)
    }

    #[inline]
    pub fn compare_less_than(&self, v: &Value) -> CmpBool {
        type_instance(self.type_id()).compare_less_than(self, v)
    }

    #[inline]
    pub fn compare_less_than_equal(&self, v: &Value) -> CmpBool {
        type_instance(self.type_id()).compare_less_than_equal(self, v)
    }

    #[inline]
    pub fn compare_greater_than(&self, v: &Value) -> CmpBool {
        type_instance(self.type_id()).compare_greater_than(self, v)
    }

    #[inline]
    pub fn compare_greater_than_equal(&self, v: &Value) -> CmpBool {
        type_instance(self.type_id()).compare_greater_than_equal(self, v)
    }

    #[inline]
    pub fn add(&self, v: &Value) -> Value {
        type_instance(self.type_id()).add(self, v)
    }

    #[inline]
    pub fn subtract(&self, v: &Value) -> Value {
        type_instance(self.type_id()).subtract(self, v)
    }

    #[inline]
    pub fn multiply(&self, v: &Value) -> Value {
        type_instance(self.type_id()).multiply(self, v)
    }

    #[inline]
    pub fn modulo(&self, v: &Value) -> Value {
        type_instance(self.type_id()).modulo(self, v)
    }

    #[inline]
    pub fn min(&self, v: &Value) -> Value {
        type_instance(self.type_id()).min(self, v)
    }

    #[inline]
    pub fn max(&self, v: &Value) -> Value {
        type_instance(self.type_id()).max(self, v)
    }

    #[inline]
    pub fn sqrt(&self, v: &Value) -> Value {
        type_instance(self.type_id()).sqrt(self, v)
    }

    #[inline]
    pub fn operate_null(&self, v: &Value) -> Value {
        type_instance(self.type_id()).operate_null(self, v)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        type_instance(self.type_id()).is_zero(self)
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        match self.value {
            Val::Null => true,
            _ => false,
        }
    }

    #[inline]
    pub fn serialize(&self, buf: &mut [u8]) {
        type_instance(self.type_id()).serialize(self, buf);
    }

    #[inline]
    pub fn deserialize(typ: TypeId, buf: &[u8]) -> Value {
        type_instance(typ).deserialize(buf)
    }

    #[inline]
    pub fn to_string(&self) -> String {
        type_instance(self.type_id()).to_string(self)
    }

    #[inline]
    pub fn copy(&self) -> Value {
        type_instance(self.type_id()).copy(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
