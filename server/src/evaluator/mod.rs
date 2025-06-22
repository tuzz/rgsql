mod create_table;
mod drop_table;
mod query;
mod select;

use crate::*;
pub use create_table::*;
pub use drop_table::*;
pub use query::*;
pub use select::*;

pub trait Evaluate {
    fn evaluate(self, database: &mut Database) -> QueryResult;
}
