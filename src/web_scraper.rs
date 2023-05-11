use scraper::{Html, Selector};

use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

fn send_request(client: &Client, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    client
        .get(url)
        .header(USER_AGENT, "curl/7.82.0")
        .send()
}

/*
The web_scrape() takes a URL as input and returns a vector of vectors of strings, representing the scraped table. Each inner vector represents a row, and each string represents a cell value.
*/
pub fn text_scrape(url: &str) -> Vec<Vec<String>> {
    let client = Client::new();
    let resp = send_request(&client, url).expect("Failed to send request");


    let body = resp.text().expect("Failed to retrieve response body");
    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table.table-light").expect("Failed to parse table selector");
    let row_selector = Selector::parse("tr").expect("Failed to parse row selector");
    let cell_selector = Selector::parse("td").expect("Failed to parse cell selector");

    let table = document.select(&table_selector).next().expect("Table not found");
    let rows = table.select(&row_selector);
    let mut frame = Vec::new();

    for row in rows {
        let cells = row.select(&cell_selector).skip(1); // Skip the first cell
        let mut info_dict = Vec::new();

        for cell in cells {
            info_dict.push(cell.text().collect());
        }

        frame.push(info_dict);
    }

    frame
}

