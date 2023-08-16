use scraper::{Html, Selector};
use crate::web_scraper::get_html_body;
use crate::common::{TableData, Scrape};
use strum::EnumIter;
use async_trait::async_trait;

#[doc(hidden)]
#[derive(Clone, EnumIter)]
pub enum InsiderType {
    Latest,
    LatestBuys,
    LatestSales,
    TopWeek,
    TopWeekBuys,
    TopWeekSales,
    TopOwnerTrade,
    TopOwnerBuys,
    TopOwnerSales,
    Numeric(String),
}

/// This struct represents the insider trading configuration.
///
/// # Example
///
/// ```
/// use finviz_rs::{
///     insider::Insider,
///     output::ToTable,
///     common::Scrape,
/// };
/// 
/// #[tokio::main]
/// async fn main() -> Result<(),Box<dyn std::error::Error>>{
///     let table_str = Insider::default()
///         .scrape().await?
///         .to_table(Some(Insider::default_header()), Some(3));
///     println!("{}", table_str);
///     Ok(())
/// }
///
/// ```
///
/// The above example demonstrates how to retrive the insider trading data into a table and print it.
pub struct Insider {
    insider_type: InsiderType,
}

impl Default for Insider {
    fn default() -> Self {
        Self::new(InsiderType::Latest)
    }
}

impl Insider {
    const BASE_URL: &'static str = "https://finviz.com/insidertrading.ashx";


    /// Create a new instance of `Insider` 
    pub fn new(insider_type: InsiderType) -> Self {
        Self{insider_type}
    }

    /// Constructs the URL for the insider trading with the specified parameters.
    fn get_url(&self) -> String {
        match &self.insider_type {
            InsiderType::Latest => Insider::BASE_URL.to_string(),
            InsiderType::LatestBuys => format!("{}?tc=1", Insider::BASE_URL),
            InsiderType::LatestSales => format!("{}?tc=2", Insider::BASE_URL),
            InsiderType::TopWeek => format!("{}?or=-10&tv=100000&tc=7&o=-transactionValue", Insider::BASE_URL),
            InsiderType::TopWeekBuys => format!("{}?or=-10&tv=100000&tc=1&o=-transactionValue", Insider::BASE_URL),
            InsiderType::TopWeekSales => format!("{}?or=-10&tv=100000&tc=2&o=-transactionValue", Insider::BASE_URL),
            InsiderType::TopOwnerTrade => format!("{}?or=10&tv=1000000&tc=7&o=-transactionValue", Insider::BASE_URL),
            InsiderType::TopOwnerBuys => format!("{}?or=10&tv=1000000&tc=1&o=-transactionValue", Insider::BASE_URL),
            InsiderType::TopOwnerSales => format!("{}?or=10&tv=1000000&tc=2&o=-transactionValue", Insider::BASE_URL),
            InsiderType::Numeric(num) => format!("{}?oc={}&tc=7", Insider::BASE_URL, num),
        }
    }

    /// Returns the default header for insider trading data.
    pub fn default_header() -> Vec<String>  {
        ["Ticker", "Owner", "Relationship", "Date", "Transaction", "Cost ", "#Shares", "Value ($)", "#Shares Total", "SEC Form 4", "SEC Form 4 Link"]
            .map(String::from).to_vec()
    }


}

#[async_trait]
impl Scrape<TableData> for Insider {

    /// Scrapes the insider trading data from the specified URL and return `TableData` on success
    async fn scrape(&self) -> Result<TableData, Box<dyn std::error::Error>> {
        let url = self.get_url();
        let body = get_html_body(&url).await?;
        let document = Html::parse_document(&body);

        let table_selector = Selector::parse("table.body-table")?;
        let row_selector = Selector::parse("tr")?;
        let header_selector = Selector::parse("td")?;
        let link_selector = Selector::parse("a")?;

        let insider_trader = document.select(&table_selector).next().ok_or("Table not found")?;
        let rows = insider_trader.select(&row_selector).collect::<Vec<_>>();

        let mut frame = Vec::new();

        // Skip the first row since the default header is used
        for row in rows[1..].iter() {
            let cols = row.select(&header_selector).collect::<Vec<_>>();
            if cols.len() < 5 {
                continue;
            }

            let mut info_dict = Vec::new();
            for (i, col) in cols.iter().enumerate() {
                //match i 
                match i {
                    n if i == cols.len() - 1 => {
                        let link = cols[n].select(&link_selector).next();
                        info_dict.push(col.text().collect::<String>());
                        if let Some(a) = link {
                            info_dict.push(a.value().attr("href").unwrap_or("").to_string());
                        } else {
                            info_dict.push(String::new());
                        }
                    },
                    _ => info_dict.push(col.text().collect::<String>())
                };
            }
            frame.push(info_dict);
        }

        Ok(frame)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insider() {
        let insider = Insider::default();
        let result = insider.scrape().await;
        assert!(result.is_ok(), "failed to get insider info");
    }

    #[test]
    fn test_url() {
        let insider = Insider::new(InsiderType::TopWeek);
        assert_eq!(insider.get_url(), "https://finviz.com/insidertrading.ashx?or=-10&tv=100000&tc=7&o=-transactionValue".to_string())
    }
}

