extern crate matrix;

use std::env;

use matrix::models::SyncResponse;
use matrix::apis::configuration::ApiKey;
fn main() {

    let args: Vec<String> = env::args().collect();

    let mut config = matrix::apis::configuration::Configuration::new();
    
    let apikey = ApiKey {
        prefix:  Some("".to_owned()),
        key: "".to_owned(),
    };

    config.api_key = Some(apikey);

    // println!("{:?}", config);

    let apicli = matrix::apis::client::APIClient::new(config);

    let filter: &str =  &""; 
    let since: &str = &"".to_owned(); 
    let full_state: bool = false; 
    let set_presence: &str = &"offline".to_owned(); 
    let timeout: i32 = 15000;

    let res = apicli.room_participation_api().sync(&filter, since, full_state, set_presence, timeout); 
    println!("{:?}", res);
}
