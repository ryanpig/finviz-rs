
extern crate finviz_rs;

use finviz_rs::{
    tickers::*,
    output::{ToTable, from_dict_to_table},
    common::Scrape
};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    // save a ticker's chart image to a file
    let tickers = Tickers::new("AAPL");
    tickers.ticker_charts(TimeFrameType::Daily, ChartType::ADVANCED, ".").await?;

    // output json to table
    let fundament_info = Tickers::new("AAPL").scrape().await?;
    println!("{}", from_dict_to_table(&fundament_info, 4).to_table(None, None));
    Ok(())
}
