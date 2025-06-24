use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub targets: Vec<Target>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pub path: String,
    pub template: Option<String>,
    pub append: Option<String>,
}
