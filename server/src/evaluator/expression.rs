use crate::*;

impl Evaluate for Expression {
    type Output = Result<Vec<Vec<RowValue>>, String>;

    fn evaluate(self, database: &mut Database) -> Self::Output {
        match self {
            Self::Literal(literal) => Ok(vec![vec![RowValue::from(literal)]]),
            Self::Projection(projection) => {
                let table_name = &projection.table_identifier.name;
                let column_names = projection.column_identifiers.iter().map(|identifier| &identifier.name);

                let table_index = database.table_index(table_name);
                if table_index.is_none() { return Err(format!("Table '{table_name}' does not exist.")); }

                let table = database.table(table_index.unwrap());
                for column_name in column_names.clone() {
                    if table.column_index(column_name).is_none() {
                        return Err(format!("Column '{column_name}' does not exist in table '{table_name}'."));
                    }
                }

                let row_values = column_names.map(|column_name| {
                    let column_index = table.column_index(column_name).unwrap();
                    table.rows().iter().map(|row| row.get_value(column_index).clone()).collect()
                }).collect::<Vec<Vec<_>>>();

                Ok(row_values)
            },
        }
    }
}
