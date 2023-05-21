extern crate finviz_rs;

use finviz_rs::{
    insider::Insider,
    output::ToTable,
    common::Scrape
};

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let table_str = Insider::default()
        .scrape()?
        .to_table(Some(Insider::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
