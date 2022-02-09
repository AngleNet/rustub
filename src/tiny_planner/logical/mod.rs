mod builder;
mod node;

pub use builder::*;
pub use node::*;

pub enum Plan {
    DataSource(DataSource),
}

impl Plan {
    pub fn schema(&mut self) -> &mut PlanSchema {
        match self {
            Plan::DataSource(s) => s.schema(),
        }
    }
}
