extern crate finviz_rs;

use finviz_rs::{
    insider::{Insider, InsiderType},
    output::ToTable,
    common::Scrape
};

use strum::IntoEnumIterator;

fn main() -> Result<(),Box<dyn std::error::Error>>{

    // fetch all types of insider trading data by iterating the enum of InsiderType
    for insider_type in InsiderType::iter() {
        let table_str = Insider::new(insider_type)
            .scrape()?
            .to_table(Some(Insider::default_header()), Some(3));
        println!("{}", table_str);
    }

    Ok(())
}
