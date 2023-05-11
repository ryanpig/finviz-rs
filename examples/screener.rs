
extern crate finviz_rs;
use finviz_rs::{
    screener::Screener,
    screener_type::ScreenerType,
    signal_type::SignalType,
    web_scraper::text_scrape,
    order_type::OrderType,
    output::to_table,
};

fn main() {
    println!("--- Overview ---");
    let mut screener = Screener::new(ScreenerType::Overview);
    screener.set_signal(SignalType::DoubleBottom);
    screener.set_order(OrderType::Ticker);
	let content = text_scrape(&screener.to_url());
    println!("{}", to_table(&content, Some(3)));

    println!("--- Performance ---");
    let mut screener = Screener::new(ScreenerType::Performance);
    screener.set_signal(SignalType::TopLosers);
    screener.set_order(OrderType::EPS);
	let content = text_scrape(&screener.to_url());
    println!("{}", to_table(&content, Some(2)));

    println!("--- Financial ---");
    let mut screener = Screener::new(ScreenerType::Financial);
    screener.set_signal(SignalType::NewHigh);
    screener.set_order(OrderType::MarketCap);
	let content = text_scrape(&screener.to_url());
    println!("{}", to_table(&content, Some(3)));

    println!("--- Technical ---");
    let mut screener = Screener::new(ScreenerType::Technical);
    screener.set_signal(SignalType::NewHigh);
    screener.set_order(OrderType::MarketCap);
	let content = text_scrape(&screener.to_url());
    println!("{}", to_table(&content, Some(2)));
}

