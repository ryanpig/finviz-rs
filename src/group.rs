use crate::web_scraper::scrape_common;
use crate::common::{TableData, Scrape};
use std::fmt;
use strum::EnumIter;
use async_trait::async_trait;

#[doc(hidden)]
#[derive(EnumIter)]
pub enum OrderBy {
    Name,
    MarketCapitalization,
    PriceEarnings,
    ForwardPriceEarnings,
    PEG,
    PriceSales,
    PriceBook,
    PriceCash,
    PriceFreeCashFlow,
    DividendYield,
    EPSGrowthPast5Years,
    EPSGrowthNext5Years,
    SalesGrowthPast5Years,
    ShortInterestShare,
    AnalystRecommendation,
    PerformanceWeek,
    PerformanceMonth,
    PerformanceQuarter,
    PerformanceHalfYear,
    PerformanceYear,
    PerformanceYearToDate,
    AverageVolume3Month,
    RelativeVolume,
    Change,
    Volume,
    NumberOfStocks,
}

impl fmt::Display for OrderBy {

    /// Formats the OrderBy enum as a string to be used in URL parameters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            OrderBy::Name => "name",
            OrderBy::MarketCapitalization => "marketcap",
            OrderBy::PriceEarnings => "pe",
            OrderBy::ForwardPriceEarnings => "forwardpe",
            OrderBy::PEG => "peg",
            OrderBy::PriceSales => "ps",
            OrderBy::PriceBook => "pb",
            OrderBy::PriceCash => "pc",
            OrderBy::PriceFreeCashFlow => "pfcf",
            OrderBy::DividendYield => "dividendyield",
            OrderBy::EPSGrowthPast5Years => "eps5years",
            OrderBy::EPSGrowthNext5Years => "estltgrowth",
            OrderBy::SalesGrowthPast5Years => "sales5years",
            OrderBy::ShortInterestShare => "shortinterestshare",
            OrderBy::AnalystRecommendation => "recom",
            OrderBy::PerformanceWeek => "perf1w",
            OrderBy::PerformanceMonth => "perf4w",
            OrderBy::PerformanceQuarter => "perf13w",
            OrderBy::PerformanceHalfYear => "perf26w",
            OrderBy::PerformanceYear => "perf52w",
            OrderBy::PerformanceYearToDate => "perfytd",
            OrderBy::AverageVolume3Month => "averagevolume",
            OrderBy::RelativeVolume => "relativevolume",
            OrderBy::Change => "change",
            OrderBy::Volume => "volume",
            OrderBy::NumberOfStocks => "count",
        };
        write!(f, "{}", value)
    }
}


#[doc(hidden)]
#[allow(dead_code)]
#[derive(EnumIter)]
pub enum GroupType {
    Overview,
    Valuation,
    Performance,
    Custom,
}

impl fmt::Display for GroupType {

    /// Formats the GroupType enum as a string to be used in URL parameters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            GroupType::Overview => "&v=110",
            GroupType::Valuation => "&v=120",
            GroupType::Performance => "&v=140",
            GroupType::Custom => "&v=150",
        };
        write!(f, "{}", value)
    }
}

#[doc(hidden)]
#[allow(dead_code)]
#[derive(EnumIter)]
pub enum GroupBy {
    Sector,
    Industry,
    IndustryBasicMaterials,
    IndustryCommunicationServices,
    IndustryConsumerCyclical,
    IndustryConsumerDefensive,
    IndustryEnergy,
    IndustryFinancial,
    IndustryHealthcare,
    IndustryIndustrials,
    IndustryRealEstate,
    IndustryTechnology,
    IndustryUtilities,
    Country,
    Capitalization,
}

