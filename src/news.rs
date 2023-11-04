use scraper::{Html, Selector};
use crate::web_scraper::get_html_body;
use crate::common::{TableData, Scrape};
use async_trait::async_trait;

/// `News` struct provides a way to scrape News page and convert the content into `TableData`
///
/// # Example
///
/// ```
/// use crate::finviz_rs::{
///     news::News,
///     output::ToTable,
///     common::Scrape,
/// };
/// 
/// #[tokio::main]
/// async fn main() -> Result<(),Box<dyn std::error::Error>>{
///     let r = News::default()
///         .scrape().await?;
///     println!("{}", r.news.to_table(Some(News::default_header()), Some(5)));
///     println!("{}", r.blogs.to_table(Some(News::default_header()), Some(5)));
///     Ok(())
/// }
/// ```
///
/// The above example demonstrates how to retrive both news and blogs of into two tables and print them.
pub struct News {}

/// a struct to store scraping data from `News`
#[derive(Debug)]
pub struct NewsData {
    /// data field to store scraping content of news
    pub news: TableData,
    /// data field to store scraping content of blogs 
    pub blogs: TableData,
}

impl Default for News {

    /// Create new instance of `News` using default constructor
    fn default() -> Self {
        Self::new()
    }
}

impl News {
    const BASE_URL: &'static str = "https://finviz.com/news.ashx";

    /// Create new instance of `News`
    pub fn new() -> Self {
        Self{}
    }


    /// Returns the default header of news or blogs type of `TableData`.
    pub fn default_header() -> Vec<String> {
        ["Time", "Title", "Source", "Link"].map(String::from).to_vec()
    }

}

#[async_trait]
impl Scrape<NewsData> for News {

    /// Scrapes the news data from the specified URL and return `TableData` on success, or `Box<dyn std::error::Error>` on failure
    async fn scrape(&self) -> Result<NewsData, Box<dyn std::error::Error>> {
        let body = get_html_body(News::BASE_URL).await?;
        let document = Html::parse_document(&body);

        let news_content_selector = Selector::parse("#news table").map_err(|err| err.to_string())?; 
        let news_content = document.select(&news_content_selector).next().ok_or("Cannot find News content ".to_string())?;

        let tables_selector = Selector::parse("table").map_err(|err| err.to_string())?;
        let mut tables = news_content.select(&tables_selector);

        let news = tables.next().ok_or("Cannot find News table".to_string())?;
        let blog = tables.next().ok_or("Cannot find Blog table".to_string())?;

        let news = parse_news_table(news)?;
        let blogs = parse_news_table(blog)?;
        //let blogs = news.clone();

        Ok( NewsData { news, blogs} )
    }
}


// helper function to parse news data and return `TableDat` on success, or `Box<dyn std::error::Error>` on failure
fn parse_news_table(table: scraper::element_ref::ElementRef<'_>) -> Result<TableData, Box<dyn std::error::Error>> {
    let row_selector = Selector::parse("tr")?;
    let rows = table.select(&row_selector);
    let mut data = Vec::new();


    for row in rows {
        let col_slector = Selector::parse("td")?;
        let mut cols = row.select(&col_slector);

        // parse the HTML <a>
        if let (Some(_), Some(date), Some(tag_a)) = (cols.next(), cols.next(), cols.next()) {
            let link_selector = Selector::parse("a")?;
            if let Some(link) = tag_a.select(&link_selector).next() {
                let date_text = date.inner_html();
                let title_text = link.inner_html();
                let link_href = link.value().attr("href").ok_or("N/A").unwrap().to_string();

                let source = if link_href.contains("feedproxy.google.com") {
                    link_href.split('/').nth(4).ok_or("Source not found".to_string())?
                } else {
                    link_href.split('/').nth(2).ok_or("Source not found".to_string())?
                };
                let row_data = vec![date_text, title_text, source.to_string(), link_href];
                data.push(row_data);
            } else {
                println!("Fail to parse tag a");
            }

        } else {
            println!("Fail to parse the row");
        }
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_news() {
        let news = News::default();
        let result = news.scrape().await;
        if result.is_err() { println!("{:?}", result); }
        assert!(result.is_ok(), "Failed");
        let r = result.unwrap();
        assert!(!r.news.is_empty(), "Empty news in the response");
        assert!(!r.blogs.is_empty(), "Empty blogs in the response");
    }
}
