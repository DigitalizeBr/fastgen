use std::fmt::Write as _;
use std::path::Path;
use std::error::Error;

pub fn add_external_service(name: &str, repo: &str) -> Result<(), Box<dyn Error>> {
    let template_path = format!("templates/external/{}.yml", name);
    let snippet = std::fs::read_to_string(&template_path)
        .map_err(|_| format!("Serviço externo '{}' não encontrado.", name))?;

    let compose_path = Path::new(repo).join("docker-compose.yml");
    let mut compose = std::fs::read_to_string(&compose_path)
        .map_err(|e| format!("Erro ao ler docker-compose.yml em '{}': {}", repo, e))?;

    if !compose.contains(&format!("  {}:", name)) {
        let _ = write!(compose, "\n{}", snippet);
        std::fs::write(&compose_path, compose)
            .map_err(|e| format!("Erro ao salvar docker-compose.yml: {}", e))?;
    }

    let env_path = Path::new(repo).join(".env");
    let mut env = std::fs::read_to_string(&env_path)
        .map_err(|e| format!("Erro ao ler .env em '{}': {}", repo, e))?;

    if !env.contains(&format!("{}_ENABLED=true", name.to_uppercase())) {
        let _ = write!(env, "{}_ENABLED=true\n", name.to_uppercase());
        std::fs::write(&env_path, env)
            .map_err(|e| format!("Erro ao salvar .env: {}", e))?;
    }

    println!("Serviço externo '{}' adicionado ao docker-compose.", name);
    Ok(())
}
