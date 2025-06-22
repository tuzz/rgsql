use crate::*;

#[derive(Debug)]
pub struct Query {
    pub select: Select,
}

pub type RichError<'a> = extra::Err<Rich<'a, char>>;

impl Query {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        Select::parser()
            .map(|select| Query { select })
            .then_ignore(just(';'))
    }
}
