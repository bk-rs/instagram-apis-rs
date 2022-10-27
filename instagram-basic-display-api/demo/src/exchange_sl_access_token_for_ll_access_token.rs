/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p instagram-basic-display-api-demo --bin ig_b_d_a_exchange_sl_access_token_for_ll_access_token -- 'YOUR_APP_SECRET' 'YOUR_SHORT_LIVED_ACCESS_TOKEN'
*/

use std::{env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use instagram_basic_display_api::operations::{
    EndpointRet, ExchangeSlAccessTokenForLlAccessTokenEndpoint,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let app_secret = env::args().nth(1).unwrap();
    let short_lived_access_token = env::args().nth(2).unwrap();

    let client = IsahcClient::new()?;

    let me =
        ExchangeSlAccessTokenForLlAccessTokenEndpoint::new(app_secret, short_lived_access_token);

    let ret = client.respond_endpoint(&me).await?;

    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);
        }
        EndpointRet::Other(_) => {
            println!("{:?}", ret);
        }
    }

    Ok(())
}
