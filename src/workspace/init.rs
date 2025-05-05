use std::fs;
use std::path::Path;

pub fn create_workspace(name: &str) {
    let base = Path::new(name);
    fs::create_dir_all(base.join("services")).unwrap();
    fs::write(
        base.join("pyproject.toml"),
        "[tool.uv]\nworkspace = true\nmembers = []\n",
    ).unwrap();
    fs::write(
        base.join("docker-compose.yml"),
        "version: \"3.9\"
services:
",
    ).unwrap();
    fs::write(base.join(".env"), "# Variáveis de ambiente
").unwrap();
    println!("Monorepo '{}' criado com sucesso!", name);
}