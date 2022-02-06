use crate::types::TypeId;

enum Val {
    Boolean(i8),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Decimal(f64),
    Timestamp(u64),
    Varlen(Vec<u8>),
}

pub struct Value {
    value: Val,
    type_id: TypeId,
}


impl Value {
    #[inline]
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn check_integer(&self) -> bool {
        match self.type_id() {
            TypeId::TinyInt | TypeId::SmallInt | TypeId::Integer | TypeId::BigInt => true,
            _ => false
        }
    }

    pub fn check_comparable(&self, o: &Value) -> bool {
        match self.type_id() {
            TypeId::Boolean => {
                o.type_id() == TypeId::Boolean || o.type_id() == TypeId::VarChar
            }
            TypeId::TinyInt | TypeId::SmallInt | TypeId::Integer | TypeId::BigInt |
            TypeId::Decimal => {
                match o.type_id() {
                    TypeId::TinyInt | TypeId::SmallInt | TypeId::Integer | TypeId::BigInt
                    | TypeId::Decimal | TypeId::VarChar => {
                        true
                    }
                    _ => false
                }
            }
            TypeId::VarChar => {
                // Anything can be cast to a string!
                true
            }
            // todo: what happens with timestamp?
            _ => false
        }
    }
}
