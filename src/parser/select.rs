use crate::*;

#[derive(Debug)]
pub struct Select {
    select_list: SelectList,
}

impl Select {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Select> {
        just("SELECT")
            .ignore_then(whitespace().at_least(1))
            .ignore_then(SelectList::parser().padded())
            .map(|select_list| Select { select_list })
    }
}
