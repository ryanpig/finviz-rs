use crate::common::DisplayString;
use std::fmt;
use strum::EnumIter;

/// Represents the signal type that can be passed as parameter in `Screener`.
#[doc(hidden)]
#[allow(dead_code)]
#[derive(Clone, Copy, EnumIter)]
pub enum SignalType {
    TopGainers,
    TopLosers,
    NewHigh,
    NewLow,
    MostVolatile,
    MostActive,
    UnusualVolume,
    Overbought,
    Oversold,
    Downgrades,
    Upgrades,
    EarningsBefore,
    EarningsAfter,
    RecentInsiderBuying,
    RecentInsiderSelling,
    MajorNews,
    HorizontalSR,
    TLResistance,
    TLSupport,
    WedgeUp,
    WedgeDown,
    Wedge,
    TriangleAscending,
    TriangleDescending,
    ChannelUp,
    ChannelDown,
    Channel,
    DoubleTop,
    DoubleBottom,
    MultipleTop,
    MultipleBottom,
    HeadShoulders,
    HeadShouldersInverse,
}

impl fmt::Display for SignalType {

    /// Formats the `SignalType` that can be used as URL parameter in `Screener` 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            SignalType::TopGainers => "ta_topgainers",
            SignalType::TopLosers => "ta_toplosers",
            SignalType::NewHigh => "ta_newhigh",
            SignalType::NewLow => "ta_newlow",
            SignalType::MostVolatile => "ta_mostvolatile",
            SignalType::MostActive => "ta_mostactive",
            SignalType::UnusualVolume => "ta_unusualvolume",
            SignalType::Overbought => "ta_overbought",
            SignalType::Oversold => "ta_oversold",
            SignalType::Downgrades => "n_downgrades",
            SignalType::Upgrades => "n_upgrades",
            SignalType::EarningsBefore => "n_earningsbefore",
            SignalType::EarningsAfter => "n_earningsafter",
            SignalType::RecentInsiderBuying => "it_latestbuys",
            SignalType::RecentInsiderSelling => "it_latestsales",
            SignalType::MajorNews => "n_majornews",
            SignalType::HorizontalSR => "ta_p_horizontal",
            SignalType::TLResistance => "ta_p_tlresistance",
            SignalType::TLSupport => "ta_p_tlsupport",
            SignalType::WedgeUp => "ta_p_wedgeup",
            SignalType::WedgeDown => "ta_p_wedgedown",
            SignalType::TriangleAscending => "ta_p_wedgeresistance",
            SignalType::TriangleDescending => "ta_p_wedgesupport",
            SignalType::Wedge => "ta_p_wedge",
            SignalType::ChannelUp => "ta_p_channelup",
            SignalType::ChannelDown => "ta_p_channeldown",
            SignalType::Channel => "ta_p_channel",
            SignalType::DoubleTop => "ta_p_doubletop",
            SignalType::DoubleBottom => "ta_p_doublebottom",
            SignalType::MultipleTop => "ta_p_multipletop",
            SignalType::MultipleBottom => "ta_p_multiplebottom",
            SignalType::HeadShoulders => "ta_p_headandshoulders",
            SignalType::HeadShouldersInverse => "ta_p_headandshouldersinv",
        };
        write!(f, "{}", value)
    }
}


impl DisplayString for SignalType {

    /// Formats the `SignalType` to reutrn a description 
    fn to_display_string(&self) -> &str {
        match self {
            SignalType::TopGainers => "Top Gainers",
            SignalType::TopLosers => "Top Losers",
            SignalType::NewHigh => "New High",
            SignalType::NewLow => "New Low",
            SignalType::MostVolatile => "Most Volatile",
            SignalType::MostActive => "Most Active",
            SignalType::UnusualVolume => "Unusual Volume",
            SignalType::Overbought => "Overbought",
            SignalType::Oversold => "Oversold",
            SignalType::Downgrades => "Downgrades",
            SignalType::Upgrades => "Upgrades",
            SignalType::EarningsBefore => "Earnings Before",
            SignalType::EarningsAfter => "Earnings After",
			SignalType::RecentInsiderBuying => "Recent Insider Buying",
			SignalType::RecentInsiderSelling => "Recent Insider Selling",
			SignalType::MajorNews => "Major News",
			SignalType::HorizontalSR => "Horizontal S/R",
			SignalType::TLResistance => "TL Resistance",
			SignalType::TLSupport => "TL Support",
			SignalType::WedgeUp => "Wedge Up",
			SignalType::WedgeDown => "Wedge Down",
			SignalType::Wedge => "Wedge",
			SignalType::TriangleAscending => "Triangle Ascending",
			SignalType::TriangleDescending => "Triangle Descending",
			SignalType::ChannelUp => "Channel Up",
			SignalType::ChannelDown => "Channel Down",
			SignalType::Channel => "Channel",
			SignalType::DoubleTop => "Double Top",
			SignalType::DoubleBottom => "Double Bottom",
			SignalType::MultipleTop => "Multiple Top",
			SignalType::MultipleBottom => "Multiple Bottom",
			SignalType::HeadShoulders => "Head & Shoulders",
			SignalType::HeadShouldersInverse => "Head & Shoulders Inverse",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_string_of_signal_type() {
        let t = SignalType::TopGainers;
        assert_eq!(t.to_string(), "ta_topgainers");
    }

    #[test]
    fn test_display_string_of_signal_type() {
        let t = SignalType::TopGainers;
        assert_eq!(t.to_display_string(), "Top Gainers");
    }
}
