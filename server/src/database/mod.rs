mod table;

pub use table::*;

pub struct Database {
    tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Self {
        Self { tables: vec![] }
    }

    pub fn add_table(&mut self, name: String) -> Result<&mut Table, String> {
        if self.tables.iter().any(|table| table.name() == name) {
            Err(format!("Table '{}' already exists", name))
        } else {
            self.tables.push(Table::new(name));
            Ok(self.tables.last_mut().unwrap())
        }
    }

    pub fn remove_table(&mut self, name: String) -> Result<Table, String> {
        let index = self.tables.iter().position(|table| table.name() == name);

        if let Some(index) = index {
            Ok(self.tables.swap_remove(index))
        } else {
            Err(format!("Table '{}' does not exist", name))
        }
    }
}
