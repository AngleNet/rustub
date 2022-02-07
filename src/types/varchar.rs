use crate::types::value::Value;
use crate::types::{CmpBool, Type, TypeId};

pub struct VarCharType {}

impl Type for VarCharType {
    fn compare_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.type_id() == TypeId::VarChar);
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        todo!()
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
