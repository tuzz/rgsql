use crate::*;

impl Evaluate for CreateTable {
    fn evaluate(self, database: &mut Database) -> QueryResult {
        let mut table = Table::new(self.identifier.name);
        let column_results = self.column_definitions.into_iter().map(|(identifier, data_type)| {
            table.add_column(identifier.name, ColumnType::from_data_type(data_type))
        });

        let columns_result = column_results.collect::<Result<Vec<_>, _>>();
        if let Err(error_message) = columns_result {
            let error_type = "validation_error".to_string();
            return QueryResult::Error(ErrorResult { error_type, error_message });
        }

        let table_result = database.add_table(table);
        if let Err(error_message) = table_result {
            let error_type = "validation_error".to_string();
            return QueryResult::Error(ErrorResult { error_type, error_message });
        }


        QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] })
    }
}
