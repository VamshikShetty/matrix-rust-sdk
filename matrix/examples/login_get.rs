
extern crate matrix;
// extern crate futures;
// extern crate hyper;
// extern crate tokio_core;

// use futures::Future;
// use hyper::client::HttpConnector;
// use hyper::Client;
// use tokio_core::reactor::Core;

use matrix::models::Model200LoginGet;

fn main() {
    // let mut core = Core::new().expect("failed to init core");
    // let handle = core.handle();

    let apicli = matrix::apis::client::APIClient::new(
        matrix::apis::configuration::Configuration::new(
        )
    );

    let res = apicli.session_management_api().get_login_flows(); 
    println!("{:?}", res);
}
