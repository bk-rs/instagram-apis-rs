/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p instagram-basic-display-api-demo --bin me -- 'YOUR_ACCESS_TOKEN'
*/

use std::{env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use instagram_basic_display_api::operations::{EndpointRet, UserEndpoint};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let access_token = env::args().nth(1).unwrap();

    let client = IsahcClient::new()?;

    let me = UserEndpoint::me(access_token, true);

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
