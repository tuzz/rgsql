use crate::*;

impl Evaluate for DropTable {
    type Output = QueryResult;

    fn evaluate(self, database: &mut Database) -> Self::Output {
        let table_index = database.table_index(&self.identifier.name);

        match (table_index, self.if_exists) {
            (Some(index), _) => {
                database.remove_table(index);
                QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] })
            },
            (None, true) => QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] }),
            (None, false) => {
                let error_type = "validation_error".to_string();
                let error_message = format!("Table '{}' does not exist.", self.identifier.name);
                QueryResult::Error(ErrorResult { error_type, error_message })
            },
        }
    }
}
