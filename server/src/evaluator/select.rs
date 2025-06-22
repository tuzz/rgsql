use crate::*;

impl Evaluate for Select {
    type Output = QueryResult;

    fn evaluate(self, database: &mut Database) -> Self::Output {
        let column_names = column_names(&self);
        let mut columns_values = vec![];

        for expression in self.select_list.expressions {
            let column_values = expression.evaluate(database).unwrap();
            columns_values.push(column_values);
        }

        let num_rows = columns_values.iter()
            .map(|column_values| column_values.len())
            .max().unwrap_or(0);

        // Iterate in reverse order so we can take column_values rather than cloning.
        let mut rows = (0..num_rows).rev().map(|_| {
            columns_values.iter_mut().map(move |column_values| {
                let repeat_value = column_values.len() == 1;
                if repeat_value { column_values[0].clone() } else { column_values.pop().unwrap() }
            }).collect()
        }).collect::<Vec<_>>();
        rows.reverse();

        QueryResult::Ok(SuccessResult { rows, column_names })
    }

}

fn column_names(select: &Select) -> Vec<String> {
    select.select_list.aliases.iter().zip(&select.select_list.expressions).map(|(alias, expression)| {
        if let Some(alias) = alias {
            alias.name.clone()
        } else if let Expression::Projection(p) = expression {
            p.column_identifier.name.clone()
        } else {
            "".to_string()
        }
    }).collect()
}
