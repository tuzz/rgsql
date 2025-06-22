use crate::*;

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
}

impl Expression {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        Literal::parser().map(Expression::Literal)
    }
}
