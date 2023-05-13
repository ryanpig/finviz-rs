use tabled::settings::{Style, Width, Modify, object::Rows};
use crate::common::{DictData, TableData};
use csv::Writer;
use std::error::Error;
use std::fs::File;

/// A trait for converting data into a formatted table.
pub trait ToTable {

    /// Converts the data into a formatted table.
    ///
    /// # Arguments
    ///
    /// * `headers` - Optional headers for the table columns.
    /// * `max_rows` - Optional maximum number of rows to include in the table.
    ///
    /// # Returns
    ///
    /// A string representation of the formatted table.
    fn to_table(&self, headers: Option<Vec<String>>, max_rows: Option<usize>) -> String;
}

/// Implements the `ToTable` trait for `TableData`.
impl ToTable for TableData {

    fn to_table(&self, headers: Option<Vec<String>>, max_rows: Option<usize>) -> String {
        let mut builder = tabled::builder::Builder::default();
        if let Some(headers) = headers {
            builder.set_header(headers);
        }

        self.iter()
            .take(max_rows.unwrap_or(self.len()))
            .for_each(
            |row|{ builder.push_record(row.to_owned());});

        let table = builder.build()
            .with(Style::modern())
            .with(Modify::new(Rows::new(..)).with(Width::truncate(50).suffix("...")))
            .to_string();
        table
    }
}

/// Converts a dictionary into a table representation.
///
/// This function takes a dictionary (`dict`) and converts it into a table representation,
/// where each key-value pair is represented as a row in the table. The maximum number of
/// key-value pairs per row can be specified using the `max_pairs_per_row` argument.
///
/// # Arguments
///
/// * `dict` - A reference to a dictionary (`DictData`) containing key-value pairs.
/// * `max_pairs_per_row` - The maximum number of key-value pairs per row in the table.
///
/// # Returns
///
/// A table representation of the dictionary as `TableData`.
///
/// # Example
///
/// ```
/// use crate::finviz_rs::output::{from_dict_to_table, ToTable};
/// use crate::finviz_rs::common::{DictData, TableData};
///
/// let dict = DictData::new(); // Assume `DictData` is properly defined and populated.
/// let max_pairs_per_row = 3;
/// let table = from_dict_to_table(&dict, max_pairs_per_row);
///
/// println!("{}", table.to_table(None, None)); // Print the formatted table.
/// ```
///
/// The above example demonstrates how to convert a dictionary into a table and print it.
pub fn from_dict_to_table(dict: &DictData, max_pairs_per_row: usize) -> TableData {
    let r =  dict
        .iter()
        .map(|(key, value)| vec![key.to_owned(), value.to_owned()])
        .collect::<Vec<Vec<String>>>()
        .chunks(max_pairs_per_row)
        .map(|chunk| {
            let mut row_vec = Vec::new();
            for v in chunk {
                row_vec.extend(v.to_owned());
            }
            row_vec
        })
        .collect();
    r
}

/// Writes data to a CSV file at the specified file path.
#[cfg(feature = "output_csv")]
pub trait ToCsvFile {

    /// This function is only available when the "output_csv" feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the table data to be written.
    /// * `file_path` - The file path where the CSV file will be created.
    ///
    /// # Returns
    ///
    /// Returns `Result<(), Box<dyn Error>>` indicating success or an error if writing to the file fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if:
    /// * The file cannot be created or opened.
    /// * Writing the CSV records to the file fails.
    /// * Flushing the writer fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::finviz_rs::common::TableData;
    /// use crate::finviz_rs::output::ToCsvFile;
    ///
    /// let data: TableData = vec![
    ///     vec!["Header 1".to_owned(), "Header 2".to_owned()],
    ///     vec!["Value 1".to_owned(), "Value 2".to_owned()],
    /// ];
    ///
    /// let file_path = "output.csv";
    ///
    /// if let Err(err) = data.to_csv_file(file_path) {
    ///     eprintln!("Failed to write CSV file: {}", err);
    /// }
    /// ```
    ///
    fn to_csv_file(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
}

impl ToCsvFile for TableData {
    fn to_csv_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut writer = Writer::from_writer(file);

        for row in self {
            writer.write_record(row)?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_to_table() {
        let headers = ["ETF Name", "Price", "Chg", "Chg(%)", "Vol"].map(String::from).to_vec();
        let row1 = ["VOO", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let row2 = ["VOO2", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let row3 = ["VOO3", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let data = Vec::from([row1, row2, row3]);
        assert_eq!(data.to_table(Some(headers.clone()), Some(1)),
"┌──────────┬───────┬─────┬────────┬──────┐
│ ETF Name │ Price │ Chg │ Chg(%) │ Vol  │
├──────────┼───────┼─────┼────────┼──────┤
│ VOO      │ 10.0  │ 3.3 │ 5.5    │ 3000 │
└──────────┴───────┴─────┴────────┴──────┘");

        assert_eq!(data.to_table(Some(headers), None),
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

    #[test]
    fn output_table_from_dict() {
        let hashmap_data: DictData = [
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
            ("key4".to_string(), "value4".to_string()),
            ("key5".to_string(), "value5".to_string()),
            ("key6".to_string(), "value6".to_string()),
            ("key7".to_string(), "value7".to_string()),
            ("key8".to_string(), "value8".to_string()),
        ]
        .iter()
        .cloned()
        .collect();


        let table_data = from_dict_to_table(&hashmap_data, 4);
        println!("{}", table_data.to_table(None, None));
        assert_eq!(table_data.to_table(None, None),
"┌──────┬────────┬──────┬────────┬──────┬────────┬──────┬────────┐
│ key1 │ value1 │ key2 │ value2 │ key3 │ value3 │ key4 │ value4 │
├──────┼────────┼──────┼────────┼──────┼────────┼──────┼────────┤
│ key5 │ value5 │ key6 │ value6 │ key7 │ value7 │ key8 │ value8 │
└──────┴────────┴──────┴────────┴──────┴────────┴──────┴────────┘");

    }

    #[cfg(feature = "output_csv")]
    #[test]
    fn test_to_csv() {
        let headers = ["ETF Name", "Price", "Chg", "Chg(%)", "Vol"].map(String::from).to_vec();
        let row1 = ["VOO", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let row2 = ["VOO2", "10.0", "3.3", "5.5", "3000"].map(String::from).to_vec();
        let data = Vec::from([headers, row1, row2]);
        assert!(data.to_csv_file("test.csv").is_ok());
        assert!(Path::new("./test.csv").is_file());
    }
}
