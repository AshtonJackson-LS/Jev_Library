use serde_json::json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Question {
    pub question: String,
}



impl Question {
    pub fn new(question: String) -> Question {
        Question { question }
    }
    // Make the request. This will later be moved to its owns script

    pub fn req(&self, api_url: &String, api_key: &String) -> Result<String, Box<dyn std::error::Error>> {
        //Build the body
        let body = json!({
  "state": &self.question,
  "model": "jev-latest",
  "questions": {
    "is_urgent": {
      "type": "noul",
      "instructions": "Does this convey urgency?"
    }
  }
});
        println!("{}", serde_json::to_string_pretty(&body).unwrap());

        let res = reqwest::blocking::Client::new()
            .post(api_url)
            .bearer_auth(&api_key)
            .json(&body)
            .send()?;
        Ok(res.text()?)
    }

}