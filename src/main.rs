mod model;
use crate::model::{Quote, Res};
use std::env;

#[derive(PartialEq, PartialOrd)]
struct Sortable {
    ticker: model::Ticker,
    price: f64,
    market_cap: f64,
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let api_key = env::var("CMC_PRO_API_KEY").unwrap();
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=BTC,ETH,XRP,BCH,LTC,XLM,ETC";
    let res: Res = surf::get(url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .recv_json()
        .await?;

    let mut sss: Vec<Sortable> = res
        .data
        .iter()
        .clone()
        .map(|d| {
            let q = d.1.quote;
            let t = d.0;
            match q {
                Quote::USD { price, market_cap } => Sortable {
                    ticker: *t,
                    price,
                    market_cap,
                },
            }
        })
        .collect();
    sss.sort_by(|a, b| b.market_cap.partial_cmp(&a.market_cap).unwrap());
    for d in sss.iter() {
        println!("{:?}\t{}\t{}", d.ticker, d.price, d.market_cap);
    }
    //dbg!(res);
    Ok(())
}
