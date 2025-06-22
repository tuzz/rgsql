use crate::*;

impl Evaluate for Query {
    fn evaluate(self, database: &mut Database) -> QueryResult {
        match self {
            Self::Select(select) => select.evaluate(database),
            Self::CreateTable(create_table) => create_table.evaluate(database),
            Self::DropTable(drop_table) => drop_table.evaluate(database),
        }
    }
}
