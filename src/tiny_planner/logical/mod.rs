mod builder;
mod plan;

pub use builder::*;
pub use plan::*;

pub trait Plan {
    fn schema(&mut self) -> &mut PlanSchema;
}

pub enum LogicalPlan {
    DataSource(DataSource),
}

impl Plan for LogicalPlan {
    fn schema(&mut self) -> &mut PlanSchema {
        match self {
            LogicalPlan::DataSource(s) => s.schema(),
        }
    }
}
