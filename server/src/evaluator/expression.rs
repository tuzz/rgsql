use crate::*;

impl Evaluate for Expression {
    type Output = Result<Vec<Literal>, String>;

    fn evaluate(self, database: &mut Database) -> Self::Output {
        match self {
            Self::Literal(literal) => Ok(vec![literal]),
        }
    }
}
