mod client;
mod question;

use std::error::Error;
pub use client::*;
pub use question::*;

pub struct Lib{
    client: Client,
}


impl Lib {
    pub fn new(api_url: &str, api_key: &str) -> Self {
        Lib {client: Client::new(api_url, api_key) }
    }

    pub fn ask(&self, prompt: String) -> Result<String, Box<dyn Error>> {
        dbg!(&prompt);
        let question = Question::new(prompt);

        question.req(&self.client.api_url, &self.client.api_key)
    }
}

