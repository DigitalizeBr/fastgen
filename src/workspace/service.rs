use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;
use tera::{Tera, Context};

pub fn add_service(name: &str, repo: &str, templates_base_path_override: Option<&Path>) {
    let service_path = Path::new(repo).join("services").join(name);
    fs::create_dir_all(&service_path).unwrap();

    // Determine template path
    let template_glob_str;
    if let Some(override_path) = templates_base_path_override {
        // The override path should point to the directory containing the 'service' subdirectory
        template_glob_str = override_path.join("service/**/*").to_str().unwrap().to_string();
    } else {
        let exe_path = env::current_exe().unwrap();
        let base_path = exe_path
            .parent().unwrap() // target/debug or target/release
            .parent().unwrap() // target
            .parent().unwrap(); // project root
        template_glob_str = base_path.join("templates/service/**/*").to_str().unwrap().to_string();
    }

    let tera = Tera::new(&template_glob_str).expect("Erro ao carregar templates de serviço");
    let mut context = Context::new();
    context.insert("service_name", name);

    for file in ["main.py", "requirements.txt", "Dockerfile"] {
        let rendered = tera.render(file, &context).unwrap();
        fs::write(service_path.join(file), rendered).unwrap();
    }

    let py_path = Path::new(repo).join("pyproject.toml");
    let mut py = fs::read_to_string(&py_path).unwrap();
    let member = format!("\"services/{}\"", name);
    if !py.contains(&member) {
        py = py.replace("members = [", &format!("members = [\n    {},", member));
        fs::write(py_path, py).unwrap();
    }

    let compose_path = Path::new(repo).join("docker-compose.yml");
    let mut compose = fs::read_to_string(&compose_path).unwrap();
    if !compose.contains(&format!("  {}:", name)) {
        let port = 8000 + rand::random::<u16>() % 1000;
        compose.push_str(&format!(
            "  {}:\n    build: ./services/{}\n    ports:\n      - \"{}:{}\"\n",
            name, name, port, 8000
        ));
        fs::write(compose_path, compose).unwrap();

        let env_path = Path::new(repo).join(".env");
        let mut env = fs::read_to_string(&env_path).unwrap();
        env.push_str(&format!("{}_PORT={}\n", name.to_uppercase(), port));
        fs::write(env_path, env).unwrap();
    }

    let app_path = format!("services/{}", name);
    let status = Command::new("uv")
        .args(["init", &app_path, "--no-workspace", "--app"])
        .current_dir(repo)
        .status()
        .expect("Erro ao rodar 'uv init <path> --no-workspace --app'");
    
    if status.success() {
        println!("FastAPI adicionado com sucesso via uv!");
    } else {
        eprintln!("Erro ao adicionar FastAPI com uv.");
    }

    println!("Serviço '{}' adicionado ao workspace '{}'.", name, repo);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::*; // Imports add_service
    use std::fs::{self, File};
    use std::io::{self, Write}; // Added io for Result
    use tempfile::TempDir;
    use std::path::{Path, PathBuf};

    fn setup_test_environment() -> io::Result<(TempDir, PathBuf)> {
        let tmp_dir = TempDir::new()?;
        let root_path = tmp_dir.path().to_path_buf();

        // Create templates directory
        let templates_dir = root_path.join("templates").join("service");
        fs::create_dir_all(&templates_dir)?;

        // Create template files
        let mut main_py = File::create(templates_dir.join("main.py"))?;
        main_py.write_all(b"print(\"Hello from {{ service_name }}\")")?;

        let mut req_txt = File::create(templates_dir.join("requirements.txt"))?;
        req_txt.write_all(b"fastapi")?;

        let mut dockerfile = File::create(templates_dir.join("Dockerfile"))?;
        dockerfile.write_all(b"FROM python:3.9-slim\nWORKDIR /app\nCOPY requirements.txt .\nRUN pip install --no-cache-dir -r requirements.txt\nCOPY . .")?;

        // Create workspace files
        let mut pyproject_toml = File::create(root_path.join("pyproject.toml"))?;
        pyproject_toml.write_all(b"[tool.poetry]\nname = \"test-workspace\"\nversion = \"0.1.0\"\ndescription = \"\"\nauthors = [\"Your Name <you@example.com>\"]\n\n[tool.poetry.dependencies]\npython = \"^3.9\"\n\n[tool.poetry.group.dev.dependencies]\npytest = \"^7.0\"\n\n[tool.pytest.ini_options]\nminversion = \"6.0\"\naddopts = \"-ra -q\"\ntestpaths = [\"tests\"]\n\n[tool.ruff]\nselect = [\"E\", \"F\", \"I\"]\n\n[workspace.members]\nmembers = []")?;

        let mut docker_compose_yml = File::create(root_path.join("docker-compose.yml"))?;
        docker_compose_yml.write_all(b"version: '3.8'\nservices:\n")?;

        let mut env_file = File::create(root_path.join(".env"))?;
        env_file.write_all(b"GLOBAL_VAR=initial_value\n")?;

        Ok((tmp_dir, root_path))
    }

    #[test]
    fn test_add_service_creates_files_and_updates_configs() {
        let (_tmp_dir, root_path) = setup_test_environment().expect("Failed to set up test environment");
        let service_name = "my_test_service";
        let root_path_str = root_path.to_str().unwrap();

        add_service(service_name, root_path_str, Some(&root_path.join("templates")));

        // 1. Assert Service Directory and Files
        let service_dir = root_path.join("services").join(service_name);
        assert!(service_dir.exists() && service_dir.is_dir(), "Service directory should exist");

        let expected_files = [
            ("main.py", "Hello from my_test_service"),
            ("requirements.txt", "fastapi"),
            ("Dockerfile", "FROM python:3.9-slim"),
        ];

        for (file_name, expected_content_part) in expected_files.iter() {
            let file_path = service_dir.join(file_name);
            assert!(file_path.exists() && file_path.is_file(), "File {} should exist", file_name);
            let content = fs::read_to_string(&file_path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_name, e));
            assert!(content.contains(expected_content_part), "Content of {} did not match. Found: {}", file_name, content);
        }

        // 2. Assert pyproject.toml Update
        let pyproject_path = root_path.join("pyproject.toml");
        assert!(pyproject_path.exists() && pyproject_path.is_file(), "pyproject.toml should exist");
        let pyproject_content = fs::read_to_string(pyproject_path).unwrap();
        let expected_pyproject_member = format!("\"services/{}\"", service_name);
        assert!(pyproject_content.contains(&expected_pyproject_member), "pyproject.toml was not updated with service member. Content: {}", pyproject_content);

        // 3. Assert docker-compose.yml Update
        let compose_path = root_path.join("docker-compose.yml");
        assert!(compose_path.exists() && compose_path.is_file(), "docker-compose.yml should exist");
        let compose_content = fs::read_to_string(compose_path).unwrap();
        assert!(compose_content.contains(&format!("  {}:", service_name)), "docker-compose.yml does not define service {}. Content: {}", service_name, compose_content);
        assert!(compose_content.contains(&format!("./services/{}", service_name)), "docker-compose.yml does not link build context for {}. Content: {}", service_name, compose_content);
        assert!(compose_content.contains("ports:"), "docker-compose.yml does not contain 'ports:' section for service {}. Content: {}", service_name, compose_content);
        // Note: Exact port matching is tricky due to randomness. This checks for presence of the section.

        // 4. Assert .env Update
        let env_path = root_path.join(".env");
        assert!(env_path.exists() && env_path.is_file(), ".env file should exist");
        let env_content = fs::read_to_string(env_path).unwrap();
        let expected_env_var = format!("{}_PORT=", service_name.to_uppercase());
        assert!(env_content.contains(&expected_env_var), ".env file does not contain port variable for {}. Content: {}", service_name, env_content);
    }
}
