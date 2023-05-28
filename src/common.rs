use std::collections::BTreeMap;
use async_trait::async_trait;

/// Convert enum types to more readable string 
pub trait DisplayString {

    /// Implemnet this function to output readable string or description of a variant of enum type
    fn to_display_string(&self) -> &str;
}

/// Alias to represent two-dimension String data
pub type TableData = Vec<Vec<String>>;

/// Alias to represent key-value String data
pub type DictData = BTreeMap<String, String>;

/// Convert the scraping data into result type T 
#[async_trait]
pub trait Scrape<T> {

    /// Scrape html content and store the result in type T on success, or `std::error::Error` on failure
    async fn scrape(&self,) -> Result<T, Box<dyn std::error::Error>>;
}


