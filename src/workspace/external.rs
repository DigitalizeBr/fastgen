use std::path::Path;

pub fn add_external_service(name: &str, repo: &str) {
    let template_path = format!("templates/external/{}.yml", name);
    let snippet = std::fs::read_to_string(&template_path)
        .unwrap_or_else(|_| panic!("Serviço externo '{}' não encontrado.", name));

    let compose_path = Path::new(repo).join("docker-compose.yml");
    let mut compose = std::fs::read_to_string(&compose_path).unwrap();

    if !compose.contains(&format!("  {}:", name)) {
        compose.push_str(&format!("\n{}", snippet));
        std::fs::write(&compose_path, compose).unwrap();
    }

    let env_path = Path::new(repo).join(".env");
    let mut env = std::fs::read_to_string(&env_path).unwrap();
    if !env.contains(&format!("{}_ENABLED=true", name.to_uppercase())) {
        env.push_str(&format!("{}_ENABLED=true\n", name.to_uppercase()));
        std::fs::write(&env_path, env).unwrap();
    }

    println!("Serviço externo '{}' adicionado ao docker-compose.", name);
}

