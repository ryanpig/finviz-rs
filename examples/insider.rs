extern crate finviz_rs;

use finviz_rs::{
    insider::Insider,
    output::ToTable
};

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let table_str = Insider::default()
        .scrape_insider()?
        .to_table(Some(Insider::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
