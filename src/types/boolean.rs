use crate::types::value::Value;
use crate::types::{CmpBool, Type, TypeId};

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
        todo!()
    }

    fn compare_less_than(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }

    fn compare_less_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }

    fn compare_greater_than(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }

    fn compare_greater_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        todo!()
    }

    #[inline]
    fn is_inlined(&self, val: &Value) -> bool {
        true
    }

    fn to_string(&self, val: &Value) -> String {
        todo!()
    }

    fn serialize(&self, val: &Value, buf: &mut [u8]) {
        todo!()
    }

    fn deserialize(&self, buf: &[u8]) -> Value {
        todo!()
    }

    fn cast_as(&self, val: &Value, typ: TypeId) -> Value {
        todo!()
    }

    fn copy(&self, v: &Value) -> Value {
        todo!()
    }
}
