use finviz_rs::{
    output::ToCsvFile,
    screener::Screener,
    screener_type::ScreenerType,
    common::Scrape
};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    Screener::new(ScreenerType::Performance)
        .scrape()?
        .to_csv_file("output.csv")?;
    Ok(())
}
