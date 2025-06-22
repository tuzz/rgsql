use crate::*;

impl Evaluate for Query {
    type Output = QueryResult;

    fn evaluate(self, database: &mut Database) -> Self::Output {
        match self {
            Self::Select(select) => select.evaluate(database),
            Self::CreateTable(create_table) => create_table.evaluate(database),
            Self::DropTable(drop_table) => drop_table.evaluate(database),
            Self::InsertInto(insert_into) => insert_into.evaluate(database),
        }
    }
}
