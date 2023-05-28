extern crate finviz_rs;

use finviz_rs::{
    forex::Forex,
    output::ToTable,
    common::Scrape
};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let table_str = Forex::default()
        .scrape().await?
        .to_table(Some(Forex::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
