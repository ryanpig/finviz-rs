extern crate finviz_rs;

use finviz_rs::{
    crypto::Crypto,
    output::ToTable
};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let table_str = Crypto::default()
        .scrape_crypto_performance()?
        .to_table(Some(Crypto::default_header()), Some(3));
    println!("{}", table_str);
    Ok(())
}
