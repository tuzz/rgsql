use crate::*;

impl Evaluate for Query {
    fn evaluate(self) -> QueryResult {
        match self {
            Self::Select(select) => select.evaluate(),
            Self::CreateTable(create_table) => create_table.evaluate(),
            Self::DropTable(drop_table) => drop_table.evaluate(),
        }
    }
}
