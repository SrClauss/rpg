use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub struct GeminiClient {
    pub api_key: String,
    pub model: String,
    pub client: Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self::with_model(api_key, "gemini-3.5-flash-lite".to_string())
    }

    pub fn with_model(api_key: String, model: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(8))
            .build()
            .unwrap_or_else(|_| Client::new());
        Self {
            api_key,
            model,
            client,
        }
    }

    pub async fn generate_narrative(&self, prompt: &str, system_instruction: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

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
