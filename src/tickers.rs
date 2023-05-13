use std::fmt;
use std::collections::BTreeMap;
use crate::web_scraper::{scrape_chart_image, get_html_body};
use scraper::{Html, Selector};
use crate::common::DictData;
use std::fs;


/// Represents a Tickers struct.
///
/// The Tickers struct provides methods for interacting with stock data and stock chart
///
/// # Examples
///
/// ```
/// use crate::finviz_rs::tickers::{Tickers, TimeFrameType, ChartType};
/// use crate::finviz_rs::output::ToTable;
/// use crate::finviz_rs::output::from_dict_to_table;
///
/// fn main() -> Result<(),Box<dyn std::error::Error>> {
/// // save a ticker's chart image to a file
/// let tickers = Tickers::new("AAPL");
/// tickers.ticker_charts(TimeFrameType::Daily, ChartType::ADVANCED, ".")?;
///
/// // output json to table
/// let fundament_info = Tickers::new("AAPL").ticker_fundament()?;
/// println!("{}", from_dict_to_table(&fundament_info, 4).to_table(None, None));
/// Ok(())
/// }
/// 
///
/// ```
pub struct Tickers {
    ticker: String,
}

#[doc(hidden)]
pub enum TimeFrameType {
    Daily, Weekly, Monthly
}


impl fmt::Display for TimeFrameType {

    /// Formats the `TimeFrameType` that can be used as URL parameter in `Tickers` 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeFrameType::Daily => write!(f, "daily"),
            TimeFrameType::Weekly => write!(f, "weekly"),
            TimeFrameType::Monthly => write!(f, "monthly"),
        }
    }
}

#[doc(hidden)]
pub enum ChartType {
    CANDLE, LINE, ADVANCED
}

impl fmt::Display for ChartType {

    /// Formats the `ChartType` that can be used as URL parameter in `Tickers` 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChartType::CANDLE => write!(f, "candle"),
            ChartType::LINE => write!(f, "line"),
            ChartType::ADVANCED => write!(f, "advanced"),
        }
    }
}


impl Tickers {

    /// Creates a new `Tickers` instance with the specified ticker.
    pub fn new(ticker: &str) -> Self {
        Self {
            ticker: ticker.to_owned(),
        }
    }

    /// Scrapes chart image by the given timeframe and chart type, return the String of the saved image path in
    /// the specified output directory on success, or error string on failure
    pub fn ticker_charts(
        &self,
        timeframe: TimeFrameType,
        charttype: ChartType,
        out_dir: &str,
    ) -> Result<String, String> {

        let (url_type , url_ta) = match (&charttype, &timeframe) {
            (ChartType::LINE, _) =>  ("l", "0"),
            (ChartType::CANDLE, _) =>  ("c", "0"),
            (ChartType::ADVANCED, TimeFrameType::Daily) =>  ("c", "1"),
            (ChartType::ADVANCED, _) =>  ("c", "0"),
        };


        let url_timeframe = match &timeframe {
            TimeFrameType::Weekly => "w",
            TimeFrameType::Monthly => "m",
            TimeFrameType::Daily => "d",
        };

        let chart_url = format!(
            "https://finviz.com/chart.ashx?t={}&ty={}&ta={}&p={}",
            self.ticker, url_type, url_ta, url_timeframe
        );

        let out_dir = if out_dir.is_empty() { "." } else { out_dir } ;
        fs::create_dir_all(out_dir).map_err(|err| err.to_string())?;

        scrape_chart_image(&chart_url, &self.ticker, out_dir)
    }

    /// Retrieves fundamental information for a specific ticker.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the current instance of the struct.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `DictData`, which is a type alias for `BTreeMap<String, String>`.
    /// The `DictData` map contains fundamental information for the ticker, where each key-value pair represents
    /// a fundamental attribute and its corresponding value.
    ///
    /// # Errors
    ///
    /// This function can return an error of type `Box<dyn std::error::Error>` if there is an issue retrieving or parsing the HTML data.
    ///
    pub fn ticker_fundament(
        &mut self,
    ) -> Result<DictData, Box<dyn std::error::Error>> {

        let body = get_html_body(&format!("https://finviz.com/quote.ashx?t={}", self.ticker))?;
        let document = Html::parse_document(&body);

        let mut fundament_info: DictData = BTreeMap::new();
        let fundament_selector = Selector::parse("table.snapshot-table2 tr").unwrap();
        let fundament_rows = document.select(&fundament_selector);

        let row_selector = &Selector::parse("td").unwrap();
        for row in fundament_rows {
            let cols = row.select(row_selector);
            let cols: Vec<String> = cols.map(|col| col.text().collect()).collect();
            cols.chunks(2) 
            .for_each(|pair| {
                if let [key, value] = pair {
                    fundament_info.insert(key.to_owned(), value.to_owned());
                }
            });
        }

        //println!("{:?}", fundament_info);

        Ok(fundament_info)

    }


}

