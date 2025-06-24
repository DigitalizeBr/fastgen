use std::fs;
use std::path::Path;
use serde_yaml::Value;
use tiny_http::{Server, Response, Header};

pub fn start_dev_ui(repo: &str) {
    let address = "127.0.0.1:9000";
    let server = Server::http(address).expect("Failed to start server");
    println!("FastGen Dev UI running at http://{}/", address);

    for request in server.incoming_requests() {
        let html = generate_html(repo);
        let response = Response::from_string(html)
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
        let _ = request.respond(response);
    }
}

fn generate_html(repo: &str) -> String {
    let compose_path = Path::new(repo).join("docker-compose.yml");
    let mut services: Vec<String> = Vec::new();

    if let Ok(content) = fs::read_to_string(compose_path) {
        if let Ok(doc) = serde_yaml::from_str::<Value>(&content) {
            if let Some(map) = doc.get("services").and_then(|v| v.as_mapping()) {
                for (k, _) in map {
                    if let Some(name) = k.as_str() {
                        services.push(name.to_string());
                    }
                }
            }
        }
    }

    let mut html = String::from("<html><head><title>FastGen Dev UI</title></head><body>");
    html.push_str("<h1>FastGen Dev UI</h1>");
    html.push_str("<h2>Services</h2><ul>");
    for srv in services {
        html.push_str(&format!("<li>{}</li>", srv));
    }
    html.push_str("</ul></body></html>");
    html
}

