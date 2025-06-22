use crate::*;

#[derive(Debug, Default)]
pub struct CreateTable {
    name: Identifier,
    column_definitions: Vec<(Identifier, DataType)>,
}

impl CreateTable {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        just("CREATE")
            .ignore_then(whitespace().at_least(1))
            .ignore_then(just("TABLE"))
            .ignore_then(whitespace().at_least(1))
            .ignore_then(Identifier::parser())
            .then(Self::column_definitions().delimited_by(just('('), just(')')).padded())
            .map(|(name, column_definitions)| Self { name, column_definitions })
    }

    fn column_definitions<'a>() -> impl Parser<'a, &'a str, Vec<(Identifier, DataType)>, RichError<'a>> {
        Identifier::parser()
            .then_ignore(whitespace().at_least(1))
            .then(DataType::parser())
            .padded()
            .separated_by(just(','))
            .collect()
    }
}
