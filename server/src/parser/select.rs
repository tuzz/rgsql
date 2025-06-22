use crate::*;

#[derive(Debug)]
pub struct Select {
    pub select_list: SelectList,
}

impl Select {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        just("SELECT").ignore_then(choice((
            whitespace().at_least(1).ignore_then(SelectList::parser()),
            SelectList::parser().and_is(empty()),
        )).padded()).map(|select_list| Select { select_list })
    }
}
