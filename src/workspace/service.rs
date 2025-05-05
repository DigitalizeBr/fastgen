use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;
use tera::{Tera, Context};

pub fn add_service(name: &str, repo: &str) {
    let service_path = Path::new(repo).join("services").join(name);
    fs::create_dir_all(&service_path).unwrap();

    // Corrigir o path dos templates
    let exe_path = env::current_exe().unwrap();
    let base_path = exe_path
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap();
    let template_glob = base_path.join("templates/service/**/*");
    let template_glob = template_glob.to_str().unwrap();

    let tera = Tera::new(template_glob).expect("Erro ao carregar templates de serviço");
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

    let app_path = format!("{}/services/{}", repo, name);
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
