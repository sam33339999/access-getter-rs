use serde::{Serialize, Deserialize};
use reqwest::Error;
// use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct OathRes {
    #[serde(rename = "access_token")]
    token: String,
    #[serde(rename = "expires_in")]
    exp: u32,
    #[serde(rename = "token_type")]
    t: String
}


#[tokio::main]
async fn main() -> Result<(), Error>{
    dotenv::dotenv().ok();
    
    let api_url = "https://api.line.me/v2/oauth/accessToken";

    // let mut map = HashMap::new();
    // map.insert("lang", "rust");

    let message_cliend_id = std::env::var("MESSAGE_CLIENT_ID").expect("MESSAGE_CLIENT_ID 沒有設定");
    let message_secret = std::env::var("MESSAGE_SECRET").expect("MESSAGE_SECRET 沒有設定");
    let login_cliend_id = std::env::var("LOGIN_CLIENT_ID").expect("LOGIN_CLIENT_ID 沒有設定");
    let login_cliend_secret = std::env::var("LOGIN_SECRET").expect("LOGIN_SECRET 沒有設定");
    
    
    let client = reqwest::Client::new();

    let params = [
        [
            ("grant_type", "client_credentials"),
            ("client_id", &message_cliend_id),
            ("client_secret", &message_secret),
        ],
        [
            ("grant_type", "client_credentials"),
            ("client_id", &login_cliend_id),
            ("client_secret", &login_cliend_secret),
        ],
    ];


    let res = client.post(api_url)
        .form(&params[0])
        .send()
        .await?
        .text_with_charset("utf-8")
        .await?;
    let o: OathRes = serde_json::from_str(res.as_str()).unwrap();

    println!("message: {:#?}", o);

    let res = client.post(api_url)
        .form(&params[1])
        .send()
        .await?
        .text_with_charset("utf-8")
        .await?;
    let o: OathRes = serde_json::from_str(res.as_str()).unwrap();

    println!("login: {:#?}", o);



    Ok(())
}
