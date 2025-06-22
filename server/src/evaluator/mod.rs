mod create_table;
mod drop_table;
mod expression;
mod insert_into;
mod query;
mod select;

pub trait Evaluate {
    type Output;

    fn evaluate(self, database: &mut crate::Database) -> Self::Output;
}
