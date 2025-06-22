use crate::*;

impl Evaluate for InsertInto {
    type Output = QueryResult;

    fn evaluate(self, database: &mut Database) -> Self::Output {
        let table_index = database.table_index(&self.identifier.name);
        if let None = table_index {
            let error_type = "validation_error".to_string();
            let error_message = format!("Table '{}' does not exist.", self.identifier.name);
            return QueryResult::Error(ErrorResult { error_type, error_message });
        }

        let table = database.table_mut(table_index.unwrap());
        let default_row = Row::with_default_values(table.column_types());
        let column_indexes = 0..table.num_columns(); // TODO: named columns
        let column_types = column_indexes.clone().map(|i| table.column_type(i));

        for values in self.values_list.iter() {
            if values.len() != column_indexes.len() {
                let error_type = "validation_error".to_string();
                let error_message = format!("Row contains {} values but there are {} columns.", values.len(), column_indexes.len());
                return QueryResult::Error(ErrorResult { error_type, error_message });
            }

            for (value, column_type) in values.iter().zip(column_types.clone()) {
                if let Err(error_message) = column_type.checked_row_value(*value) {
                    let error_type = "validation_error".to_string();
                    return QueryResult::Error(ErrorResult { error_type, error_message });
                }
            }
        }

        for values in self.values_list {
            let mut row = default_row.clone();

            for (column_index, value) in column_indexes.clone().zip(values) {
                let column_type = table.column_type(column_index);
                row.set_value(column_index, column_type.checked_row_value(value).unwrap());
            };

            table.add_row(row);
        }

        QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] })
    }
}
