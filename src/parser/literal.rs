use crate::*;

#[derive(Clone, Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
}

impl Literal {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Literal> {
        choice((
            just("TRUE").to(Literal::Boolean(true)),
            just("FALSE").to(Literal::Boolean(false)),
            int(10).map(|s: &str| Literal::Integer(s.parse::<i64>().unwrap())),
        ))
    }
}
