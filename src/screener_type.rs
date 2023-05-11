
use crate::common::UrlString;

#[allow(dead_code)]
pub enum ScreenerType {
    Financial,
    Overview,
    Ownership,
    Performance,
    Custom,
    Technical,
    Valuation,
}


impl UrlString for ScreenerType {
    fn to_url_string(&self) -> &str {
        match self {
            ScreenerType::Overview => "111",
            ScreenerType::Valuation => "121",
            ScreenerType::Ownership => "131",
            ScreenerType::Performance => "141",
            ScreenerType::Custom => "152",
            ScreenerType::Financial => "161",
            ScreenerType::Technical => "171",
        }

    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_string_of_screener_type() {
        let t = ScreenerType::Technical;
        assert_eq!(t.to_url_string(), "171");
    }
}
