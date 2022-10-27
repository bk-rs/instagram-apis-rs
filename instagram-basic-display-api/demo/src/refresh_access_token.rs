/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p instagram-basic-display-api-demo --bin ig_b_d_a_refresh_access_token -- 'YOUR_LONG_LIVED_ACCESS_TOKEN'
*/

use std::{env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use instagram_basic_display_api::operations::{EndpointRet, RefreshAccessTokenEndpoint};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let long_lived_access_token = env::args().nth(1).unwrap();

    let client = IsahcClient::new()?;

    let me = RefreshAccessTokenEndpoint::new(long_lived_access_token);

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
