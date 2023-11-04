use scraper::{Html, Selector};

use reqwest::Client;
use reqwest::header::USER_AGENT;
use crate::common::TableData;

use std::io::prelude::*;

/// Sends an HTTP GET request to the specified URL using the provided client.
///
/// # Arguments
///
/// * `client` - A reference to the `reqwest` client.
/// * `url` - The URL to which the request is sent.
///
/// # Returns
///
/// Returns a `Result` containing the `reqwest::blocking::Response` if the request is successful, or a `reqwest::Error` if an error occurs.
///
async fn send_request(client: &Client, url: &str) -> Result<reqwest::Response, reqwest::Error> {
    client
        .get(url)
        .header(USER_AGENT, "curl/7.82.0")
        .send().await
}


/// Scrapes table data from a web page using the provided URL and skip_header flag.
///
/// # Arguments
///
/// * `url` - The URL of the web page to scrape.
/// * `skip_header` - A boolean flag indicating whether to skip the table header row.
///
/// # Returns
///
/// Returns a `Result` containing the scraped table data as `TableData` if successful, or a `Box<dyn std::error::Error>` if an error occurs.
///
pub async fn scrape_common(url: &str, skip_header: bool) -> Result<TableData, Box<dyn std::error::Error>> {

    let body = get_html_body(url).await?;
    let document = Html::parse_document(&body);

    let table_selector = Selector::parse("table.styled-table-new")?;
    let row_selector = Selector::parse("tr")?;
    let cell_selector = Selector::parse("td")?;

    let frame: Result<Vec<Vec<String>>, &str> = document
        .select(&table_selector).next()
        .ok_or("Failed to select the table")
        .map(|table| {
            let n = if skip_header {1} else {0};
            let rows = table.select(&row_selector).skip(n);
            let mut frame = Vec::new();

            for row in rows {
                let cells = row.select(&cell_selector).skip(1); // Skip the first cell
                let mut info_dict = Vec::new();

                for cell in cells {
                    let text = cell.text().collect::<String>().trim().to_owned();
                    info_dict.push(text);
                }

                frame.push(info_dict);
            }

            frame
        });
    Ok(frame?)
}

/// Retrieves the HTML body of a web page specified by the URL.
///
/// # Arguments
///
/// * `url` - The URL of the web page.
///
/// # Returns
///
/// Returns a `Result` containing the HTML body as a string if successful, or a `Box<dyn std::error::Error>` if an error occurs.
///
pub async fn get_html_body(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = send_request(&client, url).await?;
    
    Ok(resp.text().await?)
}

/// Scrapes the chart image for a given ticker from the specified chart URL and saves it to the output directory.
///
/// # Arguments
///
/// * `chart_url` - The URL of the chart image.
/// * `ticker` - The ticker symbol associated with the chart.
/// * `out_dir` - The output directory where the chart image will be saved.
///
/// # Returns
///
/// Returns a `Result` containing the file path of the saved chart image if successful, or an error message as a string if an error occurs.
///
pub async fn scrape_chart_image(chart_url: &str, ticker: &str, out_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Getting image for ticker {} from URL: {} (out dir={})", ticker, chart_url, out_dir);
    let file_path = format!("{}/{}.png", out_dir, ticker);
    let mut file = std::fs::File::create(&file_path).unwrap();

    let client = Client::new();
    let resp = send_request(&client, chart_url).await?;
    let bytes_data = resp.bytes().await?;
    file.write_all(bytes_data.as_ref())?;
    Ok(file_path)
}
