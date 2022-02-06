/// Rustub supports ordinary types, such as boolean, tiny int, small int, int, big int, varchar,
/// timestamp, decimal. Every value has a specific type and a bunch of compatible types which can be
/// casted to.
mod value;
mod boolean;

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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CmpBool {
    False,
    True,
    Null,
}

lazy_static! {
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

    fn deserialize_from(&self, buf: &[u8]) -> Value {
        unimplemented!()
    }

    fn cast_as(&self, val: &Value, typ: TypeId) -> Value {
        unimplemented!()
    }
}
