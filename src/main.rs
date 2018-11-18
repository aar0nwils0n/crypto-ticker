use std::collections::HashMap;

extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    price: f64, 
    volume_24h: f64,
    market_cap: f64, 
    percent_change_1h: f64, 
    percent_change_24h: f64, 
    percent_change_7d: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Coin {
    id: u32, 
    name: String, 
    symbol: String, 
    website_slug: String, 
    rank: u32, 
    circulating_supply: f64, 
    total_supply: f64, 
    quotes: HashMap<String, Quote>,
    last_updated: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct Ticker {
    data : HashMap<String, Coin>
}

fn add_padding(string : &String, column_length : u32) -> String {
    let padding_length : u32 = ((column_length - 2) - (string.len() as u32)) / 2;
    let padding : String = (0..padding_length as i16).into_iter().map(|_| " ").collect::<String>();
    let tail : String = String::from(if string.len() as i16 % 2 == 0 { "" } else { " " });
    format!("{}{}{}{}", padding, string, padding, tail)
}

fn coin_to_row(coin : Coin, column_length : u32) -> String {
    let price = coin.quotes.get("USD")
        .map(|quote| quote.price.to_string())
        .unwrap_or_default();
        
    let cols = [
        coin.name,
        coin.symbol,
        coin.total_supply.to_string(),
        price
    ]
    .into_iter()
    .map(|col| add_padding(col, column_length))
    .collect::<Vec<String>>()
    ;


    format!("|{}|", cols.join("|"))
}

fn main() -> Result<(), reqwest::Error> {
    loop {

        let ticker: Ticker = reqwest::Client::new()
            .get("https://api.coinmarketcap.com/v2/ticker/?limit=10")
            .send()?
            .json()?;

        

        let column_length : u32 = 18;
        let columns = ["NAME", "SYMBOL", "TOTAL SUPPLY", "PRICE (USD)"]
            .into_iter()
            .map(|&col| add_padding(&String::from(col), column_length));

        let border : String = (0..=(column_length - 1) * columns.len() as u32).map(|_| "-").collect::<String>();
        let header : String = format!(
            "{}\n|{}|",
            border,
            columns.collect::<Vec<String>>().join("|"),
        );

        let ticker_str : String = ticker.data
            .into_iter()
            .fold(
                String::from(""),
                |acc, (_, coin)| format!("{}\n{}\n{}", acc, border, coin_to_row(coin, column_length))
            );

        println!("{}{}", header, ticker_str);

        let duration = std::time::Duration::from_secs(5);
        std::thread::sleep(duration)
    }
}

