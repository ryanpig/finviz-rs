use finviz_rs::{
    output::ToJson,
    forex::Forex,
    common::{Scrape, TableData},
};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {

    let json_data = Forex::default()
        .scrape().await?
        .into_iter()
        .take(2)
        .collect::<TableData>()
        .to_json(Some(Forex::default_header()))?;
    println!("{}", serde_json::to_string_pretty(&json_data)?);
    Ok(())
}
