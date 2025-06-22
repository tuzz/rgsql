use crate::*;

#[derive(Debug)]
pub struct InsertInto {
    pub identifier: Identifier,
    pub values_list: Vec<Vec<Literal>>,
}

impl InsertInto {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        just("INSERT")
            .ignore_then(whitespace().at_least(1))
            .ignore_then(just("INTO"))
            .ignore_then(whitespace().at_least(1))
            .ignore_then(Identifier::parser())
            .then_ignore(whitespace().at_least(1))
            .then_ignore(just("VALUES"))
            .then(Self::values_list_parser())
            .map(|(identifier, values_list)| InsertInto { identifier, values_list })
    }

    fn values_list_parser<'a>() -> impl Parser<'a, &'a str, Vec<Vec<Literal>>, RichError<'a>> {
        Self::values_parser()
            .separated_by(just(',').padded())
            .at_least(1)
            .collect()
    }

    fn values_parser<'a>() -> impl Parser<'a, &'a str, Vec<Literal>, RichError<'a>> {
        Literal::parser()
            .padded()
            .separated_by(just(','))
            .at_least(1)
            .collect()
            .delimited_by(just('('), just(')'))
    }
}
