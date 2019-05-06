
extern crate matrix;

use std::env;

use matrix::models::LoginRequestBody;
use matrix::models::UserIdentifier;

fn main() {

    let apicli = matrix::apis::client::APIClient::new(
        matrix::apis::configuration::Configuration::new()
    );

    let args: Vec<String> = env::args().collect();

    let resq_body = LoginRequestBody {
        _type: "m.login.password".to_string(),
        identifier: Some(UserIdentifier { 
            _type: "m.id.user".to_string(), 
            user: Some(args[1].to_string()),
            }
        ),
        password: Some(args[2].to_string()),
        initial_device_display_name: Some("my laptop".to_string()),
        address: None,
        medium: None,
        token: None,
        device_id: None,
    };

    let res = apicli.session_management_api().login(resq_body); 
    println!("{:?}", res);
}
