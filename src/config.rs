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

impl Default for Config {
    fn default() -> Self {
        Self {
            github_token: None,
            default_author: None,
            llm_provider: None,
            llm_model: None,
            openai_api_key: None,
            gemini_api_key: None,
            ollama_url: Some("http://localhost:11434".to_string()),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let current_dir_path = std::path::PathBuf::from("config.yaml");
        let exe_dir_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from(""))
            .parent()
            .map(|p| p.join("config.yaml"))
            .unwrap_or_else(|| std::path::PathBuf::from("config.yaml"));

        if current_dir_path.exists() {
            Self::load_from_path(&current_dir_path)
        } else if exe_dir_path.exists() {
            Self::load_from_path(&exe_dir_path)
        } else {
            // Default behavior: create in current directory
            Self::load_from_path(&current_dir_path)
        }
    }

    pub fn load_from_path(path: &std::path::Path) -> Self {
        if !path.exists() {
            println!("Aviso: Arquivo '{}' não encontrado. Um arquivo padrão foi criado na raiz do projeto. Por favor, edite-o conforme necessário.", path.display());
            let default_yaml = r#"# Arquivo de configuracao gerado automaticamente
# github_token: "seu_token_aqui"
# default_author: "Seu Nome"

# Configuracoes de Inteligencia Artificial para 'ai-generate'
# llm_provider: "openai"           # "openai", "gemini", ou "ollama"
# llm_model: "gpt-4o"              # Ex: "gpt-4o", "gemini-1.5-pro", ou "llama3"
# openai_api_key: "sk-..."
# gemini_api_key: "AIza..."
ollama_url: "http://localhost:11434"
"#;
            if let Err(e) = fs::write(path, default_yaml) {
                eprintln!("Erro ao criar {} padrao: {}", path.display(), e);
            }
            return Config::default();
        }

        let content = fs::read_to_string(path)
            .expect("Erro ao ler config.yaml");

        println!("DEBUG config.yaml:\n{}", content); 

        serde_yaml::from_str(&content)
            .expect("Erro ao parsear config.yaml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn get_temp_path(test_name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("fastgen_test_{}.yaml", test_name));
        path
    }

    #[test]
    fn test_load_creates_default_if_missing() {
        let path = get_temp_path("creates_default");

        // Ensure test file does not exist initially
        if path.exists() {
            fs::remove_file(&path).unwrap();
        }

        let config = Config::load_from_path(&path);

        // Check if file was created
        assert!(path.exists(), "config file should be created");

        // Verify config matches default
        assert!(config.github_token.is_none());
        assert_eq!(config.ollama_url, Some("http://localhost:11434".to_string()));

        // Cleanup
        let _ = fs::remove_file(&path);
    }

    #[test]
    #[should_panic(expected = "Erro ao parsear config.yaml")]
    fn test_load_panics_on_invalid_yaml() {
        let path = get_temp_path("invalid_yaml");

        // Create an invalid yaml file
        fs::write(&path, "invalid: yaml: format: :").unwrap();

        struct Cleanup(PathBuf);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = fs::remove_file(&self.0);
            }
        }
        let _cleanup = Cleanup(path.clone());

        // This should panic
        Config::load_from_path(&path);
    }
}
