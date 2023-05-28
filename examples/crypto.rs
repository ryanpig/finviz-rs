extern crate finviz_rs;

use finviz_rs::{
    crypto::Crypto,
    output::ToTable,
    common::Scrape
};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let table_str = Crypto::default()
        .scrape().await?
        .to_table(Some(Crypto::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
