use crate::*;

impl Evaluate for CreateTable {
    fn evaluate(self) -> QueryResult {
        QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] })
    }
}
