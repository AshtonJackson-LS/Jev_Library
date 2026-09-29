use jev_lib::Lib;
use std::io;
use serde_json::{Value};

fn main() {
    // API and jev intialized
    let apikey = "";
    let apiurl = "https://api.typesafe.ai/v1/systemone";

    let jev = Lib::new(apiurl, apikey);

    // Input from console
    let mut input = String::new();

    println!("Please enter a prompt to see if its urgent:");
    io::stdin().read_line(&mut input).expect("Failed to read line");


    match jev.ask(input.trim().to_string()) {
        Ok(reply) => {
            let json: Value = serde_json::from_str(&reply).expect("reply wasn't JSON");
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Err(e) => println!("Request failed: {e}"),
    }

}