use clap::Parser;
use reqwest::Error;
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

const CURRENCY: &str = "usd";

async fn check_from_coingecko(name: &String) -> Result<CoinInfo, Error> {
    let api_url = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days=0", name.to_lowercase(), CURRENCY);
    let resp = reqwest::get(api_url).await?.json::<CoinInfo>().await?;
    println!("-- Price -- : {:.2} {}", resp.prices[0][1], CURRENCY.to_uppercase());
    println!("-- Market Cap -- : {:.2} {}", resp.market_caps[0][1], CURRENCY.to_uppercase());
    Ok(resp)
}

fn main() {
    let args = Cli::parse();
    println!("Checking {} ...", args.name.to_uppercase());

    let rt = Runtime::new().unwrap();
    rt.block_on(check_from_coingecko(&args.name)).unwrap();
}
