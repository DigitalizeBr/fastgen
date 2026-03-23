use crate::config::Config;
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::time::Duration;

pub trait LlmProvider {
    fn chat(&self, prompt: &str) -> Result<String, String>;
    fn generate_files(&self, prompt: &str) -> Result<Vec<GeneratedFile>, String>;
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(config: &Config) -> Result<Self, String> {
        let api_key = config.openai_api_key.clone().ok_or("openai_api_key is required")?;
        let model = config.llm_model.clone().unwrap_or_else(|| "gpt-4o-mini".to_string());
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { client, api_key, model })
    }
}

impl LlmProvider for OpenAiProvider {
    fn chat(&self, prompt: &str) -> Result<String, String> {
        let body = json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
        });

        let res: Value = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        res["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Failed to parse content".to_string())
    }

    fn generate_files(&self, prompt: &str) -> Result<Vec<GeneratedFile>, String> {
        let function_schema = json!({
            "name": "create_files",
            "description": "Create a set of files in the project structure",
            "parameters": {
                "type": "object",
                "properties": {
                    "files": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "path": {"type": "string", "description": "The path of the file to create, relative to the project root."},
                                "content": {"type": "string", "description": "The exact full content of the file."}
                            },
                            "required": ["path", "content"]
                        }
                    }
                },
                "required": ["files"]
            }
        });

        let body = json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "functions": [function_schema],
            "function_call": {"name": "create_files"}
        });

        let res: Value = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        let args_str = res["choices"][0]["message"]["function_call"]["arguments"]
            .as_str()
            .ok_or_else(|| "Failed to get function arguments".to_string())?;

        let parsed_args: Value = serde_json::from_str(args_str).map_err(|e| e.to_string())?;

        let files: Vec<GeneratedFile> = serde_json::from_value(parsed_args["files"].clone())
            .map_err(|e| e.to_string())?;

        Ok(files)
    }
}


pub struct GeminiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(config: &Config) -> Result<Self, String> {
        let api_key = config.gemini_api_key.clone().ok_or("gemini_api_key is required")?;
        let model = config.llm_model.clone().unwrap_or_else(|| "gemini-1.5-pro".to_string());
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { client, api_key, model })
    }
}

impl LlmProvider for GeminiProvider {
    fn chat(&self, prompt: &str) -> Result<String, String> {
        let body = json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }]
        });

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", self.model, self.api_key);

        let res: Value = self.client.post(&url)
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        res["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Failed to parse content".to_string())
    }

    fn generate_files(&self, prompt: &str) -> Result<Vec<GeneratedFile>, String> {
        let function_declaration = json!({
            "name": "create_files",
            "description": "Create a set of files in the project structure",
            "parameters": {
                "type": "object",
                "properties": {
                    "files": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "path": {"type": "string", "description": "The path of the file to create, relative to the project root."},
                                "content": {"type": "string", "description": "The exact full content of the file."}
                            },
                            "required": ["path", "content"]
                        }
                    }
                },
                "required": ["files"]
            }
        });

        let body = json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }],
            "tools": [{"function_declarations": [function_declaration]}],
            "tool_config": {
                "function_calling_config": {
                    "mode": "ANY",
                    "allowed_function_names": ["create_files"]
                }
            }
        });

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", self.model, self.api_key);

        let res: Value = self.client.post(&url)
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        let args = res["candidates"][0]["content"]["parts"][0]["functionCall"]["args"].clone();

        let files: Vec<GeneratedFile> = serde_json::from_value(args["files"].clone())
            .map_err(|e| format!("Failed to parse files: {}", e))?;

        Ok(files)
    }
}


pub struct OllamaProvider {
    client: Client,
    url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(config: &Config) -> Result<Self, String> {
        let url = config.ollama_url.clone().unwrap_or_else(|| "http://localhost:11434".to_string());
        let model = config.llm_model.clone().unwrap_or_else(|| "llama3".to_string());
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { client, url, model })
    }
}

impl LlmProvider for OllamaProvider {
    fn chat(&self, prompt: &str) -> Result<String, String> {
        let body = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false
        });

        let endpoint = format!("{}/api/generate", self.url);

        let res: Value = self.client.post(&endpoint)
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        res["response"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Failed to parse response".to_string())
    }

    fn generate_files(&self, prompt: &str) -> Result<Vec<GeneratedFile>, String> {
        // Ollama native function calling is hit or miss depending on the model,
        // we'll enforce a strict JSON output instruction instead.
        let json_prompt = format!(
            "{}\n\nIMPORTANT: You must return the output as a strict JSON format exactly like this, without any markdown formatting or extra text outside the JSON block. Do not wrap it in ```json blocks:\n[{{\"path\": \"app/main.py\", \"content\": \"print('hello')\"}}]",
            prompt
        );

        let body = json!({
            "model": self.model,
            "prompt": json_prompt,
            "stream": false,
            "format": "json" // Tell Ollama to output valid JSON
        });

        let endpoint = format!("{}/api/generate", self.url);

        let res: Value = self.client.post(&endpoint)
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        let text = res["response"]
            .as_str()
            .ok_or_else(|| "Failed to parse response".to_string())?;

        // Some models still wrap in markdown despite format: json
        let clean_text = text.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

        // Sometimes the model wraps the array in an object
        let parsed: Value = serde_json::from_str(clean_text).map_err(|e| format!("Invalid JSON from Ollama: {}", e))?;

        let files_val = if parsed.is_array() {
            parsed
        } else if parsed.get("files").is_some() {
            parsed["files"].clone()
        } else {
            return Err("Unexpected JSON structure from Ollama".into());
        };

        let files: Vec<GeneratedFile> = serde_json::from_value(files_val)
            .map_err(|e| e.to_string())?;

        Ok(files)
    }
}

pub fn get_provider(config: &Config) -> Result<Box<dyn LlmProvider>, String> {
    let provider_name = config.llm_provider.clone().unwrap_or_else(|| "ollama".to_string());

    match provider_name.to_lowercase().as_str() {
        "openai" => Ok(Box::new(OpenAiProvider::new(config)?)),
        "gemini" => Ok(Box::new(GeminiProvider::new(config)?)),
        "ollama" => Ok(Box::new(OllamaProvider::new(config)?)),
        _ => Err(format!("Unknown LLM provider: {}", provider_name)),
    }
}
