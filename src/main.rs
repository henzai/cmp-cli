mod model;
use crate::model::{Quote, Res};
use std::env;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let api_key = env::var("CMC_PRO_API_KEY").unwrap();
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=BTC,ETH,XRP,BCH,LTC,XLM,ETC";
    let res: Res = surf::get(url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .recv_json()
        .await?;
    for d in res.data.iter() {
        let q = &d.1.quote;
        match q {
            Quote::USD { price, market_cap } => println!("{:?}: {}\t{}", d.0, price, market_cap),
        }
    }
    //dbg!(res);
    Ok(())
}
