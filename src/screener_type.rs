use std::fmt;

/// Represents the possible screener type that can be passed as parameter in `Screener`.
#[doc(hidden)]
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


impl fmt::Display for ScreenerType {

    /// Formats the `ScreenerType` that can be used as URL parameter in `Screener` 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            ScreenerType::Overview => "111",
            ScreenerType::Valuation => "121",
            ScreenerType::Ownership => "131",
            ScreenerType::Performance => "141",
            ScreenerType::Custom => "152",
            ScreenerType::Financial => "161",
            ScreenerType::Technical => "171",
        };
        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_string_of_screener_type() {
        let t = ScreenerType::Technical;
        assert_eq!(t.to_string(), "171");
    }
}
