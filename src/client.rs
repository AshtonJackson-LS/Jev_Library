pub struct Client {
    pub api_url: String,
    pub api_key: String,
}

impl Client {
    pub fn new(api_url: &str, api_key: &str) -> Self {
        Client {api_url: api_url.to_owned(), api_key: api_key.to_owned() }
    }

}