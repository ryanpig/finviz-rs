extern crate finviz_rs;

use finviz_rs::{
    forex::Forex,
    output::ToTable
};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let table_str = Forex::default()
        .scrape_forex_performance()?
        .to_table(Some(Forex::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
