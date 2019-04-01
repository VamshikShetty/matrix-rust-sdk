
extern crate matrix;

use matrix::models::LoginRequestBody;
use matrix::models::UserIdentifier;

fn main() {

    let apicli = matrix::apis::client::APIClient::new(
        matrix::apis::configuration::Configuration::new()
    );

    let resq_body = LoginRequestBody {
        _type: "m.login.password".to_string(),
        identifier: Some(UserIdentifier { 
            _type: "m.id.user".to_string(), 
            user: Some("dummy_test".to_string()),
            }
        ),
        password: Some("dummy@test".to_string()),
        initial_device_display_name: Some("my laptop".to_string()),
        address: None,
        medium: None,
        token: None,
        device_id: None,
    };

    let res = apicli.session_management_api().login(resq_body); 
    println!("{:?}", res);
}
