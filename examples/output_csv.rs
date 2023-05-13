use finviz_rs::output::ToCsvFile;
use finviz_rs::screener::Screener;
use finviz_rs::screener_type::ScreenerType;

fn main() -> Result<(),Box<dyn std::error::Error>> {
    Screener::new(ScreenerType::Performance)
        .scrape_screener()?
        .to_csv_file("output.csv")?;
    Ok(())
}
