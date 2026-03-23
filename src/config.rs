use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub github_token: Option<String>,
    pub default_author: Option<String>,

    // AI Configurations
    pub llm_provider: Option<String>, // "openai", "gemini", "ollama"
    pub llm_model: Option<String>,    // e.g., "gpt-4o", "gemini-1.5-pro", "llama3"
    pub openai_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
    pub ollama_url: Option<String>,   // default: "http://localhost:11434"
}

impl Config {
    pub fn load() -> Self {
        let content = fs::read_to_string("config.yaml")
            .expect("Erro ao ler config.yaml");

        println!("DEBUG config.yaml:\n{}", content); 

        serde_yaml::from_str(&content)
            .expect("Erro ao parsear config.yaml")
    }
}
