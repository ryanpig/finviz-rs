extern crate finviz_rs;

use finviz_rs::{
    insider::Insider,
    output::ToTable,
    common::Scrape
};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let table_str = Insider::default()
        .scrape().await?
        .to_table(Some(Insider::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
