use crate::*;

#[derive(Clone, Debug)]
pub enum DataType {
    Integer,
    Boolean,
}

impl DataType {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        choice((
            just("INTEGER").to(DataType::Integer),
            just("BOOLEAN").to(DataType::Boolean),
        ))
    }
}
