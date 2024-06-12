use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Kline {
    close: String, #[allow(dead_code)]
    open_time: u64, #[allow(dead_code)]
    open: String, #[allow(dead_code)]
    high: String, #[allow(dead_code)]
    low: String, #[allow(dead_code)]
    volume: String, #[allow(dead_code)]
    close_time: u64, #[allow(dead_code)]
    quote_asset_volume: String, #[allow(dead_code)]
    number_of_trades: u64, #[allow(dead_code)]
    taker_buy_base_asset_volume: String, #[allow(dead_code)]
    taker_buy_quote_asset_volume: String, #[allow(dead_code)]
    ignore: String,
}

async fn fetch_klines(client: &Client, symbol: &str, interval: &str) -> Result<Vec<Kline>, Box<dyn Error>> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}",
        symbol, interval
    );

    let response = client.get(&url).send().await?;
    let data: Vec<Vec<serde_json::Value>> = response.json().await?;
    let klines: Vec<Kline> = data.into_iter().map(|kline| Kline {
        open_time: kline[0].as_u64().unwrap(),
        open: kline[1].as_str().unwrap().to_string(),
        high: kline[2].as_str().unwrap().to_string(),
        low: kline[3].as_str().unwrap().to_string(),
        close: kline[4].as_str().unwrap().to_string(),
        volume: kline[5].as_str().unwrap().to_string(),
        close_time: kline[6].as_u64().unwrap(),
        quote_asset_volume: kline[7].as_str().unwrap().to_string(),
        number_of_trades: kline[8].as_u64().unwrap(),
        taker_buy_base_asset_volume: kline[9].as_str().unwrap().to_string(),
        taker_buy_quote_asset_volume: kline[10].as_str().unwrap().to_string(),
        ignore: kline[11].as_str().unwrap().to_string(),
    }).collect();

    Ok(klines)
}

fn calculate_moving_average(klines: &[Kline]) -> f64 {
    let sum: f64 = klines.iter().map(|kline| kline.close.parse::<f64>().unwrap()).sum();
    sum / klines.len() as f64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let symbol = "BTCUSDT";
    let interval = "1h";
    
    let klines = fetch_klines(&client, symbol, interval).await?;

    let current_price: f64 = klines.last().unwrap().close.parse().unwrap();

    // Calculate 24-hour sma
    let ma_24h = if klines.len() >= 24 {
        calculate_moving_average(&klines[klines.len() - 24..])
    } else {
        calculate_moving_average(&klines)
    };

    // Calculate 7-day sma
    let ma_7d = if klines.len() >= 168 {
        calculate_moving_average(&klines[klines.len() - 168..])
    } else {
        calculate_moving_average(&klines)
    };

    let fair_value = (current_price + ma_24h + ma_7d) / 3.0;

    println!("Current price of Bitcoin: ${:.2}", current_price);
    println!("24-hour moving average: ${:.2}", ma_24h);
    println!("7-day moving average: ${:.2}", ma_7d);
    println!("Fair value of Bitcoin: ${:.2}", fair_value);
    
    Ok(())
}
