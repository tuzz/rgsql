use crate::*;

#[derive(Clone, Debug)]
pub struct Projection {
    pub column_identifier: Identifier,
    pub table_identifier: Identifier,
}

impl Projection {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        Identifier::parser()
            .then_ignore(whitespace().at_least(1))
            .then_ignore(just("FROM"))
            .then_ignore(whitespace().at_least(1))
            .then(Identifier::parser())
            .map(|(column_identifier, table_identifier)| Self { column_identifier, table_identifier })
    }
}
