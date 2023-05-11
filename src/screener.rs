
use crate::screener_type::ScreenerType;
use crate::signal_type::SignalType;
use crate::order_type::OrderType;
use crate::common::{UrlString};

const BASE_URL: &'static str = "https://finviz.com/screener.ashx?";
pub struct Screener {
    base_type: ScreenerType,
    signal_type: Option<SignalType>,
    order_type: Option<OrderType>
}

impl Screener {
    pub fn new(base_type: ScreenerType) ->  Self {
        Self {base_type, signal_type: None, order_type: None}
    }

    pub fn set_signal(self: &mut Self, signal_type: SignalType) {
        self.signal_type = Some(signal_type);
    }

    pub fn set_order(self: &mut Self, order_type: OrderType) {
        self.order_type = Some(order_type);
    }

    pub fn to_url(self: &Self) ->  String {
        format!("{}v={}{}{}", BASE_URL, 
                            self.base_type.to_url_string(), 
                            self.signal_type.as_ref().map_or(String::new(), |s| format!("&s={}", s.to_url_string())),
                            self.order_type.as_ref().map_or(String::new(), |s| format!("&{}", s.to_url_string()))
               )

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_url_with_screenr_type() {
        let screener = Screener::new(ScreenerType::Performance);
        assert_eq!(screener.to_url(), "https://finviz.com/screener.ashx?v=141");
    }

    #[test]
    fn test_base_url_with_signal_type() {
        let mut screener = Screener::new(ScreenerType::Performance);
        screener.set_signal(SignalType::TopGainers);
        assert_eq!(screener.to_url(), "https://finviz.com/screener.ashx?v=141&s=ta_topgainers");
    }

    #[test]
    fn test_base_url_with_order_type() {
        let mut screener = Screener::new(ScreenerType::Performance);
        screener.set_signal(SignalType::TopLosers);
        screener.set_order(OrderType::Company);
        assert_eq!(screener.to_url(), "https://finviz.com/screener.ashx?v=141&s=ta_toplosers&o=company");
    }

}
