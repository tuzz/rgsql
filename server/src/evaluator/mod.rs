mod create_table;
mod drop_table;
mod expression;
mod insert_into;
mod query;
mod select;

pub use create_table::*;
pub use drop_table::*;
pub use expression::*;
pub use insert_into::*;
pub use query::*;
pub use select::*;

pub trait Evaluate {
    type Output;

    fn evaluate(self, database: &mut crate::Database) -> Self::Output;
}
