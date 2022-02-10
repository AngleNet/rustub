pub enum FieldType {}

pub enum DataBox {
    Null,
    Int64(i64),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
    String(String),
    Bytes,
    BinaryLiteral,
    MysqlBit,
    MysqlSet,
    MysqlTime,
    Interface,
    MinNotNull,
    MaxValue,
    Raw,
    MysqlJson,
}
