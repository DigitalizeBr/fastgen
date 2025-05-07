use std::fs;
use std::path::{Path, PathBuf};
use std::env;

use tera::{Tera, Context};
use crate::plugin::metadata::Plugin;
use crate::config::Config;

use reqwest::blocking::Client;
use serde::Deserialize;
use base64;

pub fn apply_plugin(name: &str, project: &str, config: &Config) {
    let plugin_path = format!("templates/plugins/{}/plugin.yaml", name);

    if !Path::new(&plugin_path).exists() {
        println!("Plugin '{}' não encontrado localmente. Baixando do GitHub...", name);
        download_plugin_from_github(name, config).expect("Erro ao baixar plugin remoto");
    }

    let yaml = fs::read_to_string(&plugin_path).expect("Erro ao ler plugin.yaml");
    let plugin: Plugin = serde_yaml::from_str(&yaml).expect("Erro ao parsear plugin");

    // 🔧 Configura o Tera para olhar especificamente na pasta do plugin
    let tera_pattern = format!("templates/plugins/{}/**/*", name);
    let tera = Tera::new(&tera_pattern).expect("Erro ao carregar templates do plugin");

    for target in plugin.targets {
        let path = Path::new(project).join(&target.path);

        if let Some(template_file) = target.template {
            let mut context = Context::new();
            context.insert("project_name", project);
            context.insert("service_name", name);

            // 🔧 Garante que o template seja referenciado corretamente
            let template_path = format!("templates/plugins/{}/{}", name, template_file);
            let rendered = tera
                .render(&template_file, &context)
                .unwrap_or_else(|e| panic!("Erro ao renderizar template '{}': {:?}", template_path, e));

            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, rendered).unwrap();
        } else if let Some(append_text) = target.append {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut content = fs::read_to_string(&path).unwrap_or_default();
            content.push_str(&append_text);
            fs::write(&path, content).unwrap();
        }
    }

    println!("Plugin '{}' aplicado ao projeto '{}'.", name, project);
}

#[derive(Deserialize)]
struct GitHubFile {
    name: String,
    content: Option<String>,
    download_url: Option<String>,
    path: String,
    encoding: Option<String>,
}

fn download_plugin_from_github(plugin_name: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/DigitalizeBr/fastgen/contents/templates/plugins/{}",
        plugin_name
    );

    let token = config.github_token.clone().unwrap_or_else(|| {
        panic!("Token do GitHub não encontrado. Adicione em config.yaml como 'github_token'")
    });

    let client = Client::builder()
        .user_agent("fastgen")
        .build()?;

    let response = client
        .get(&url)
        .bearer_auth(&token)
        .send()?
        .error_for_status()?;

    let files: Vec<GitHubFile> = response.json()?;
    let plugin_dir = PathBuf::from(format!("templates/plugins/{}", plugin_name));
    fs::create_dir_all(&plugin_dir)?;

    for file in files {
        let file_url = format!(
            "https://api.github.com/repos/DigitalizeBr/fastgen/contents/{}",
            file.path
        );

        let file_resp = client
            .get(&file_url)
            .bearer_auth(&token)
            .send()?
            .error_for_status()?;

        let file_data: GitHubFile = file_resp.json()?;

        if let (Some(encoded), Some(enc)) = (file_data.content, file_data.encoding) {
            if enc == "base64" {
                let decoded = base64::decode(encoded.replace('\n', ""))?;
                let path = plugin_dir.join(file_data.name);
                fs::write(path, decoded)?;
            }
        }
    }

    Ok(())
}
