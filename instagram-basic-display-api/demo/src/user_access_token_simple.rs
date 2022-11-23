/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p instagram-basic-display-api-demo --bin ig_b_d_a_user_access_token_simple -- 'YOUR_APP_SECRET' 'YOUR_SHORT_LIVED_USER_ACCESS_TOKEN'
*/

use std::{env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use instagram_basic_display_api::operations::{
    EndpointRet, ExchangeSlAccessTokenForLlAccessTokenEndpoint, RefreshAccessTokenEndpoint,
    UserEndpoint, UserMediasEndpoint,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let app_secret = env::args().nth(1).unwrap();
    let short_lived_user_access_token = env::args().nth(2).unwrap();

    let client = IsahcClient::new()?;

    //
    let me = UserEndpoint::me(&short_lived_user_access_token, false);
    let ret = client.respond_endpoint(&me).await?;
    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);
        }
        EndpointRet::Other(_) => {
            panic!("{:?}", ret);
        }
    }

    //
    let exchange_sl_access_token_for_ll_access_token =
        ExchangeSlAccessTokenForLlAccessTokenEndpoint::new(
            &app_secret,
            &short_lived_user_access_token,
        );
    let ret = client
        .respond_endpoint(&exchange_sl_access_token_for_ll_access_token)
        .await?;
    let long_lived_user_access_token = match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);

            ok_json.access_token.to_owned()
        }
        EndpointRet::Other(_) => {
            panic!("{:?}", ret);
        }
    };

    //
    let user_medias = UserMediasEndpoint::me(&long_lived_user_access_token, 1, None);
    let ret = client.respond_endpoint(&user_medias).await?;
    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);
        }
        EndpointRet::Other(_) => {
            panic!("{:?}", ret);
        }
    }

    //
    let refresh_access_token = RefreshAccessTokenEndpoint::new(&long_lived_user_access_token);
    let ret = client.respond_endpoint(&refresh_access_token).await?;
    let long_lived_user_access_token_with_refreshed = match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);

            ok_json.access_token.to_owned()
        }
        EndpointRet::Other(_) => {
            panic!("{:?}", ret);
        }
    };

    //
    let me = UserEndpoint::me(&long_lived_user_access_token_with_refreshed, true);
    let ret = client.respond_endpoint(&me).await?;
    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);
        }
        EndpointRet::Other(_) => {
            panic!("{:?}", ret);
        }
    }

    Ok(())
}
