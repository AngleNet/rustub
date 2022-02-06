use crate::types::{CmpBool, Type, TypeId};
use crate::types::value::Value;

pub struct BooleanType {
    type_id: TypeId,
}

impl BooleanType {
    pub(crate) fn new() -> Self {
        BooleanType {
            type_id: TypeId::Boolean
        }
    }
}


impl Type for BooleanType {

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

}
