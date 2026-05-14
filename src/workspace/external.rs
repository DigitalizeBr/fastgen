use std::path::Path;
use std::error::Error;
use std::fs;

pub fn add_external_service(name: &str, repo: &str) -> Result<(), Box<dyn Error>> {
    let template_path = format!("templates/external/{}.yml", name);
    let snippet = fs::read_to_string(&template_path)
        .map_err(|_| format!("Serviço externo '{}' não encontrado em {}.", name, template_path))?;

    let compose_path = Path::new(repo).join("docker-compose.yml");
    let mut compose = fs::read_to_string(&compose_path)?;

    if !compose.contains(&format!("  {}:", name)) {
        compose.push_str(&format!("\n{}", snippet));
        fs::write(&compose_path, compose)?;
    }

    let env_path = Path::new(repo).join(".env");
    let mut env = fs::read_to_string(&env_path)?;
    if !env.contains(&format!("{}_ENABLED=true", name.to_uppercase())) {
        env.push_str(&format!("{}_ENABLED=true\n", name.to_uppercase()));
        fs::write(&env_path, env)?;
    }

    println!("Serviço externo '{}' adicionado ao docker-compose.", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn setup_test_env(test_name: &str) -> (PathBuf, PathBuf) {
        let mut repo_path = std::env::temp_dir();
        repo_path.push(format!("fastgen_test_repo_{}", test_name));
        if repo_path.exists() {
            fs::remove_dir_all(&repo_path).unwrap();
        }
        fs::create_dir_all(&repo_path).unwrap();

        let compose_path = repo_path.join("docker-compose.yml");
        fs::write(&compose_path, "services:\n").unwrap();

        let env_path = repo_path.join(".env");
        fs::write(&env_path, "").unwrap();

        // Create a mock template
        let mut template_dir = std::env::current_dir().unwrap();
        template_dir.push("templates/external");
        fs::create_dir_all(&template_dir).unwrap();
        let template_file = template_dir.join(format!("{}.yml", test_name));
        fs::write(&template_file, format!("  {}:\n    image: {}:latest", test_name, test_name)).unwrap();

        (repo_path, template_file)
    }

    #[test]
    fn test_add_external_service_success() {
        let test_name = "test_service";
        let (repo_path, template_file) = setup_test_env(test_name);

        let result = add_external_service(test_name, repo_path.to_str().unwrap());
        assert!(result.is_ok());

        let compose_content = fs::read_to_string(repo_path.join("docker-compose.yml")).unwrap();
        assert!(compose_content.contains("  test_service:"));
        assert!(compose_content.contains("    image: test_service:latest"));

        let env_content = fs::read_to_string(repo_path.join(".env")).unwrap();
        assert!(env_content.contains("TEST_SERVICE_ENABLED=true"));

        // Cleanup
        fs::remove_dir_all(&repo_path).unwrap();
        fs::remove_file(&template_file).unwrap();
    }

    #[test]
    fn test_add_external_service_already_exists() {
        let test_name = "existing_service";
        let (repo_path, template_file) = setup_test_env(test_name);

        // Add it once
        add_external_service(test_name, repo_path.to_str().unwrap()).unwrap();
        let compose_before = fs::read_to_string(repo_path.join("docker-compose.yml")).unwrap();

        // Add it again
        let result = add_external_service(test_name, repo_path.to_str().unwrap());
        assert!(result.is_ok());

        let compose_after = fs::read_to_string(repo_path.join("docker-compose.yml")).unwrap();
        assert_eq!(compose_before, compose_after);

        // Cleanup
        fs::remove_dir_all(&repo_path).unwrap();
        fs::remove_file(&template_file).unwrap();
    }

    #[test]
    fn test_add_external_service_template_missing() {
        let repo_path = std::env::temp_dir().join("fastgen_test_repo_missing");
        fs::create_dir_all(&repo_path).unwrap();
        fs::write(repo_path.join("docker-compose.yml"), "services:").unwrap();
        fs::write(repo_path.join(".env"), "").unwrap();

        let result = add_external_service("non_existent", repo_path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("não encontrado"));

        // Cleanup
        fs::remove_dir_all(&repo_path).unwrap();
    }
}
