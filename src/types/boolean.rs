use std::fmt::format;
use bytes::{Buf, BufMut};
use crate::types::value::Value;
use crate::types::{CmpBool, Type, TypeId};
use crate::types::TypeId::VarChar;

pub struct BooleanType {
    type_id: TypeId,
}

impl BooleanType {
    pub(crate) fn new() -> Self {
        BooleanType {
            type_id: TypeId::Boolean,
        }
    }
}

impl Type for BooleanType {
    fn compare_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        (left.as_boolean() == right.cast_as(TypeId::Boolean).as_boolean()).into()
    }

    fn compare_not_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        (left.as_boolean() != right.cast_as(TypeId::Boolean).as_boolean()).into()
    }

    fn compare_less_than(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        (left.as_boolean() < right.cast_as(TypeId::Boolean).as_boolean()).into()
    }

    fn compare_less_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        (left.as_boolean() <= right.cast_as(TypeId::Boolean).as_boolean()).into()
    }

    fn compare_greater_than(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        (left.as_boolean() > right.cast_as(TypeId::Boolean).as_boolean()).into()
    }

    fn compare_greater_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        (left.as_boolean() >= right.cast_as(TypeId::Boolean).as_boolean()).into()
    }

    #[inline]
    fn is_inlined(&self, val: &Value) -> bool {
        true
    }

    fn to_string(&self, val: &Value) -> String {
        let v = val.as_boolean();
        if v == 1 {
            return "true".into();
        }
        if v == 0 {
            return "false".into();
        }
        return "boolean_null".into();
    }

    fn serialize(&self, val: &Value, mut buf: &mut [u8]) {
        buf.put_i8(val.as_boolean());
    }

    fn deserialize(&self, mut buf: &[u8]) -> Value {
        let v = buf.get_i8();
        Value::with_boolean(v)
    }

    fn cast_as(&self, val: &Value, typ: TypeId) -> Value {
        match typ {
            TypeId::Boolean => val.copy(),
            TypeId::VarChar => {
                if val.is_null() {
                    return Value::with_null(TypeId::VarChar);
                }
                return Value::with_varchar(val.as_varchar());
            }
            _ => {
                panic!("BOOLEAN is not coercable to {}", typ.to_string())
            }
        }
    }

    #[inline]
    fn copy(&self, v: &Value) -> Value {
        Value::with_boolean(v.as_boolean())
    }
}
