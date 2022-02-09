use std::sync::{Arc, RwLock};

pub struct Table {}

pub type TableRef = Arc<RwLock<Table>>;
