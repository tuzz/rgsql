use crate::*;

#[derive(Debug)]
pub enum QueryResult {
    Ok(SuccessResult),
    Error(ErrorResult),
}

#[derive(Debug)]
pub struct SuccessResult {
    pub rows: Vec<Vec<RowValue>>,
    pub column_names: Vec<String>,
}

#[derive(Debug)]
pub struct ErrorResult {
    pub error_type: String,
    pub error_message: String,
}

impl QueryResult {
    pub fn from_parse_errors<'a>(errors: &[Rich<'a, char>]) -> Self {
        let error_type = "parsing_error".to_string();
        let error_message = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");

        Self::Error(ErrorResult { error_type, error_message })
    }
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

impl Serialize for RowValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RowValue::Boolean(value) => serializer.serialize_bool(*value),
            RowValue::Integer(value) => serializer.serialize_i64(*value),
        }
    }
}
