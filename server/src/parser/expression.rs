use crate::*;

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    Projection(Projection),
}

impl Expression {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        choice((
            Literal::parser().map(Self::Literal),
            Projection::parser().map(Self::Projection),
        ))
    }
}
