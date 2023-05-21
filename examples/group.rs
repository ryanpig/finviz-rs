extern crate finviz_rs;

use finviz_rs::{
    group::*,
    output::ToTable,
    common::Scrape
};

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let table_str = Group::new(GroupBy::Industry, GroupType::Valuation, OrderBy::PerformanceWeek, Ordering::Ascending)
        .scrape()?
        .to_table(None, Some(5));
    println!("{}", table_str);
    Ok(())
}
