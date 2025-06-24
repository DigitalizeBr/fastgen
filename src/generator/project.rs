use std::fs;
use std::path::Path;
use tera::{Tera, Context};
use crate::generator::blueprint::Blueprint;

pub fn generate_project(blueprint_path: &str) {
    let yaml = fs::read_to_string(blueprint_path).expect("Erro ao ler blueprint.yaml");
    let blueprint: Blueprint = serde_yaml::from_str(&yaml).expect("Erro ao parsear YAML");

    let base_dir = Path::new(&blueprint.service.name);
    for folder in &blueprint.structure.folders {
        fs::create_dir_all(base_dir.join(folder)).unwrap();
    }

    let tera = Tera::new("templates/base/**/*").unwrap();
    let mut context = Context::new();
    context.insert("project_name", &blueprint.service.name);

    for file in &blueprint.structure.files {
        let rendered = tera.render(file, &context).unwrap();
        fs::write(base_dir.join(file), rendered).unwrap();
    }

    println!("Projeto '{}' gerado com sucesso!", blueprint.service.name);
}

