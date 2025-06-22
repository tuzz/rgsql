use crate::*;

impl Evaluate for Select {
    fn evaluate(self, database: &mut Database) -> QueryResult {
        let rows = vec![self.select_list.literals];
        let column_names = self.select_list.aliases;

        QueryResult::Ok(SuccessResult { rows, column_names })
    }
}
