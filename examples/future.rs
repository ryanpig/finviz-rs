extern crate finviz_rs;

use finviz_rs::{
    future::Future,
    output::ToTable,
    common::Scrape
};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let table_str = Future::default()
        .scrape().await?
        .to_table(Some(Future::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
