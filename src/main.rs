
mod screener_type;
mod common;
mod signal_type;
mod screener;
mod order_type;
mod web_scraper;

use screener_type::ScreenerType;
use screener::Screener;
use signal_type::SignalType;
use web_scraper::text_scrape;

#[cfg(feature = "output_table")]
mod output;


fn main() {
    println!("--- Finviz-rs ---");
    let mut screener = Screener::new(ScreenerType::Performance);
    screener.set_signal(SignalType::TopLosers);
    screener.set_order(order_type::OrderType::EPS);
	let r = text_scrape(&screener.to_url());
    println!("{}", output::to_table(&r, None));
}
