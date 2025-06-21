use crate::*;

#[derive(Debug)]
pub struct SelectList {
    pub literals: Vec<Literal>,
    pub aliases: Vec<Identifier>,
}

impl SelectList {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, SelectList, RichError<'a>> {
        Literal::parser()
            .then(Self::alias_parser())
            .separated_by(just(',').padded())
            .collect()
            .map(|tuples: Vec<_>| {
                let (literals, aliases) = tuples.into_iter().unzip();
                SelectList { literals, aliases }
            })
    }

    fn alias_parser<'a>() -> impl Parser<'a, &'a str, Identifier, RichError<'a>> {
        whitespace().at_least(1)
            .ignore_then(just("AS"))
            .ignore_then(whitespace().at_least(1))
            .ignore_then(Identifier::parser())
            .or_not()
            .map(|alias| alias.unwrap_or_default())
    }
}
