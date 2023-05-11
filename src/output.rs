
use tabled::settings::Style;

// show a whole table or max number of rows
pub fn to_table(data: &Vec<Vec<String>>, max_rows: Option<usize>) -> String 
{
    let mut builder = tabled::builder::Builder::default();
    builder.set_header(data.first().unwrap().to_owned());

    data.into_iter()
        .skip(1)
        .take(max_rows.unwrap_or(data.len() - 1))
        .for_each(
        |row|{ builder.push_record(row.to_owned());});

    let table = builder.build()
        .with(Style::modern())
        .to_string();
    return table;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_table() {
        let headers = ["ETF Name", "Price", "Chg", "Chg(%)", "Vol"].map(String::from).to_vec();
        let row1 = ["VOO", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let row2 = ["VOO2", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let row3 = ["VOO3", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let data = Vec::from([headers, row1, row2, row3]);
        assert_eq!(to_table(&data, Some(1)), 
"┌──────────┬───────┬─────┬────────┬──────┐
│ ETF Name │ Price │ Chg │ Chg(%) │ Vol  │
├──────────┼───────┼─────┼────────┼──────┤
│ VOO      │ 10.0  │ 3.3 │ 5.5    │ 3000 │
└──────────┴───────┴─────┴────────┴──────┘");

        assert_eq!(to_table(&data, None), 
"┌──────────┬───────┬─────┬────────┬──────┐
│ ETF Name │ Price │ Chg │ Chg(%) │ Vol  │
├──────────┼───────┼─────┼────────┼──────┤
│ VOO      │ 10.0  │ 3.3 │ 5.5    │ 3000 │
├──────────┼───────┼─────┼────────┼──────┤
│ VOO2     │ 10.0  │ 3.3 │ 5.5    │ 3000 │
├──────────┼───────┼─────┼────────┼──────┤
│ VOO3     │ 10.0  │ 3.3 │ 5.5    │ 3000 │
└──────────┴───────┴─────┴────────┴──────┘");

    }
}
