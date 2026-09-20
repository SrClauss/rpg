use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub struct GeminiClient {
    pub api_key: String,
    pub base_url: String,
    pub client: Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:generateContent".to_string(),
            client: Client::new(),
        }
    }

    pub async fn generate_narrative(&self, prompt: &str, system_instruction: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}?key={}", self.base_url, self.api_key);

        let body = json!({
            "system_instruction": {
                "parts": [{ "text": system_instruction }]
            },
            "contents": [{
                "parts": [{ "text": prompt }]
            }],
            "generationConfig": {
                "temperature": 0.7
            }
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(format!(
                "Erro na API Gemini (Status {}): {}",
                status, response_text
            ).into());
        }

        let res_json: Value = serde_json::from_str(&response_text)?;
        
        let content = res_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("");

        Ok(content.to_string())
    }
}