impl fmt::Display for GroupBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            GroupBy::Sector => "g=sector",
            GroupBy::Industry => "g=industry",
            GroupBy::IndustryBasicMaterials => "g=industry&sg=basicmaterials",
            GroupBy::IndustryCommunicationServices => "g=industry&sg=communicationservices",
            GroupBy::IndustryConsumerCyclical => "g=industry&sg=consumercyclical",
            GroupBy::IndustryConsumerDefensive => "g=industry&sg=consumerdefensive",
            GroupBy::IndustryEnergy => "g=industry&sg=energy",
            GroupBy::IndustryFinancial => "g=industry&sg=financial",
            GroupBy::IndustryHealthcare => "g=industry&sg=healthcare",
            GroupBy::IndustryIndustrials => "g=industry&sg=industrials",
            GroupBy::IndustryRealEstate => "g=industry&sg=realestate",
            GroupBy::IndustryTechnology => "g=industry&sg=technology",
            GroupBy::IndustryUtilities => "g=industry&sg=utilities",
            GroupBy::Country => "g=country",
            GroupBy::Capitalization => "g=capitalization",
        };
        write!(f, "{}", value)
    }
}

#[doc(hidden)]
#[allow(dead_code)]
#[derive(EnumIter)]
pub enum Ordering {
    Ascending, 
    Descending,
}

impl fmt::Display for Ordering {

    /// Formats the GroupBy enum as a string to be used in URL parameters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Ordering::Ascending => "", 
            Ordering::Descending => "-",
        };
        write!(f, "{}", value)
    }
}

/// This struct represents a group configuration.
///
/// It contains the following fields:
///
/// - group_by: Represents the grouping category 
/// - group_type: Represents the grouping type
/// - order_by: Represents the sorting criteria
/// - ordering: Represents the sorting order
///
/// # Example
///
/// ```
/// use finviz_rs::{
///     group::*,
///     output::ToTable,
///     common::Scrape,
/// };
/// 
/// #[tokio::main]
/// async fn main() -> Result<(),Box<dyn std::error::Error>>{
///     let table_str = Group::new(GroupBy::Industry, GroupType::Valuation, OrderBy::PerformanceWeek, Ordering::Ascending)
///         .scrape().await?
///         .to_table(None, Some(5));
///     println!("{}", table_str);
///     Ok(())
/// }
/// ```
///
/// The above example demonstrates how to retrive Group type of data into a table and print it.
pub struct Group {
    group_by: GroupBy,
    group_type: GroupType,
    order_by: OrderBy,
    ordering: Ordering,
}

impl Default for Group {
    fn default() -> Self {
        Self::new(GroupBy::Sector, GroupType::Overview, OrderBy::Name, Ordering::Ascending)
    }
}

impl Group {
    const BASE_URL: &'static str = "https://finviz.com/groups.ashx";

    /// Creates a new Group instance with the specified parameters.
    pub fn new(group_by: GroupBy, group_type: GroupType, order_by: OrderBy, ordering: Ordering) -> Self {
        Self{group_by, group_type, order_by, ordering}
    }

    /// Constructs the URL for the group with the specified parameters.
    fn get_url(&self) -> String {
        let order_by_url = format!("&o={}{}", self.ordering, self.order_by);
        format!("{}?{}{}{}", Group::BASE_URL, self.group_by, self.group_type, order_by_url)
    }
}

#[async_trait]
impl Scrape<TableData> for Group {

    /// The `scrape` function scrapes the group data from the specified URL and returns a `TableData` result
    async fn scrape(&self) -> Result<TableData, Box<dyn std::error::Error>> {
        let url = self.get_url() ;
        scrape_common(&url, false).await
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        {
            let group = Group::new(GroupBy::Industry, GroupType::Valuation, OrderBy::PerformanceWeek, Ordering::Ascending);
            assert_eq!(group.get_url(), "https://finviz.com/groups.ashx?g=industry&v=120&o=perf1w".to_string())
        }
        {
            let group = Group::new(GroupBy::Sector, GroupType::Performance, OrderBy::Name, Ordering::Descending);
            assert_eq!(group.get_url(), "https://finviz.com/groups.ashx?g=sector&v=140&o=-name".to_string())
        }
    }
}

