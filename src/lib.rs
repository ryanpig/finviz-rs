
/*!
finviz-rs is a crate gathering financial data from [finviz.com](https://finviz.com/).
Check `READMD.md` to get started with examples.
                                                                          
*/

#![deny(missing_docs)]

/// a module includes the enum of screener type taht is used in `Screener`
pub mod screener_type;
/// a module includes the enum of signal type taht is used in `Screener`
pub mod signal_type;
/// a module includes the enum of order type taht is used in `Screener`
pub mod order_type;
/// a module includes `Screener` struct and related function for data scraping
pub mod screener;


/// a module includes `News` struct for news and blogs scraping
pub mod news;
/// a module includes `Insider` struct for insider data scraping
pub mod insider;
/// a module includes `Forex` struct for forex data scraping
pub mod forex;
/// a module includes `Crypto` struct for cryptocurrency data scraping
pub mod crypto;
/// a module includes `Future` struct for futures data scraping
pub mod future;
/// a module includes `Group` struct for group data scraping 
pub mod group;
/// a module includes `Tickers` struct for stock data scraping and chart downloading
pub mod tickers;

/// a module includes helper function to convert scraped data to different types of output 
pub mod output;

/// common types and traits
pub mod common;

/// utility functions for scraping web content
pub mod web_scraper;
