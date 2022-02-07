use crate::types::value::Value;
use crate::types::{CmpBool, Type, TypeId};

struct TinyIntType {}

impl Type for TinyIntType {
    fn compare_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.check_integer());
        assert!(left.check_comparable(right));
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

    fn add(&self, left: &Value, right: &Value) -> Value {
        assert!(left.check_integer());
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return left.operate_null(right);
        }
        todo!()
    }

    fn subtract(&self, left: &Value, right: &Value) -> Value {
        todo!()
    }

    fn multiply(&self, left: &Value, right: &Value) -> Value {
        todo!()
    }

    fn divide(&self, left: &Value, right: &Value) -> Value {
        todo!()
    }

    fn modulo(&self, left: &Value, right: &Value) -> Value {
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

pub struct IntegerType {}

macro_rules! integer_compare_func {
    ($left: ident $op: tt $right: ident) => {
        match $left.type_id() {
            TypeId::TinyInt => {
                return ($left.as_integer() $op $right.as_tinyint() as i32).into();
            }
            TypeId::SmallInt => {
                return ($left.as_integer() $op $right.as_smallint() as i32).into();
            }
            TypeId::Integer => {
                return ($left.as_integer() $op $right.as_integer()).into();
            }
            TypeId::BigInt => {
                return ($left.as_integer() as i64 $op $right.as_bigint()).into();
            }
            TypeId::Decimal => {
                return ($left.as_integer() as f64 $op $right.as_decimal()).into();
            }
            TypeId::VarChar => {
                let v = $right.cast_as(TypeId::Integer);
                return ($left.as_integer() $op v.as_integer()).into();
            }
            _ => {panic!("type error")}
        }
    };
}

impl Type for IntegerType {
    fn compare_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.type_id() == TypeId::Integer);
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        // Retrieve the right value from right and compare it with left
        integer_compare_func!(left == right);
    }

    fn compare_not_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.type_id() == TypeId::Integer);
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        // Retrieve the right value from right and compare it with left
        integer_compare_func!(left != right);
    }

    fn compare_less_than(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.type_id() == TypeId::Integer);
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        // Retrieve the right value from right and compare it with left
        integer_compare_func!(left <= right);
    }

    fn compare_less_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.type_id() == TypeId::Integer);
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        // Retrieve the right value from right and compare it with left
        integer_compare_func!(left <= right);
    }

    fn compare_greater_than(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.type_id() == TypeId::Integer);
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        // Retrieve the right value from right and compare it with left
        integer_compare_func!(left > right);
    }

    fn compare_greater_than_equal(&self, left: &Value, right: &Value) -> CmpBool {
        assert!(left.type_id() == TypeId::Integer);
        assert!(left.check_comparable(right));
        if left.is_null() || right.is_null() {
            return CmpBool::Null;
        }
        // Retrieve the right value from right and compare it with left
        integer_compare_func!(left >= right);
    }

    fn add(&self, left: &Value, right: &Value) -> Value {
        todo!()
    }

    fn subtract(&self, left: &Value, right: &Value) -> Value {
        todo!()
    }

    fn multiply(&self, left: &Value, right: &Value) -> Value {
        todo!()
    }

    fn divide(&self, left: &Value, right: &Value) -> Value {
        todo!()
    }

    fn modulo(&self, left: &Value, right: &Value) -> Value {
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
