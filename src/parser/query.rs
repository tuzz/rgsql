use crate::*;

#[derive(Debug)]
pub struct Query {
    select: Select,
}

impl Query {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Query> {
        Select::parser()
            .map(|select| Query { select })
            .then_ignore(just(';'))
    }
}
