mod create_table;
mod drop_table;
mod query;
mod select;

pub use create_table::*;
pub use drop_table::*;
pub use query::*;
pub use select::*;

pub trait Evaluate {
    fn evaluate(self) -> crate::QueryResult;
}
