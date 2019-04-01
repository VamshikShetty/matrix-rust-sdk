
extern crate matrix;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::Future;
use hyper::client::HttpConnector;
use hyper::Client;
use matrix::apis::Error;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().expect("failed to init core");
    let handle = core.handle();

    let apicli = matrix::apis::client::APIClient::new(
        matrix::apis::configuration::Configuration::new(
            Client::configure()
                .connector(HttpConnector::new(4, &handle))
                .build(&handle),
        ),
    );

    let work = apicli.session_management_api().get_login_flows().and_then(|resp|{
            println!("{:?}", resp.flows);
            futures::future::ok(())
        });
    
    core.run(work).expect("failed to run core");
}
