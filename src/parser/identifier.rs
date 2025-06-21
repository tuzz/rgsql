use crate::*;

#[derive(Debug, Default)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Identifier, RichError<'a>> {
        ident().map(|s: &str| Identifier { name: s.to_string() })
    }
}
