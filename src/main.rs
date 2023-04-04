use clap::Parser;
use tokio::runtime::Runtime;
use serde::{Deserialize,Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the coin to check
    #[arg(short, long)]
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinInfo {
    prices: [[f64; 2]; 1],
    market_caps: [[f64; 2]; 1],
    total_volumes: [[f64; 2]; 1]
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinInfoError {
    error: String
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ResponseType {
  Ok(CoinInfo),
  Err(CoinInfoError),
}

const CURRENCY: &str = "usd";

async fn check_from_coingecko(name: &String) {
    let api_url = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days=0", name.to_lowercase(), CURRENCY);
    let resp = reqwest::get(api_url).await.unwrap();

    match resp.status() {
        reqwest::StatusCode::OK => {
            match resp.json::<CoinInfo>().await {
                Ok(data) => {
                    println!("-- Price -- : {:.2} {}", data.prices[0][1], CURRENCY.to_uppercase());
                    println!("-- Market Cap -- : {:.2} {}", data.market_caps[0][1], CURRENCY.to_uppercase());
                },
                Err(_) => println!("Parsing error")
            }
        }
        _ => println!("Coin not found")
    }
}

fn main() {
    let args = Cli::parse();
    println!("Checking {} ...", args.name.to_uppercase());

    let rt = Runtime::new().unwrap();
    rt.block_on(check_from_coingecko(&args.name));
}
