use crate::*;

pub struct Table {
    name: String,
    metadata: Vec<(String, ColumnType)>,
    rows: Vec<Vec<RowValue>>,
}

#[derive(Clone, Copy)]
pub enum ColumnType {
    Integer,
    Boolean,
}

pub enum RowValue {
    Integer(i64),
    Boolean(bool),
}

impl Table {
    pub fn new(name: String) -> Self {
        Self { name, metadata: vec![], rows: vec![] }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_column(&mut self, name: String, column_type: ColumnType) {
        self.metadata.push((name, column_type));

        for row in &mut self.rows {
            match &column_type {
                ColumnType::Integer => row.push(RowValue::Integer(0)),
                ColumnType::Boolean => row.push(RowValue::Boolean(false)),
            }
        }
    }
}

impl ColumnType {
    pub fn from_data_type(data_type: DataType) -> Self {
        match data_type {
            DataType::Integer => ColumnType::Integer,
            DataType::Boolean => ColumnType::Boolean,
        }
    }
}
