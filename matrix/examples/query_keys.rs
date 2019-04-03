extern crate matrix;

use std::env;
use std::collections::HashMap;

use matrix::models::Model200QueryKeys;
use matrix::models::QueryKeys;
use matrix::apis::configuration::ApiKey;

fn main() {

    let args: Vec<String> = env::args().collect();
    // args[1] -> pass access_token
    // args[2] -> user id
    // args[3] -> 'since' token of that sync request

    let apikey = ApiKey {
        prefix: None,
        key: args[1].to_string(),
    };

    let mut config = matrix::apis::configuration::Configuration::new();
    config.api_key = Some(apikey);

    let apicli = matrix::apis::client::APIClient::new(config);

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let device_ids: Vec<String> = Vec::new();

    map.insert(args[2].to_string(), device_ids);

    let qukeys = QueryKeys {
        timeout: Some(10000),
        device_keys: map,
        token: Some(args[3].to_string()),
    };

    let res = apicli.end_to_end_encryption_api().query_keys(qukeys); 
    println!("{:?}", res);
}
