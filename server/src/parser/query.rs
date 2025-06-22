use crate::*;

#[derive(Debug)]
pub enum Query {
    Select(Select),
    CreateTable(CreateTable),
    DropTable(DropTable),
    InsertInto(InsertInto),
}

pub type RichError<'a> = extra::Err<Rich<'a, char>>;

impl Query {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, RichError<'a>> {
        choice((
            Select::parser().map(Query::Select),
            CreateTable::parser().map(Query::CreateTable),
            DropTable::parser().map(Query::DropTable),
            InsertInto::parser().map(Query::InsertInto),
        )).then_ignore(just(';'))
    }
}
