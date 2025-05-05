use std::fs;
use std::path::Path;
use tera::{Tera, Context};
use crate::plugin::metadata::Plugin;

pub fn apply_plugin(name: &str, project: &str) {
    let plugin_path = format!("templates/plugins/{}/plugin.yaml", name);
    let yaml = fs::read_to_string(plugin_path).expect("Erro ao ler plugin.yaml");
    let plugin: Plugin = serde_yaml::from_str(&yaml).expect("Erro ao parsear plugin");

    for target in plugin.targets {
        let path = Path::new(project).join(&target.path);
        if let Some(template_file) = target.template {
            let tera = Tera::new("templates/**/*").unwrap();
            let mut context = Context::new();
            context.insert("project_name", project);
            let rendered = tera.render(&template_file, &context).unwrap();
            fs::write(&path, rendered).unwrap();
        } else if let Some(append_text) = target.append {
            let mut content = fs::read_to_string(&path).unwrap_or_default();
            content.push_str(&append_text);
            fs::write(&path, content).unwrap();
        }
    }

    println!("Plugin '{}' aplicado ao projeto '{}'.", name, project);
}
