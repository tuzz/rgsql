use crate::*;

#[derive(Debug)]
pub struct SelectList {
    pub expressions: Vec<Expression>,
    pub aliases: Vec<Option<Identifier>>,
}

impl SelectList {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        Expression::parser()
            .then(Self::alias_parser())
            .separated_by(just(',').padded())
            .collect()
            .map(|tuples: Vec<_>| {
                let (expressions, aliases) = tuples.into_iter().unzip();
                SelectList { expressions, aliases }
            })
    }

    fn alias_parser<'a>() -> impl Parser<'a, &'a str, Option<Identifier>, RichError<'a>> {
        whitespace().at_least(1)
            .ignore_then(just("AS"))
            .ignore_then(whitespace().at_least(1))
            .ignore_then(Identifier::parser())
            .or_not()
            .map(|option| option)
    }
}
