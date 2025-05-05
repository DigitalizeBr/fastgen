use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Blueprint {
    pub service: Service,
    pub structure: Structure,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Structure {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}