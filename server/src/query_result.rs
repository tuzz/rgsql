use crate::*;

#[derive(Debug)]
pub enum QueryResult {
    Ok(SuccessResult),
    Error(ErrorResult),
}

#[derive(Debug)]
pub struct SuccessResult {
    pub rows: Vec<Vec<Literal>>,
    pub column_names: Vec<Identifier>,
}

#[derive(Debug)]
pub struct ErrorResult {
    pub error_type: String,
    pub error_message: String,
}

impl Serialize for QueryResult {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(3))?;

        match self {
            QueryResult::Ok(success_result) => {
                map.serialize_entry("status", "ok")?;
                map.serialize_entry("rows", &success_result.rows)?;
                map.serialize_entry("column_names", &success_result.column_names)?;
                map.end()
            },
            QueryResult::Error(error_result) => {
                map.serialize_entry("status", "error")?;
                map.serialize_entry("error_type", &error_result.error_type)?;
                map.serialize_entry("error_message", &error_result.error_message)?;
                map.end()
            },
        }
    }
}

impl Serialize for Literal {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Literal::Boolean(value) => serializer.serialize_bool(*value),
            Literal::Integer(value) => serializer.serialize_i64(*value),
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.name)
    }
}
