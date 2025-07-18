use crate::*;

#[derive(Clone)]
pub struct Row {
    values: Vec<RowValue>,
}

#[derive(Clone, Debug)]
pub enum RowValue {
    Integer(i64),
    Boolean(bool),
}

impl Row {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn with_default_values(column_types: &[ColumnType]) -> Self {
        let mut row = Self::new();

        for &column_type in column_types {
            row.add_value(column_type.default_row_value());
        }

        row
    }

    pub fn add_value(&mut self, value: RowValue) {
        self.values.push(value);
    }

    pub fn get_value(&self, column_index: usize) -> &RowValue {
        &self.values[column_index]
    }

    pub fn set_value(&mut self, column_index: usize, value: RowValue) {
        self.values[column_index] = value;
    }
}

impl RowValue {
    pub fn from(value: Literal) -> Self {
        match value {
            Literal::Integer(value) => RowValue::Integer(value),
            Literal::Boolean(value) => RowValue::Boolean(value),
        }
    }
}
