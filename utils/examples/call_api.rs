extern crate reqwest;
extern crate utils;

use reqwest::header::{Authorization, Basic};
use std::env;

fn main() {
    let mut response = reqwest::Client::new()
        .post("https://icfpc-api.appspot.com/time.php")
        .header(Authorization(Basic {
            username: "unagi".to_owned(),
            password: Some(env::var("UNAGI_PASSWORD").unwrap().to_owned()),
        }))
        .form(&[("key", "value")])
        .send()
        .unwrap();
    let body = response.text().unwrap();
    println!("{}", body);
}
