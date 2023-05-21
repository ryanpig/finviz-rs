
finviz-rs
=====
The Rust library `finviz-rs` is a crate for fetching financial data and stock charts from Finviz website. It aims to provide a lightweight and type-safe library for accessing a range of financial information. 

[![Cargo Build & Test](https://github.com/ryanpig/finviz-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ryanpig/finviz-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/finviz-rs.svg)](https://crates.io/crates/finviz-rs)

# Table of contents
1. [Introduction](#introduction)
2. [Installation](#installation)
2. [Examples](#example)
    - [Fundament](#fundament)
    - [News](#news)
    - [Screener](#screener)
    - [Insider](#insider)
    - [Forex](#forex)
    - [Crypto](#crypto)
    - [Future](#future)
    - [Group](#group)
3. [Output types](#output)
    - [CSV](#csv)
4. [Others](#others]
    - [Retrieve multiple tables](#retrieve_multiple_data)

## Introduction <a name="introduction"></a>
#### Available data
The library offers access to various types of financial data, including:

- Stock charts: retrieve visually appealing and informative stock charts.
- Stock news: access the latest news and updates related to stocks.
- Stock fundamental data: retrieve comprehensive fundamental information about stocks.
- Screener: utilize a screener tool to filter and find stocks based on specific criteria.
- Insider information: get insights into insider trading activities and transactions.
- Forex performance: access information about the performance of various forex currency pairs.
- Crypto performance: retrieve data on the performance of different cryptocurrencies.
- Group: explore data related to stock groups or sectors.

#### Crate features
- Retrieve multiple tables: retrieve all possible combinations of enum variants by iterator 
- Output to a table: easily format and display the fetched data in a table format for convenient viewing and analysis.
- Output to a CSV file: export the data to a CSV file, allowing for seamless integration with other tools and workflows.
- Save stock chart image: capture and save stock chart images to local file system  


### Installation <a name="installation"></a>
```bash
cargo add finviz_rs
```


### Examples <a name="example"></a>
#### Fundament <a name="fundament"></a>
```bash
cargo run --example tickers
```
```rust
    // output json to table
    let fundament_info = Tickers::new("AAPL").scrape()?;
    println!("{}", from_dict_to_table(&fundament_info, 4).to_table(None, None));
```

```text
┌────────────────┬────────────┬───────────────┬────────────────────┬───────────────┬─────────────────┬─────────────────────┬──────────────┐
│ 52W High       │ -2.03%     │ 52W Low       │ 38.98%             │ 52W Range     │ 124.17 - 176.15 │ ATR                 │ 2.97         │
├────────────────┼────────────┼───────────────┼────────────────────┼───────────────┼─────────────────┼─────────────────────┼──────────────┤
│ Avg Volume     │ 59.16M     │ Beta          │ 1.29               │ Book/sh       │ 3.94            │ Cash/sh             │ 3.53         │
├────────────────┼────────────┼───────────────┼────────────────────┼───────────────┼─────────────────┼─────────────────────┼──────────────┤
│ Change         │ -0.68%     │ Current Ratio │ 0.90               │ Debt/Eq       │ 1.76            │ Dividend            │ 0.96         │
├────────────────┼────────────┼───────────────┼────────────────────┼───────────────┼─────────────────┼─────────────────────┼──────────────┤
│ Dividend %     │ 0.56%      │ EPS (ttm)     │ 5.89               │ EPS Q/Q       │ 0.00%           │ EPS next 5Y         │ 8.02%        │
├────────────────┼────────────┼───────────────┼────────────────────┼───────────────┼─────────────────┼─────────────────────┼──────────────┤
... skip below rows

```

To save the chart of a ticker
```rust
    // save a ticker's chart image to a file
    let tickers = Tickers::new("AAPL");
    tickers.ticker_charts(TimeFrameType::Daily, ChartType::ADVANCED, ".")?;
```

#### News <a name="news"></a>
```bash
cargo run --example news 
```

```rust
    let r = News::default()
        .scrape()?;
    println!("{}", r.news.to_table(Some(News::default_header()), Some(5)));
```

```text
┌─────────┬────────────────────────────────────────────────────┬───────────────────┬────────────────────────────────────────────────────┐
│ Time    │ Title                                              │ Source            │ Link                                               │
├─────────┼────────────────────────────────────────────────────┼───────────────────┼────────────────────────────────────────────────────┤
│ 10:36AM │ Dip Buyers Scorched by Cratering Bank Stocks Ru... │ www.bloomberg.com │ https://www.bloomberg.com/news/articles/2023-05... │
├─────────┼────────────────────────────────────────────────────┼───────────────────┼────────────────────────────────────────────────────┤
│ 10:00AM │ Abortion Bans Can Help Make This Cheap, Accessi... │ www.wsj.com       │ https://www.wsj.com/articles/abortion-bans-can-... │
├─────────┼────────────────────────────────────────────────────┼───────────────────┼────────────────────────────────────────────────────┤
│ 09:00AM │ RIP, Lumber-Futures Contract That Jumped During... │ www.wsj.com       │ https://www.wsj.com/articles/rip-lumber-futures... │
├─────────┼────────────────────────────────────────────────────┼───────────────────┼────────────────────────────────────────────────────┤
│ 08:09AM │ Credit crunch targeting manufacturing as intere... │ foxbusiness.com   │ https://foxbusiness.com/markets/credit-crunch-t... │
├─────────┼────────────────────────────────────────────────────┼───────────────────┼────────────────────────────────────────────────────┤
│ 08:00AM │ Who Would Want to Be a C.E.O.?                     │ www.nytimes.com   │ https://www.nytimes.com/2023/05/14/business/dea... │
└─────────┴────────────────────────────────────────────────────┴───────────────────┴────────────────────────────────────────────────────┘
```


#### Screener <a name="screener"></a>
```bash
cargo run --example screener
```

``` rust
    println!("--- Performance ---");
    let table_str = Screener::new(ScreenerType::Performance)
        .set_signal(SignalType::TopLosers)
        .set_order(OrderType::EPS)
        .scrape()?
        .to_table(None, Some(2));
    println!("{}", table_str);

    println!("--- Financial ---");
    let table_str= Screener::new(ScreenerType::Financial)
        .set_signal(SignalType::NewHigh)
        .set_order(OrderType::MarketCap)
        .scrape()?
        .to_table(None, Some(3));
    println!("{}", table_str);

```


```text
--- Performance ---
┌────────┬───────────┬────────────┬────────────┬───────────┬───────────┬──────────┬──────────────┬──────────────┬───────┬────────────┬────────────┬───────┬─────────┬─────────┐
│ Ticker │ Perf Week │ Perf Month │ Perf Quart │ Perf Half │ Perf Year │ Perf YTD │ Volatility W │ Volatility M │ Recom │ Avg Volume │ Rel Volume │ Price │ Change  │ Volume  │
├────────┼───────────┼────────────┼────────────┼───────────┼───────────┼──────────┼──────────────┼──────────────┼───────┼────────────┼────────────┼───────┼─────────┼─────────┤
│ AYTU   │ 20.26%    │ -18.58%    │ -42.86%    │ -40.61%   │ -80.00%   │ -51.32%  │ 16.14%       │ 10.41%       │ 2.00  │ 84.14K     │ 1.22       │ 1.84  │ -13.21% │ 102,664 │
└────────┴───────────┴────────────┴────────────┴───────────┴───────────┴──────────┴──────────────┴──────────────┴───────┴────────────┴────────────┴───────┴─────────┴─────────┘
--- Financial ---
┌────────┬────────────┬──────────┬─────────┬─────────┬─────────┬────────┬─────────┬───────────┬─────────┬─────────┬─────────┬──────────┬──────────┬───────┬─────────┬─────────────┐
│ Ticker │            │ Dividend │ ROA     │ ROE     │ ROI     │ Curr R │ Quick R │ LTDebt/Eq │ Debt/Eq │ Gross M │ Oper M  │ Profit M │ Earnings │ Price │ Change  │ Volume      │
│        │            │          │         │         │         │        │         │           │         │         │         │          │          │       │         │             │
│        │ Market Cap │          │         │         │         │        │         │           │         │         │         │          │          │       │         │             │
├────────┼────────────┼──────────┼─────────┼─────────┼─────────┼────────┼─────────┼───────────┼─────────┼─────────┼─────────┼──────────┼──────────┼───────┼─────────┼─────────────┤
│ LPCN   │ 25.21M     │ -        │ 9.80%   │ 10.50%  │ -33.30% │ 20.30  │ 20.30   │ 0.00      │ 0.00    │ -       │ -       │ -        │ Mar 10/b │ 4.31  │ -11.04% │ 191,855     │
├────────┼────────────┼──────────┼─────────┼─────────┼─────────┼────────┼─────────┼───────────┼─────────┼─────────┼─────────┼──────────┼──────────┼───────┼─────────┼─────────────┤
│ GSIT   │ 40.52M     │ -        │ -21.40% │ -25.10% │ -25.30% │ 6.50   │ 5.60    │ 0.00      │ 0.00    │ 59.90%  │ -45.20% │ -45.50%  │ May 16/a │ 5.09  │ 210.37% │ 104,612,024 │
└────────┴────────────┴──────────┴─────────┴─────────┴─────────┴────────┴─────────┴───────────┴─────────┴─────────┴─────────┴──────────┴──────────┴───────┴─────────┴─────────────┘
```

#### Insider <a name="insider"></a>
```bash
cargo run --example screener
```
```rust
    let table_str = Insider::default()
        .scrape()?
        .to_table(Some(Insider::default_header()), Some(3));
    println!("{}", table_str);
```

```text
┌────────┬───────────────────────┬─────────────────────────┬────────┬─────────────┬───────┬───────────┬───────────┬───────────────┬─────────────────┬────────────────────────────────────────────────────┐
│ Ticker │ Owner                 │ Relationship            │ Date   │ Transaction │ Cost  │ #Shares   │ Value ($) │ #Shares Total │ SEC Form 4      │ SEC Form 4 Link                                    │
├────────┼───────────────────────┼─────────────────────────┼────────┼─────────────┼───────┼───────────┼───────────┼───────────────┼─────────────────┼────────────────────────────────────────────────────┤
│ SKLZ   │ Paradise Andrew       │ Chief Executive Officer │ May 12 │ Buy         │ 0.52  │ 1,342,656 │ 698,181   │ 10,299,303    │ May 15 09:35 PM │ http://www.sec.gov/Archives/edgar/data/1801661/... │
├────────┼───────────────────────┼─────────────────────────┼────────┼─────────────┼───────┼───────────┼───────────┼───────────────┼─────────────────┼────────────────────────────────────────────────────┤
│ SKLZ   │ Paradise Andrew       │ Chief Executive Officer │ May 15 │ Buy         │ 0.55  │ 157,344   │ 86,539    │ 10,456,647    │ May 15 09:35 PM │ http://www.sec.gov/Archives/edgar/data/1801661/... │
├────────┼───────────────────────┼─────────────────────────┼────────┼─────────────┼───────┼───────────┼───────────┼───────────────┼─────────────────┼────────────────────────────────────────────────────┤
│ LINC   │ Harbour Ronald Edward │ Director                │ May 12 │ Sale        │ 6.50  │ 9,009     │ 58,558    │ 44,555        │ May 15 09:30 PM │ http://www.sec.gov/Archives/edgar/data/1286613/... │
└────────┴───────────────────────┴─────────────────────────┴────────┴─────────────┴───────┴───────────┴───────────┴───────────────┴─────────────────┴────────────────────────────────────────────────────┘
```

#### Forex <a name="forex"></a>
```bash
cargo run --example forex
```
```rust
    let table_str = Forex::default()
        .scrape()?
        .to_table(Some(Forex::default_header()), Some(3));
    println!("{}", table_str);

```

```text
┌────────┬────────┬───────────┬───────────┬──────────┬───────────┬────────────┬────────────┬───────────┬───────────┬──────────┐
│ Ticker │ Price  │ Perf 5Min │ Perf Hour │ Perf Day │ Perf Week │ Perf Month │ Perf Quart │ Perf Half │ Perf Year │ Perf YTD │
├────────┼────────┼───────────┼───────────┼──────────┼───────────┼────────────┼────────────┼───────────┼───────────┼──────────┤
│ EURGBP │ 0.8704 │ 0.00%     │ 0.00%     │ 0.28%    │ -0.09%    │ -1.67%     │ -2.16%     │ -0.21%    │ 2.78%     │ -1.56%   │
├────────┼────────┼───────────┼───────────┼──────────┼───────────┼────────────┼────────────┼───────────┼───────────┼──────────┤
│ USDJPY │ 136.30 │ 0.00%     │ -0.02%    │ 0.14%    │ 0.41%     │ 1.92%      │ 1.76%      │ -2.28%    │ 5.56%     │ 3.96%    │
├────────┼────────┼───────────┼───────────┼──────────┼───────────┼────────────┼────────────┼───────────┼───────────┼──────────┤
│ USDCHF │ 0.8960 │ 0.00%     │ 0.01%     │ 0.04%    │ -0.25%    │ 0.27%      │ -3.17%     │ -5.07%    │ -10.56%   │ -3.05%   │
└────────┴────────┴───────────┴───────────┴──────────┴───────────┴────────────┴────────────┴───────────┴───────────┴──────────┘
```

#### Crypto <a name="crypto"></a>
```bash
cargo run --example crypto
```
```rust
    let table_str = Crypto::default()
        .scrape()?
        .to_table(Some(Crypto::default_header()), Some(3));
    println!("{}", table_str);

```

```text
┌────────┬─────────┬───────────┬───────────┬──────────┬───────────┬────────────┬────────────┬───────────┬───────────┬──────────┐
│ Ticker │ Price   │ Perf 5Min │ Perf Hour │ Perf Day │ Perf Week │ Perf Month │ Perf Quart │ Perf Half │ Perf Year │ Perf YTD │
├────────┼─────────┼───────────┼───────────┼──────────┼───────────┼────────────┼────────────┼───────────┼───────────┼──────────┤
│ LTCEUR │ 82.5000 │ -0.07%    │ -0.36%    │ 2.75%    │ 10.92%    │ -5.67%     │ -13.32%    │ 43.58%    │ 25.63%    │ 27.55%   │
├────────┼─────────┼───────────┼───────────┼──────────┼───────────┼────────────┼────────────┼───────────┼───────────┼──────────┤
│ LTCBTC │ 0.0033  │ 0.00%     │ -0.30%    │ 2.71%    │ 10.15%    │ 4.38%      │ -19.80%    │ -5.83%    │ 46.85%    │ -21.23%  │
├────────┼─────────┼───────────┼───────────┼──────────┼───────────┼────────────┼────────────┼───────────┼───────────┼──────────┤
│ LTCUSD │ 89.6300 │ 0.11%     │ -0.11%    │ 2.63%    │ 10.98%    │ -6.69%     │ -12.05%    │ 50.97%    │ 31.38%    │ 29.49%   │
└────────┴─────────┴───────────┴───────────┴──────────┴───────────┴────────────┴────────────┴───────────┴───────────┴──────────┘

```

#### Future <a name="future"></a>
```bash
cargo run --example future
```
```rust
    let table_str = Future::default()
        .scrape()?
        .to_table(Some(Future::default_header()), Some(3));
    println!("{}", table_str);
```

```text
┌────────┬──────────────┬─────────┬───────┐
│ ticker │ label        │ group   │ perf  │
├────────┼──────────────┼─────────┼───────┤
│ VX     │ VIX          │ INDICES │ 12.13 │
├────────┼──────────────┼─────────┼───────┤
│ JO     │ Orange Juice │ SOFTS   │ 1.61  │
├────────┼──────────────┼─────────┼───────┤
│ LH     │ Lean Hogs    │ MEATS   │ 1.25  │
└────────┴──────────────┴─────────┴───────┘
```

#### Group <a name="group"></a>
```bash
cargo run --example future
```
```rust
    let table_str = Group::new(GroupBy::Industry, GroupType::Valuation, OrderBy::PerformanceWeek, Ordering::Ascending)
        .scrape()?
        .to_table(None, Some(5));
    println!("{}", table_str);

```

```text
┌────────────────────────┬────────────┬───────┬─────────┬──────┬──────┬──────┬───────┬───────┬─────────────┬─────────────┬───────────────┬────────┬────────┐
│ Name                   │ Market Cap │ P/E   │ Fwd P/E │ PEG  │ P/S  │ P/B  │ P/C   │ P/FCF │ EPS past 5Y │ EPS next 5Y │ Sales past 5Y │ Change │ Volume │
├────────────────────────┼────────────┼───────┼─────────┼──────┼──────┼──────┼───────┼───────┼─────────────┼─────────────┼───────────────┼────────┼────────┤
│ Footwear & Accessories │ 218.49B    │ 29.00 │ 23.54   │ 2.93 │ 2.75 │ 8.79 │ 15.87 │ 63.31 │ 21.32%      │ 9.89%       │ 7.52%         │ -3.85% │ 24.19M │
├────────────────────────┼────────────┼───────┼─────────┼──────┼──────┼──────┼───────┼───────┼─────────────┼─────────────┼───────────────┼────────┼────────┤
│ Solar                  │ 83.25B     │ 28.69 │ 16.35   │ 1.19 │ 1.89 │ 2.65 │ 6.27  │ 88.49 │ 24.73%      │ 24.07%      │ 31.63%        │ -1.87% │ 40.45M │
├────────────────────────┼────────────┼───────┼─────────┼──────┼──────┼──────┼───────┼───────┼─────────────┼─────────────┼───────────────┼────────┼────────┤
│ Coking Coal            │ 7.46B      │ 2.33  │ 5.72    │ 0.36 │ 0.61 │ 1.32 │ 5.17  │ 3.31  │ 36.66%      │ 6.53%       │ 14.94%        │ -1.48% │ 1.14M  │
├────────────────────────┼────────────┼───────┼─────────┼──────┼──────┼──────┼───────┼───────┼─────────────┼─────────────┼───────────────┼────────┼────────┤
│ Gold                   │ 247.00B    │ 30.11 │ 24.68   │ 5.10 │ 3.75 │ 1.38 │ 13.56 │ 91.32 │ 12.76%      │ 5.91%       │ 10.42%        │ 0.39%  │ 70.35M │
└────────────────────────┴────────────┴───────┴─────────┴──────┴──────┴──────┴───────┴───────┴─────────────┴─────────────┴───────────────┴────────┴────────┘
```

### Output <a name="output"></a>
#### Output to a CSV file <a name="csv"></a>
```rust
    Screener::new(ScreenerType::Performance)
        .scrape()?
        .to_csv_file("output.csv")?;
```


### Others <a name="others"></a>
#### Retrieve multiple tables <a name="retrieve_multiple_data"></a>
Powered by the crate [strum](https://crates.io/crates/strum), we're able to iterate all enum types as following example:

```bash
cargo run iter_all_enum_variants
    
```

```rust
    use strum::IntoEnumIterator;

    // fetch all types of insider trading data by iterating the enum of InsiderType
    for insider_type in InsiderType::iter() {
        let table_str = Insider::new(insider_type)
            .scrape()?
            .to_table(Some(Insider::default_header()), Some(3));
        println!("{}", table_str);
    }

```

