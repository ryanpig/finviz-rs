use crate::web_scraper::scrape_common;
use crate::common::{TableData, Scrape};
use strum::EnumIter;
use async_trait::async_trait;

/// Represents the type of Forex data.
#[derive(Clone, Copy, EnumIter)]
pub enum ForexType {
    /// Forex performance represented as a percentage.
    Percent,
    /// Forex performance represented in PIPS.
    PIPS,
}

/// Represents Forex data and provides methods for interacting with it.
///
/// # Example
///
/// ```
/// use finviz_rs::{
///     forex::Forex,
///     output::ToTable,
///     common::Scrape,
/// };
/// 
/// #[tokio::main]
/// async fn main() -> Result<(),Box<dyn std::error::Error>> {
///     let table_str = Forex::default()
///         .scrape().await?
///         .to_table(Some(Forex::default_header()), Some(3));
///     println!("{}", table_str);
///     Ok(())
/// }
/// ```
///
/// The above example demonstrates how to retrive Forex performance data into a table and print it.
pub struct Forex {
    forext_type: ForexType,
}

impl Default for Forex {
    /// Creates a new instance of Forex using the default constructor.
    fn default() -> Self {
        Self::new(ForexType::Percent)
    }
}

impl Forex {
    /// The base URL for Forex performance data.
    const BASE_URL: &'static str = "https://finviz.com/forex_performance.ashx";

    /// Creates a new instance of Forex with the specified ForexType.
    ///
    /// # Arguments
    ///
    /// * `forext_type` - The type of Forex data to retrieve.
    pub fn new(forext_type: ForexType) -> Self {
        Self { forext_type }
    }

    /// Returns the URL for retrieving Forex performance data based on the ForexType.
    fn get_url(&self) -> String {
        match self.forext_type {
            ForexType::Percent => Forex::BASE_URL.to_string(),
            ForexType::PIPS => format!("{}?v=1&tv=2&o=-perfdaypct", Forex::BASE_URL),
        }
    }

    /// Returns the default header for Forex performance table.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the default header.
    pub fn default_header() -> Vec<String> {
        [
            "Ticker", "Price", "Perf 5Min", "Perf Hour", "Perf Day", "Perf Week", "Perf Month",
            "Perf Quart", "Perf Half", "Perf Year", "Perf YTD",
        ].map(String::from).to_vec()
    }

}

#[async_trait]
impl Scrape<TableData> for Forex {

    /// Scrapes Forex performance data from the specified URL.
    ///
    /// # Returns
    ///
    /// A Result containing the scraped data as TableData on success, or a `Box<dyn std::error::Error>`
    /// on failure.
    async fn scrape(&self,) -> Result<TableData, Box<dyn std::error::Error>> {
        scrape_common(&self.get_url(), true).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let forex = Forex::new(ForexType::PIPS);
        assert_eq!(forex.get_url(), "https://finviz.com/forex_performance.ashx?v=1&tv=2&o=-perfdaypct".to_string())
    }
}



