
extern crate matrix;
// extern crate futures;
// extern crate hyper;
// extern crate tokio_core;

// use hyper::client::HttpConnector;
// use hyper::Client;
// use tokio_core::reactor::Core;

use matrix::models::LoginRequestBody;
use matrix::models::UserIdentifier;

fn main() {
    // let mut core = Core::new().expect("failed to init core");
    // let handle = core.handle();

    let apicli = matrix::apis::client::APIClient::new(
        matrix::apis::configuration::Configuration::new()
    );

    let mut resq_body = LoginRequestBody {
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
