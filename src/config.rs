use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub github_token: Option<String>,
    pub default_author: Option<String>,
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
