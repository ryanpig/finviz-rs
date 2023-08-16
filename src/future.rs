use crate::web_scraper::get_html_body;
use crate::common::{TableData, Scrape};
use serde_json::Value;
use std::fmt;
use strum::EnumIter;
use async_trait::async_trait;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Represents the possible errors that can occur when working with futures.
#[derive(Debug)]
enum FutureError {
    /// Indicates that the expected JSON value is not an array.
    NoJsonArray,
}

impl fmt::Display for FutureError {
    /// Formats the error message associated with the FutureError.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match *self {
            FutureError::NoJsonArray =>  write!(f, "expect json array, but failed to convert it via as_array")
         }
    }
}

impl std::error::Error for FutureError {
    /// Retrieves the underlying source of the error, if any.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FutureError::NoJsonArray => None,
        }
    }
}

/// Represents the time frame for futures data.
#[doc(hidden)]
#[derive(Clone, Copy, EnumIter)]
pub enum TimeFrame {
    Daily, Weekly, Monthly, Quarter, HalfYear, Year
}

/// Represents futures data and provides methods for interacting with it.
///
/// # Example
///
/// ```
/// use finviz_rs::{
///     future::Future,
///     output::ToTable,
///     common::Scrape,
/// };
/// 
/// #[tokio::main]
/// async fn main() -> Result<(),Box<dyn std::error::Error>>{
///     let table_str = Future::default()
///         .scrape().await?
///         .to_table(Some(Future::default_header()), Some(3));
///     println!("{}", table_str);
///     Ok(())
/// }
/// ```
///
///
/// The above example demonstrates how to retrive futures data into a table and print it.
pub struct Future {
    timeframe: TimeFrame,
}

impl Default for Future {

    /// Creates a new instance of Future using the default constructor.
    fn default() -> Self {
        Self::new(TimeFrame::Daily)
    }
}

impl Future {

    /// The base URL for futures performance data.
    const BASE_URL: &'static str = "https://finviz.com/futures_performance.ashx";

    /// Creates a new instance of Future with the specified TimeFrame.
    ///
    /// # Arguments
    ///
    /// * `timeframe` - The time frame for the futures data.
    pub fn new(timeframe: TimeFrame) -> Self {
        Self{timeframe}
    }

    /// Returns the URL for retrieving futures performance data based on the TimeFrame.
    fn get_url(&self) -> String {
        match &self.timeframe {
            TimeFrame::Daily => Future::BASE_URL.to_string(),
            TimeFrame::Weekly => format!("{}?v=12", Future::BASE_URL),
            TimeFrame::Monthly => format!("{}?v=13", Future::BASE_URL),
            TimeFrame::Quarter => format!("{}?v=14", Future::BASE_URL),
            TimeFrame::HalfYear => format!("{}?v=15", Future::BASE_URL),
            TimeFrame::Year => format!("{}?v=16", Future::BASE_URL),
        }
    }


    /// Returns the default header for the futures performance table.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the default header.
    pub fn default_header() -> Vec<String>  {
        ["ticker", "label", "group", "perf"]
        .map(String::from).to_vec()
    }
}

#[async_trait]
impl Scrape<TableData> for Future {

    /// Scrapes futures performance data from the specified URL.
    ///
    /// # Returns
    ///
    /// A Result containing the scraped data as TableData on success, or a FutureError on failure.
    async fn scrape(&self) -> Result<TableData> {
        let url = self.get_url();
        let body = get_html_body(&url).await?;
        let start_index = body.find("var rows = ").unwrap() + 11;
        let end_index = body.find("FinvizInitFuturesPerformance(rows);").unwrap();
        let data_str = body[start_index..end_index].trim().trim_end_matches(';');
        let data: Value = serde_json::from_str(data_str)?;

        let mut result = Vec::new();
        let expected_keys = Future::default_header();

        if let Some(rows) = data.as_array() {
            for row in rows {
              let row_values: Vec<String> = expected_keys
                    .iter()
                    .map(|key| {
                        if let Some(value) = row.get(key) {
                            if value.is_string() {
                                // remove double quotes if Value is String type
                                value.as_str().unwrap().to_string()
                            } else {
                                value.to_string()
                            }
                        } else {
                            String::new()
                        }
                    })
                    .collect();
                result.push(row_values);
            }
            return Ok(result);
        } 
        Err(Box::new(FutureError::NoJsonArray))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let future = Future::new(TimeFrame::Quarter);
        assert_eq!(future.get_url(), "https://finviz.com/futures_performance.ashx?v=14".to_string())
    }
}


