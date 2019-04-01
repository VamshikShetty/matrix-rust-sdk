
extern crate matrix;

use matrix::apis::client::APIClient;
use matrix::apis::configuration::Configuration;

fn main() {

    let apicli = APIClient::new( Configuration::new());
    let res = apicli.session_management_api().get_login_flows(); 
    println!("{:?}", res);
}
