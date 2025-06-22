use crate::*;

#[derive(Debug, Default)]
pub struct DropTable {
    pub if_exists: bool,
    pub identifier: Identifier,
}

impl DropTable {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        just("DROP")
            .ignore_then(whitespace().at_least(1))
            .ignore_then(just("TABLE"))
            .ignore_then(whitespace().at_least(1))
            .ignore_then(Self::if_exists_parser())
            .then(Identifier::parser())
            .map(|(if_exists, identifier)| DropTable { if_exists, identifier })
    }

    fn if_exists_parser<'a>() -> impl Parser<'a, &'a str, bool, RichError<'a>> {
        just("IF")
            .ignore_then(whitespace().at_least(1))
            .ignore_then(just("EXISTS"))
            .then_ignore(whitespace().at_least(1))
            .padded()
            .or_not()
            .map(|option| option.is_some())
    }
}
