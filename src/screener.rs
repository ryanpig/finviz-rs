use crate::screener_type::ScreenerType;
use crate::signal_type::SignalType;
use crate::order_type::OrderType;
use crate::order_type::Ordering;
use crate::web_scraper::scrape_common;
use crate::common::{TableData, Scrape};
use async_trait::async_trait;

const BASE_URL: &str = "https://finviz.com/screener.ashx?";

/// This struct represents a screener configuration for data retrieval.
///
/// It has fields as following: 
/// - base_type is of type ScreenerType and represents the base type of the screener.
/// - signal_type is an optional field of type SignalType that represents the signal type for filtering.
/// - order_type is an optional field of type OrderType that represents the order type for sorting.
///
/// # Example
///
/// ```
/// use crate::finviz_rs::{
///     screener::Screener,
///     screener_type::ScreenerType,
///     signal_type::SignalType,
///     order_type::OrderType,
///     output::ToTable,
///     common::Scrape,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(),Box<dyn std::error::Error>> {
/// let table_str = Screener::new(ScreenerType::Overview)
///     .set_signal(SignalType::DoubleBottom)
///     .set_order(OrderType::Ticker)
///     .scrape().await?
///     .to_table(None, Some(3));
/// println!("{}", table_str);
/// Ok(())
/// }
/// ```
///
/// The above example demonstrates how to retrive the Overview of screener page into a table and print it.
pub struct Screener {
    base_type: ScreenerType,
    signal_type: Option<SignalType>,
    order_type: Option<OrderType>,
    ordering: Ordering
}

impl Default for Screener {
    fn default() -> Self {
        Self::new(ScreenerType::Overview)
    }
}

impl Screener {

    /// Creates a new `Screener` instance with the specified `ScreenerType`.
    pub fn new(base_type: ScreenerType) ->  Self {
        Self {base_type, signal_type: None, order_type: None, ordering: Ordering::Ascending }
    }

    /// Sets the signal type for the screener.
    pub fn set_signal(&mut self, signal_type: SignalType) -> &mut Self {
        self.signal_type = Some(signal_type);
        self
    }

    /// Sets the order type for the screener with ascending ordering.
    pub fn set_order(&mut self, order_type: OrderType) -> &mut Self {
        self.order_type = Some(order_type);
        self
    }

    /// Sets the order type for the screener with descending ordering.
    pub fn set_descending_order(&mut self, order_type: OrderType) -> &mut Self {
        self.order_type = Some(order_type);
        self.ordering = Ordering::Descending;
        self
    }


    /// Generates the URL based on the current screener configuration.
    pub fn to_url(&self) ->  String {
        format!("{}v={}{}{}", BASE_URL,
                            self.base_type, 
                            self.signal_type.as_ref().map_or(String::new(), |s| format!("&s={}", s)),
                            self.order_type.as_ref().map_or(String::new(), |s| format!("&o={}{}", self.ordering, s))
               )

    }


}

#[async_trait]
impl Scrape<TableData> for Screener {

    /// The scrape function scrapes the data from the generated URL using the scrape_common function and returns a TableData result.
    async fn scrape(&self) -> Result<TableData, Box<dyn std::error::Error>> {
        scrape_common(&self.to_url(), false).await
    }
}

    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_url_with_screenr_type() {
        let screener = Screener::new(ScreenerType::Performance);
        assert_eq!(screener.to_url(), "https://finviz.com/screener.ashx?v=141");
    }

    #[test]
    fn test_base_url_with_signal_type() {
        let mut screener = Screener::new(ScreenerType::Performance);
        screener.set_signal(SignalType::TopGainers);
        assert_eq!(screener.to_url(), "https://finviz.com/screener.ashx?v=141&s=ta_topgainers");
    }

    #[test]
    fn test_base_url_with_order_type() {
        let mut screener = Screener::new(ScreenerType::Performance);
        screener.set_signal(SignalType::TopLosers);
        screener.set_order(OrderType::Company);
        assert_eq!(screener.to_url(), "https://finviz.com/screener.ashx?v=141&s=ta_toplosers&o=company");
    }

    #[test]
    fn test_base_url_with_descending_order_type() {
        let mut screener = Screener::new(ScreenerType::Performance);
        screener.set_signal(SignalType::TopLosers);
        screener.set_descending_order(OrderType::Ticker);
        assert_eq!(screener.to_url(), "https://finviz.com/screener.ashx?v=141&s=ta_toplosers&o=-ticker");
    }

}
