use crate::web_scraper::scrape_common;
use crate::common::{TableData, Scrape};

/// Represents a Crypto struct.
///
/// The Crypto struct provides methods for interacting with cryptocurrency data.
///
/// # Examples
///
/// ```
/// use crate::finviz_rs::crypto::Crypto;
/// use crate::finviz_rs::output::ToTable;
/// use crate::finviz_rs::common::Scrape;
///
/// let crypto = Crypto::new();
/// let table = crypto.scrape();
///
/// if let Ok(table_data) = table {
///     println!("{}", table_data.to_table(None, None));
/// }
/// ````
pub struct Crypto {}

impl Default for Crypto {

    /// Creates a new instance of Crypto using the default constructor.
    fn default() -> Self {
        Self::new()
    }
}

impl Crypto {

    /// The base URL for crypto performance data.
    const BASE_URL: &'static str = "https://finviz.com/crypto_performance.ashx";

    /// Creates a new instance of Crypto.
    pub fn new() -> Self {
        Self{}
    }

    /// Returns the default header for crypto performance table.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the default header.
    pub fn default_header() -> Vec<String>  {
        ["Ticker", "Price", "Perf 5Min", "Perf Hour", "Perf Day", "Perf Week", "Perf Month", "Perf Quart", "Perf Half", "Perf Year", "Perf YTD"]
        .map(String::from).to_vec()
    }
}

impl Scrape<TableData> for Crypto {

    /// Scrapes crypto performance data from the specified URL.
    ///
    /// # Returns
    ///
    /// A Result containing the scraped data as TableData on success, or a `Box<dyn std::error::Error>`
    /// on failure.
    fn scrape(&self,) -> Result<TableData, Box<dyn std::error::Error>> {
        scrape_common(Crypto::BASE_URL, true)
    }
}
