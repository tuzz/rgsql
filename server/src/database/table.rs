use crate::*;

pub struct Table {
    name: String,
    column_names: Vec<String>,
    column_types: Vec<ColumnType>,
    rows: Vec<Row>,
}

#[derive(Clone, Copy, Debug)]
pub enum ColumnType {
    Integer,
    Boolean,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self { name, column_names: vec![], column_types: vec![], rows: vec![] }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn column_types(&self) -> &[ColumnType] {
        &self.column_types
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn num_columns(&self) -> usize {
        self.column_names.len()
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.column_names.iter().position(|n| n == name)
    }

    pub fn column_type(&self, column_index: usize) -> ColumnType {
        self.column_types[column_index]
    }

    pub fn add_column(&mut self, name: String, column_type: ColumnType) -> Result<(), String> {
        if self.column_names.iter().any(|n| n == &name) {
            Err(format!("Column '{}' already exists in table '{}'", name, self.name))
        } else {
            self.column_names.push(name);
            self.column_types.push(column_type);

            for row in &mut self.rows {
                row.add_value(column_type.default_row_value());
            }

            Ok(())
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }
}

impl ColumnType {
    pub fn from_data_type(data_type: DataType) -> Self {
        match data_type {
            DataType::Integer => ColumnType::Integer,
            DataType::Boolean => ColumnType::Boolean,
        }
    }

    pub fn checked_row_value(&self, literal: Literal) -> Result<RowValue, String> {
        match (self, &literal) {
            (ColumnType::Integer, Literal::Integer(value)) => Ok(RowValue::Integer(*value)),
            (ColumnType::Boolean, Literal::Boolean(value)) => Ok(RowValue::Boolean(*value)),
            _ => Err(format!("{literal:?} is invalid for {self:?}")),
        }
    }

    pub fn default_row_value(&self) -> RowValue {
        match self {
            ColumnType::Integer => RowValue::Integer(0),
            ColumnType::Boolean => RowValue::Boolean(false),
        }
    }
}
