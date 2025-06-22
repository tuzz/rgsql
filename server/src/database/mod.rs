mod table;
mod row;

pub use table::*;
pub use row::*;

pub struct Database {
    tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Self {
        Self { tables: vec![] }
    }

    pub fn table_index(&self, name: &str) -> Option<usize> {
        self.tables.iter().position(|table| table.name() == name)
    }

    pub fn table(&self, table_index: usize) -> &Table {
        &self.tables[table_index]
    }

    pub fn table_mut(&mut self, table_index: usize) -> &mut Table {
        &mut self.tables[table_index]
    }

    pub fn add_table(&mut self, table: Table) -> Result<(), String> {
        if self.tables.iter().any(|t| t.name() == table.name()) {
            Err(format!("Table '{}' already exists", table.name()))
        } else {
            self.tables.push(table);
            Ok(())
        }
    }

    pub fn remove_table(&mut self, table_index: usize) -> Table {
        self.tables.swap_remove(table_index)
    }
}
