//check here: https://github.com/dermesser/yup-oauth2/blob/7e26db39ca4a17f0057c2b106bcd6fd654eb986c/examples/drive_example/src/main.rs
mod google_sheets_connector;

extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2;
extern crate google_sheets4 as sheets4;
extern crate dotenv;

use google_sheets_connector as gsc;
//use google_sheets4::ValueRange;
//use google_sheets4::{Result, Error};
//use google_sheets4::Sheets;
//use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
//    DefaultAuthenticatorDelegate, read_application_secret};
//use hyper::net::HttpsConnector;
//use hyper_native_tls::NativeTlsClient;


fn main(){
	let read_data = gsc::get_read(
		gsc::get_connection("TestSheet"),
		1,
        1,
        4,
        0
	);

    let mut test_data = gsc::gsc_reader(read_data);

	println!("{:#?}", &test_data);

	let write_data = gsc::get_write(
		gsc::get_connection("TestSheet"),
		1,
		9,
		test_data,
		Some(gsc::ValueOption::Raw));
	
	let mut test_write = gsc::gsc_writer(write_data);
	println!("Response: {:#?}", test_write);
}
