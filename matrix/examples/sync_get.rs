extern crate matrix;

use std::env;

use matrix::models::SyncResponse;
use matrix::apis::configuration::ApiKey;
fn main() {

    let args: Vec<String> = env::args().collect();
    // args[1] -> pass access_token

    
    
    let apikey = ApiKey {
        prefix: None,
        key: args[1].to_string(),
    };

    let mut config = matrix::apis::configuration::Configuration::new();
    config.api_key = Some(apikey);

    let apicli = matrix::apis::client::APIClient::new(config);

    let filter: &str =  &"{}".to_owned(); 
    let since: &str = &"".to_owned(); 
    let full_state: bool = false; 
    let set_presence: &str = &"offline".to_owned(); 
    let timeout: i32 = 1500;

    let res = apicli.room_participation_api().sync(filter, since, full_state, &set_presence, timeout); 
    println!("{:?}", res);
}