use std::fs;
use std::path::Path;

pub fn create_workspace(name: &str) {
    let base = Path::new(name);
    fs::create_dir_all(base.join("services")).unwrap();

    fs::write(
        base.join("docker-compose.yml"),
        "version: \"3.9\"\nservices:\n",
    ).unwrap();

    fs::write(base.join(".env"), "# Variáveis de ambiente\n").unwrap();

    println!("Monorepo '{}' criado com sucesso!", name);
}
