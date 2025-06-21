use crate::*;

#[derive(Debug)]
pub struct SelectList {
    literals: Vec<Literal>,
}

impl SelectList {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, SelectList> {
        Literal::parser()
            .separated_by(just(',').padded())
            .at_least(1)
            .collect()
            .map(|literals| SelectList { literals })
    }
}
