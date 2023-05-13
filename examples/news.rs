extern crate finviz_rs;

use finviz_rs::{
    news::News,
    output::ToTable
};

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let r = News::default()
        .scrape_news()?;
    println!("{}", r.news.to_table(Some(News::default_header()), Some(5)));
    println!("{}", r.blogs.to_table(Some(News::default_header()), Some(5)));
    Ok(())
}
