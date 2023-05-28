use finviz_rs::{
    output::ToCsvFile,
    screener::Screener,
    screener_type::ScreenerType,
    common::Scrape
};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    Screener::new(ScreenerType::Performance)
        .scrape().await?
        .to_csv_file("output.csv")?;
    Ok(())
}
