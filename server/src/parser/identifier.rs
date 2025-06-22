use crate::*;

#[derive(Debug, Default)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        let unquoted = ident().map(|s: &str| s.to_string());

        let non_quotes = any().filter(|c: &char| *c != '"');
        let escaped_quote = just("\"\"").to('"');
        let quoted = choice((non_quotes, escaped_quote)).repeated().collect().padded_by(just('"'));

        choice((unquoted, quoted)).map(|name| Identifier { name })
    }
}
