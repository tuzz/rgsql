use crate::*;

#[derive(Debug, Default)]
pub struct DropTable {
    name: Identifier,
}

impl DropTable {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        just("DROP")
            .ignore_then(whitespace().at_least(1))
            .ignore_then(just("TABLE"))
            .ignore_then(whitespace().at_least(1))
            .ignore_then(Identifier::parser())
            .map(|name| DropTable { name })
    }
}
