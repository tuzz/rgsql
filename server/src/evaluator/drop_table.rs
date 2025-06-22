use crate::*;

impl Evaluate for DropTable {
    fn evaluate(self) -> QueryResult {
        QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] })
    }
}
