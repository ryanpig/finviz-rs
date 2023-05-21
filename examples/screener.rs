
extern crate finviz_rs;
use finviz_rs::{
    screener::Screener,
    screener_type::ScreenerType,
    signal_type::SignalType,
    order_type::OrderType,
    output::ToTable,
    common::Scrape
};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("--- Overview ---");
    let table_str = Screener::new(ScreenerType::Overview)
        .set_signal(SignalType::DoubleBottom)
        .set_order(OrderType::Ticker)
        .scrape()?
        .to_table(None, Some(3));
    println!("{}", table_str);

    println!("--- Performance ---");
    let table_str = Screener::new(ScreenerType::Performance)
        .set_signal(SignalType::TopLosers)
        .set_order(OrderType::EPS)
        .scrape()?
        .to_table(None, Some(2));
    println!("{}", table_str);

    println!("--- Financial ---");
    let table_str= Screener::new(ScreenerType::Financial)
        .set_signal(SignalType::NewHigh)
        .set_order(OrderType::MarketCap)
        .scrape()?
        .to_table(None, Some(3));
    println!("{}", table_str);

    println!("--- Technical ---");
    let table_str = Screener::new(ScreenerType::Technical)
        .set_signal(SignalType::NewHigh)
        .set_order(OrderType::MarketCap)
        .scrape()?
        .to_table(None, Some(3));
    println!("{}", table_str);
    Ok(())
}

