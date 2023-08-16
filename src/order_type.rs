use std::fmt;
use strum::EnumIter;

/// Represents the possible order type that can be passed as parameter in `Screener`.
#[doc(hidden)]
#[allow(dead_code)]
#[derive(Clone, Copy, EnumIter)]
pub enum OrderType {
    Ticker,
    Company,
    Sector,
    Industry,
    Country,
    MarketCap,
    PriceEarnings,
    ForwardPriceEarnings,
    PEG,
    PriceSales,
    PriceBook,
    PriceCash,
    PriceFreeCashFlow,
    DividendYield,
    PayoutRatio,
    EPS,
    EPSGrowthThisYear,
    EPSGrowthNextYear,
    EPSGrowthPast5Years,
    EPSGrowthNext5Years,
    SalesGrowthPast5Years,
    EPSGrowthQtrOverQtr,
    SalesGrowthQtrOverQtr,
    SharesOutstanding,
    SharesFloat,
    InsiderOwnership,
    InsiderTransactions,
    InstitutionalOwnership,
    InstitutionalTransactions,
    ShortInterestShare,
    ShortInterestRatio,
    EarningsDate,
    ReturnOnAssets,
    ReturnOnEquity,
    ReturnOnInvestment,
    CurrentRatio,
    QuickRatio,
    LTDebtEquity,
    TotalDebtEquity,
    GrossMargin,
    OperatingMargin,
    NetProfitMargin,
    AnalystRecommendation,
    PerformanceWeek,
    PerformanceMonth,
    PerformanceQuarter,
    PerformanceHalfYear,
    PerformanceYear,
    PerformanceYearToDate,
    Beta,
    AverageTrueRange,
    VolatilityWeek,
    VolatilityMonth,
    Sma20,
    Sma50,
    Sma200,
    High50Day,
    Low50Day,
    High52Week,
    Low52Week,
    RelativeStrengthIndex,
    AverageVolume3Month,
    RelativeVolume,
    Change,
    ChangeFromOpen,
    Gap,
    Volume,
    Price,
    TargetPrice,
    IPODate,
}

impl fmt::Display for OrderType {

    /// Formats the `OrderType` that can be used as URL parameter in `Screener` 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            OrderType::Ticker => "ticker",
            OrderType::Company => "company",
            OrderType::Sector => "sector",
            OrderType::Industry => "industry",
            OrderType::Country => "country",
            OrderType::MarketCap => "marketcap",
            OrderType::PriceEarnings => "pe",
            OrderType::ForwardPriceEarnings => "forwardpe",
            OrderType::PEG => "peg",
            OrderType::PriceSales => "ps",
            OrderType::PriceBook => "pb",
            OrderType::PriceCash => "pc",
            OrderType::PriceFreeCashFlow => "pfcf",
            OrderType::DividendYield => "dividendyield",
            OrderType::PayoutRatio => "payoutratio",
            OrderType::EPS => "eps",
            OrderType::EPSGrowthThisYear => "epsyoy",
            OrderType::EPSGrowthNextYear => "epsyoy1",
            OrderType::EPSGrowthPast5Years => "eps5years",
            OrderType::EPSGrowthNext5Years => "estltgrowth",
            OrderType::SalesGrowthPast5Years => "sales5years",
            OrderType::EPSGrowthQtrOverQtr => "epsqoq",
            OrderType::SalesGrowthQtrOverQtr => "salesqoq",
            OrderType::SharesOutstanding => "sharesoutstanding2",
            OrderType::SharesFloat => "sharesfloat",
            OrderType::InsiderOwnership => "insiderown",
            OrderType::InsiderTransactions => "insidertrans",
            OrderType::InstitutionalOwnership => "instown",
            OrderType::InstitutionalTransactions => "insttrans",
            OrderType::ShortInterestShare => "shortinterestshare",
            OrderType::ShortInterestRatio => "shortinterestratio",
            OrderType::EarningsDate => "earningsdate",
            OrderType::ReturnOnAssets => "roa",
            OrderType::ReturnOnEquity => "roe",
            OrderType::ReturnOnInvestment => "roi",
            OrderType::CurrentRatio => "curratio",
            OrderType::QuickRatio => "quickratio",
            OrderType::LTDebtEquity => "ltdebteq",
            OrderType::TotalDebtEquity => "debteq",
            OrderType::GrossMargin => "grossmargin",
            OrderType::OperatingMargin => "opermargin",
            OrderType::NetProfitMargin => "netmargin",
            OrderType::AnalystRecommendation => "recom",
            OrderType::PerformanceWeek => "perf1w",
            OrderType::PerformanceMonth => "perf4w",
            OrderType::PerformanceQuarter => "perf13w",
            OrderType::PerformanceHalfYear => "perf26w",
            OrderType::PerformanceYear => "perf52w",
            OrderType::PerformanceYearToDate => "perfytd",
            OrderType::Beta => "beta",
            OrderType::AverageTrueRange => "averagetruerange",
            OrderType::VolatilityWeek => "volatility1w",
            OrderType::VolatilityMonth => "volatility4w",
            OrderType::Sma20 => "sma20",
            OrderType::Sma50 => "sma50",
            OrderType::Sma200 => "sma200",
            OrderType::High50Day => "high50d",
            OrderType::Low50Day => "low50d",
            OrderType::High52Week => "high52w",
            OrderType::Low52Week => "low52w",
            OrderType::RelativeStrengthIndex => "rsi",
            OrderType::AverageVolume3Month => "averagevolume",
            OrderType::RelativeVolume => "relativevolume",
            OrderType::Change => "change",
            OrderType::ChangeFromOpen => "changeopen",
            OrderType::Gap => "gap",
            OrderType::Volume => "volume",
            OrderType::Price => "price",
            OrderType::TargetPrice => "targetprice",
            OrderType::IPODate => "ipodate",
        };
        write!(f, "{}", value)
    }
}


/// Represents the possible ordering in `Screener`.
#[doc(hidden)]
#[allow(dead_code)]
#[derive(Clone, Copy, EnumIter)]
pub enum Ordering {
    Ascending, 
    Descending,
}

impl fmt::Display for Ordering {

    /// Formats the enum as a string to be used in URL parameters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Ordering::Ascending => "", 
            Ordering::Descending => "-",
        };
        write!(f, "{}", value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_string_of_signal_type() {
        let t = OrderType::PerformanceWeek;
        assert_eq!(t.to_string(), "perf1w");
    }

}
