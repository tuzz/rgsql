use crate::*;

#[derive(Clone, Copy, Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
}

impl Literal {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> + Clone {
        choice((
            just("TRUE").to(Literal::Boolean(true)),
            just("FALSE").to(Literal::Boolean(false)),
            Self::integer_parser().map(Literal::Integer),
        ))
    }

    fn integer_parser<'a>() -> impl Parser<'a, &'a str, i64, RichError<'a>> + Clone {
        let sign = just("-").to(-1).or_not().map(|sign| sign.unwrap_or(1));
        let number = text::int(10).map(|s: &str| s.parse::<i64>().unwrap());

        sign.then(number).map(|(sign, number)| sign * number)
    }
}
