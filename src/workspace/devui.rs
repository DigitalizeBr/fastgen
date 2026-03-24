use std::fs;
use std::path::{Path, PathBuf};
use serde_yaml::Value as YamlValue;
use serde_json::{json, Value as JsonValue};
use tiny_http::{Server, Response, Header, Method};
use crate::config::Config;
use crate::ai::providers::get_provider;
use crate::ai::generator::{generate_plan, execute_plan, validate_code};
use std::io::Read;
use std::process::Command;

pub fn start_dev_ui(repo: &str, ai_path: Option<&str>, config: &Config) {
    let address = "127.0.0.1:9000";

    // Auto-start docker-compose if exists
    let compose_path = Path::new(repo).join("docker-compose.yml");
    if compose_path.exists() {
        println!("Found docker-compose.yml. Attempting to start services...");
        let status = Command::new(get_docker_compose_cmd())
            .arg("compose")
            .arg("up")
            .arg("-d")
            .current_dir(repo)
            .status();

        match status {
            Ok(s) if s.success() => println!("Services started successfully."),
            Ok(s) => println!("Failed to start some services. Exit code: {}", s),
            Err(e) => println!("Error trying to start services: {}", e),
        }
    } else {
        println!("No docker-compose.yml found in the workspace.");
    }

    let server = Server::http(address).expect("Failed to start server");
    println!("FastGen Dev UI running at http://{}/", address);
    if let Some(path) = ai_path {
        println!("AI Generator enabled pointing to manifests at: {}", path);
    }

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().clone();

        if url.starts_with("/api/") {
            let res_json = match (method.clone(), url.as_str()) {
                (Method::Get, "/api/services") => {
                    handle_get_services(ai_path)
                },
                (Method::Get, "/api/compose/status") => {
                    handle_get_compose_status(repo)
                },
                (Method::Post, "/api/compose/action") => {
                    handle_post_compose_action(&mut request, repo)
                },
                (Method::Get, u) if u.starts_with("/api/compose/logs/") => {
                    let service_name = u.trim_start_matches("/api/compose/logs/");
                    handle_get_compose_logs(repo, service_name)
                },
                (Method::Post, "/api/plan") => {
                    handle_post_plan(&mut request, config, ai_path)
                },
                (Method::Post, "/api/execute") => {
                    handle_post_execute(&mut request, config, ai_path)
                },
                _ => {
                    json!({"error": "Not Found"}).to_string()
                }
            };

            let response = Response::from_string(res_json)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            let _ = request.respond(response);
            continue;
        }

        // Default to HTML UI
        let html = generate_html(repo, ai_path);
        let response = Response::from_string(html)
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
        let _ = request.respond(response);
    }
}

fn get_docker_compose_cmd() -> &'static str {
    // A simple helper to prefer `docker compose` over `docker-compose` if needed,
    // but for now we will rely on standard `docker compose` which is the modern standard.
    // If it fails, users can alias it or we can do a fallback, but `docker` is standard.
    "docker"
}

fn handle_get_compose_status(repo: &str) -> String {
    // Run `docker compose ps --format json`
    let output = Command::new(get_docker_compose_cmd())
        .arg("compose")
        .arg("ps")
        .arg("--format")
        .arg("json")
        .current_dir(repo)
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout_str = String::from_utf8_lossy(&out.stdout);

                // parse the JSON lines or JSON array from `docker compose ps`
                let mut containers = Vec::new();
                for line in stdout_str.lines() {
                    if let Ok(val) = serde_json::from_str::<JsonValue>(line) {
                        containers.push(val);
                    }
                }

                // If the output is a single JSON array rather than JSON lines
                if containers.is_empty() && stdout_str.trim().starts_with('[') {
                    if let Ok(arr) = serde_json::from_str::<Vec<JsonValue>>(&stdout_str) {
                        containers = arr;
                    }
                }

                json!({"status": "success", "containers": containers}).to_string()
            } else {
                let err = String::from_utf8_lossy(&out.stderr);
                json!({"error": format!("Command failed: {}", err)}).to_string()
            }
        },
        Err(e) => json!({"error": format!("Failed to execute docker compose: {}", e)}).to_string(),
    }
}

fn handle_post_compose_action(request: &mut tiny_http::Request, repo: &str) -> String {
    let mut content = String::new();
    if request.as_reader().read_to_string(&mut content).is_err() {
        return json!({"error": "Failed to read body"}).to_string();
    }

    let parsed: Result<JsonValue, _> = serde_json::from_str(&content);
    let (action, service) = match parsed {
        Ok(v) => (
            v["action"].as_str().unwrap_or("").to_string(),
            v["service"].as_str().unwrap_or("").to_string(),
        ),
        Err(_) => return json!({"error": "Invalid JSON"}).to_string(),
    };

    if !["start", "stop", "restart", "up"].contains(&action.as_str()) {
        return json!({"error": "Invalid action"}).to_string();
    }

    let mut cmd = Command::new(get_docker_compose_cmd());
    cmd.arg("compose").arg(&action).current_dir(repo);

    if !service.is_empty() && action != "up" {
        cmd.arg(&service);
    } else if action == "up" && !service.is_empty() {
        cmd.arg("-d").arg(&service);
    } else if action == "up" {
        cmd.arg("-d");
    }

    match cmd.output() {
        Ok(out) => {
            if out.status.success() {
                json!({"status": "success", "action": action, "service": service}).to_string()
            } else {
                let err = String::from_utf8_lossy(&out.stderr);
                json!({"error": format!("Action failed: {}", err)}).to_string()
            }
        },
        Err(e) => json!({"error": format!("Execution failed: {}", e)}).to_string(),
    }
}

fn handle_get_compose_logs(repo: &str, service: &str) -> String {
    let output = Command::new(get_docker_compose_cmd())
        .arg("compose")
        .arg("logs")
        .arg("--tail")
        .arg("100")
        .arg(service)
        .current_dir(repo)
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout_str = String::from_utf8_lossy(&out.stdout);
                json!({"status": "success", "logs": stdout_str}).to_string()
            } else {
                let err = String::from_utf8_lossy(&out.stderr);
                json!({"error": format!("Failed to fetch logs: {}", err)}).to_string()
            }
        },
        Err(e) => json!({"error": format!("Execution failed: {}", e)}).to_string(),
    }
}

fn handle_get_services(ai_path: Option<&str>) -> String {
    let mut services = Vec::new();
    if let Some(path) = ai_path {
        let base_path = Path::new(path);
        if base_path.exists() && base_path.is_dir() {
            if let Ok(entries) = fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                        if name != "validation" {
                            services.push(name);
                        }
                    }
                }
            }
        }
    }
    json!({ "services": services }).to_string()
}

fn handle_post_plan(request: &mut tiny_http::Request, config: &Config, ai_path: Option<&str>) -> String {
    let mut content = String::new();
    if request.as_reader().read_to_string(&mut content).is_err() {
        return json!({"error": "Failed to read body"}).to_string();
    }

    let parsed: Result<JsonValue, _> = serde_json::from_str(&content);
    let service_name = match &parsed {
        Ok(v) => v["service"].as_str().unwrap_or("").to_string(),
        Err(_) => return json!({"error": "Invalid JSON"}).to_string(),
    };

    if service_name.is_empty() {
        return json!({"error": "Service name required"}).to_string();
    }

    let provider = match get_provider(config) {
        Ok(p) => p,
        Err(e) => return json!({"error": e}).to_string(),
    };

    let Some(path) = ai_path else {
        return json!({"error": "AI Path not configured"}).to_string();
    };

    let service_dir = Path::new(path).join(&service_name);
    let mut manifest_content = String::new();

    if let Ok(entries) = fs::read_dir(&service_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() && p.extension().map(|e| e == "md" || e == "yml" || e == "yaml").unwrap_or(false) {
                if let Ok(c) = fs::read_to_string(&p) {
                    manifest_content = c;
                    break;
                }
            }
        }
    }

    if manifest_content.is_empty() {
        return json!({"error": "No manifest found"}).to_string();
    }

    // Scan for existing code
    let mut existing_code = String::new();
    fn read_existing_code(dir: &Path, acc: &mut String) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();

                // Skip hidden files/directories (like .git, .env) and common build/dependency folders
                if name.starts_with('.') || name == "target" || name == "node_modules" || name == "build" || name == "dist" || name == "venv" {
                    continue;
                }

                if path.is_dir() {
                    read_existing_code(&path, acc);
                } else if path.is_file() {
                    let ext = path.extension().unwrap_or_default().to_string_lossy();
                    if ext != "md" && ext != "yml" && ext != "yaml" {
                        if let Ok(content) = fs::read_to_string(&path) {
                            acc.push_str(&format!("\n\n--- Existing File: {:?} ---\n\n{}", path, content));
                        }
                    }
                }
            }
        }
    }

    read_existing_code(&service_dir, &mut existing_code);

    // Extract feedback and previous plan if they exist (for interactive generation from UI in the future)
    let feedback = match &parsed {
        Ok(v) => v.get("feedback").and_then(|f| f.as_str()).map(|s| s.to_string()),
        Err(_) => None,
    };
    let previous_plan = match &parsed {
        Ok(v) => v.get("previous_plan").and_then(|p| p.as_str()).map(|s| s.to_string()),
        Err(_) => None,
    };

    match generate_plan(&provider, &manifest_content, &service_name, &existing_code, feedback.as_deref(), previous_plan.as_deref()) {
        Ok(plan) => json!({"plan": plan, "manifest": manifest_content, "existing_code": existing_code}).to_string(),
        Err(e) => json!({"error": format!("Plan generation failed: {}", e)}).to_string(),
    }
}

fn handle_post_execute(request: &mut tiny_http::Request, config: &Config, ai_path: Option<&str>) -> String {
    let mut content = String::new();
    if request.as_reader().read_to_string(&mut content).is_err() {
        return json!({"error": "Failed to read body"}).to_string();
    }

    let parsed: Result<JsonValue, _> = serde_json::from_str(&content);
    let (service_name, plan, manifest, existing_code) = match parsed {
        Ok(v) => (
            v["service"].as_str().unwrap_or("").to_string(),
            v["plan"].as_str().unwrap_or("").to_string(),
            v["manifest"].as_str().unwrap_or("").to_string(),
            v["existing_code"].as_str().unwrap_or("").to_string(),
        ),
        Err(_) => return json!({"error": "Invalid JSON"}).to_string(),
    };

    if service_name.is_empty() || plan.is_empty() || manifest.is_empty() {
        return json!({"error": "Service, plan, and manifest required"}).to_string();
    }

    let provider = match get_provider(config) {
        Ok(p) => p,
        Err(e) => return json!({"error": e}).to_string(),
    };

    let Some(path) = ai_path else {
        return json!({"error": "AI Path not configured"}).to_string();
    };

    let service_dir = Path::new(path).join(&service_name);

    if let Err(e) = execute_plan(&provider, &manifest, &service_name, &plan, &existing_code, &service_dir) {
        return json!({"error": format!("Execution failed: {}", e)}).to_string();
    }

    // Optional Validation
    let mut validation_report = String::new();
    let val_dir = Path::new(path).join("validation");
    if val_dir.exists() && val_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&val_dir) {
            for entry in entries.flatten() {
                let vp = entry.path();
                if vp.is_file() && vp.extension().map(|e| e == "md" || e == "yml" || e == "yaml").unwrap_or(false) {
                    if let Ok(rules) = fs::read_to_string(&vp) {
                        match validate_code(&provider, &service_dir, &rules) {
                            Ok(report) => validation_report = report,
                            Err(e) => validation_report = format!("Validation failed to run: {}", e),
                        }
                        break;
                    }
                }
            }
        }
    }

    json!({"status": "success", "validation": validation_report}).to_string()
}

fn generate_html(repo: &str, ai_path: Option<&str>) -> String {
    let compose_path = Path::new(repo).join("docker-compose.yml");
    let mut compose_services: Vec<String> = Vec::new();

    if let Ok(content) = fs::read_to_string(compose_path) {
        if let Ok(doc) = serde_yaml::from_str::<YamlValue>(&content) {
            if let Some(map) = doc.get("services").and_then(|v| v.as_mapping()) {
                for (k, _) in map {
                    if let Some(name) = k.as_str() {
                        compose_services.push(name.to_string());
                    }
                }
            }
        }
    }

    let compose_list_html = compose_services.iter()
        .map(|srv| format!(r#"
            <li class="p-4 bg-gray-50 border border-gray-200 rounded-md flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div>
                    <span class="font-bold text-gray-800 text-lg">{}</span>
                    <span id="status-{}" class="ml-2 px-2 py-1 text-xs font-semibold rounded bg-gray-200 text-gray-700">Checking...</span>
                </div>
                <div class="flex flex-wrap gap-2">
                    <button onclick="composeAction('start', '{}')" class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 text-sm">Start</button>
                    <button onclick="composeAction('stop', '{}')" class="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700 text-sm">Stop</button>
                    <button onclick="composeAction('restart', '{}')" class="px-3 py-1 bg-yellow-500 text-white rounded hover:bg-yellow-600 text-sm">Restart</button>
                    <button onclick="viewLogs('{}')" class="px-3 py-1 bg-gray-600 text-white rounded hover:bg-gray-700 text-sm">Logs</button>
                </div>
            </li>
        "#, srv, srv, srv, srv, srv, srv))
        .collect::<Vec<String>>()
        .join("");

    let ai_tab_disabled = if ai_path.is_some() { "false" } else { "true" };

    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FastGen Dev UI</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <!-- Marked.js for markdown rendering -->
    <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
</head>
<body class="bg-gray-100 min-h-screen">
    <div class="max-w-6xl mx-auto p-6">
        <header class="mb-8">
            <h1 class="text-4xl font-bold text-gray-800 flex items-center gap-2">
                ⚡ FastGen Dev UI
            </h1>
            <p class="text-gray-500 mt-2">Workspace: <span class="font-mono text-gray-700 bg-gray-200 px-2 py-1 rounded">{}</span></p>
        </header>

        <div class="mb-4 border-b border-gray-200">
            <ul class="flex flex-wrap -mb-px text-sm font-medium text-center" id="tabs">
                <li class="mr-2">
                    <button class="inline-block p-4 border-b-2 border-blue-600 rounded-t-lg text-blue-600" id="tab-compose" onclick="switchTab('compose')">Docker Compose</button>
                </li>
                <li class="mr-2">
                    <button class="inline-block p-4 border-b-2 border-transparent hover:text-gray-600 hover:border-gray-300 rounded-t-lg text-gray-500" id="tab-ai" onclick="switchTab('ai')">🤖 AI Generator</button>
                </li>
            </ul>
        </div>

        <div id="content-compose" class="tab-content block">
            <div class="flex flex-col gap-6">
                <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-2xl font-semibold">Compose Services</h2>
                        <div class="flex gap-2">
                            <button onclick="composeAction('up', '')" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 font-medium shadow-sm">Start All (Up)</button>
                            <button onclick="composeAction('stop', '')" class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 font-medium shadow-sm">Stop All</button>
                        </div>
                    </div>

                    <ul class="space-y-3" id="compose-services-list">
                        {}
                    </ul>
                    {}
                </div>

                <div id="logs-panel" class="hidden bg-gray-900 p-4 rounded-lg shadow-sm border border-gray-700 text-gray-100 font-mono text-sm h-96 flex flex-col">
                    <div class="flex justify-between items-center mb-2 border-b border-gray-700 pb-2">
                        <h3 class="font-bold text-gray-300">Logs: <span id="log-service-name" class="text-green-400"></span></h3>
                        <button onclick="closeLogs()" class="text-gray-400 hover:text-white px-2 py-1 rounded">✖ Close</button>
                    </div>
                    <div id="logs-content" class="overflow-auto flex-1 whitespace-pre-wrap">Loading...</div>
                </div>
            </div>
        </div>

        <div id="content-ai" class="tab-content hidden">
            <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200" id="ai-container">
                <h2 class="text-2xl font-semibold mb-4">AI Service Generator</h2>

                <div id="ai-disabled-msg" class="hidden text-red-500 bg-red-50 p-4 rounded-md border border-red-200">
                    AI Generator is not enabled. Start Dev UI with <code class="bg-white px-1 rounded text-red-700">--ai-path &lt;manifest-folder&gt;</code> to use this feature.
                </div>

                <div id="ai-app" class="hidden flex gap-6">
                    <!-- Sidebar: Services List -->
                    <div class="w-1/3 border-r border-gray-200 pr-6">
                        <h3 class="font-medium text-gray-700 mb-3">Manifests Found</h3>
                        <ul id="ai-services-list" class="space-y-2">
                            <li class="text-gray-400 italic text-sm">Loading...</li>
                        </ul>
                    </div>

                    <!-- Main Area: Planning & Execution -->
                    <div class="w-2/3">
                        <div id="ai-welcome" class="text-center py-12 text-gray-500">
                            Select a service from the left to start.
                        </div>

                        <div id="ai-service-panel" class="hidden">
                            <h3 id="panel-title" class="text-xl font-bold mb-4"></h3>

                            <button id="btn-generate-plan" class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick="generatePlan()">
                                Generate Architecture Plan
                            </button>

                            <div id="loading-spinner" class="hidden mt-4 text-gray-600 flex items-center gap-2">
                                <svg class="animate-spin h-5 w-5 text-blue-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                                <span id="loading-text">Generating plan... this may take a minute.</span>
                            </div>

                            <div id="plan-container" class="hidden mt-6">
                                <h4 class="font-semibold text-lg border-b pb-2 mb-3">Proposed Plan</h4>
                                <div id="plan-content" class="prose max-w-none bg-gray-50 p-4 rounded border border-gray-200 overflow-auto max-h-96 text-sm"></div>

                                <div class="mt-4 flex gap-3">
                                    <button class="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded" onclick="executePlan()">
                                        Approve & Execute
                                    </button>
                                </div>
                            </div>

                            <div id="execution-result" class="hidden mt-6">
                                <h4 class="font-semibold text-lg border-b pb-2 mb-3 text-green-700">Execution Successful!</h4>
                                <div id="validation-report" class="prose max-w-none bg-blue-50 p-4 rounded border border-blue-200 overflow-auto max-h-96 text-sm"></div>
                            </div>

                            <div id="error-container" class="hidden mt-6 text-red-600 bg-red-50 p-4 rounded border border-red-200"></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script>
        const IS_AI_DISABLED = {};
        let currentService = "";
        let currentPlan = "";
        let currentManifest = "";
        let currentExistingCode = "";

        async function fetchComposeStatus() {{
            try {{
                const res = await fetch('/api/compose/status');
                const data = await res.json();

                if (data.status === 'success') {{
                    // Reset all statuses to off
                    document.querySelectorAll('[id^="status-"]').forEach(el => {{
                        el.textContent = 'Exited/Not Running';
                        el.className = 'ml-2 px-2 py-1 text-xs font-semibold rounded bg-gray-200 text-gray-700';
                    }});

                    if (data.containers && Array.isArray(data.containers)) {{
                        data.containers.forEach(c => {{
                            const serviceName = c.Service;
                            const state = c.State; // running, exited, etc.
                            const el = document.getElementById('status-' + serviceName);
                            if (el) {{
                                el.textContent = state.charAt(0).toUpperCase() + state.slice(1);
                                if (state === 'running') {{
                                    el.className = 'ml-2 px-2 py-1 text-xs font-semibold rounded bg-green-100 text-green-800';
                                }} else {{
                                    el.className = 'ml-2 px-2 py-1 text-xs font-semibold rounded bg-red-100 text-red-800';
                                }}
                            }}
                        }});
                    }}
                }}
            }} catch(e) {{
                console.error("Failed to fetch status", e);
            }}
        }}

        async function composeAction(action, service) {{
            const btnText = service ? service : "all services";
            console.log(`Executing ${{action}} on ${{btnText}}`);
            try {{
                const res = await fetch('/api/compose/action', {{
                    method: 'POST',
                    headers: {{'Content-Type': 'application/json'}},
                    body: JSON.stringify({{ action, service }})
                }});
                const data = await res.json();
                if (data.error) {{
                    alert("Error: " + data.error);
                }} else {{
                    setTimeout(fetchComposeStatus, 1000);
                }}
            }} catch (e) {{
                alert("Request failed: " + e);
            }}
        }}

        let logsInterval = null;

        async function viewLogs(service) {{
            document.getElementById('logs-panel').classList.remove('hidden');
            document.getElementById('log-service-name').textContent = service;
            document.getElementById('logs-content').textContent = "Loading...";

            if (logsInterval) clearInterval(logsInterval);

            const fetchLogs = async () => {{
                try {{
                    const res = await fetch('/api/compose/logs/' + service);
                    const data = await res.json();
                    if (data.status === 'success') {{
                        const logsContent = document.getElementById('logs-content');
                        logsContent.textContent = data.logs || "No logs available.";
                        logsContent.scrollTop = logsContent.scrollHeight;
                    }}
                }} catch (e) {{
                    console.error("Log fetch failed", e);
                }}
            }};

            fetchLogs();
            logsInterval = setInterval(fetchLogs, 3000); // Poll logs every 3 seconds
        }}

        function closeLogs() {{
            document.getElementById('logs-panel').classList.add('hidden');
            if (logsInterval) clearInterval(logsInterval);
        }}

        // Periodically refresh compose status
        setInterval(fetchComposeStatus, 5000);

        function switchTab(tabId) {{
            document.querySelectorAll('.tab-content').forEach(el => el.classList.add('hidden'));
            document.getElementById('content-' + tabId).classList.remove('hidden');

            document.getElementById('tab-compose').className = "inline-block p-4 border-b-2 border-transparent hover:text-gray-600 hover:border-gray-300 rounded-t-lg text-gray-500";
            document.getElementById('tab-ai').className = "inline-block p-4 border-b-2 border-transparent hover:text-gray-600 hover:border-gray-300 rounded-t-lg text-gray-500";

            document.getElementById('tab-' + tabId).className = "inline-block p-4 border-b-2 border-blue-600 rounded-t-lg text-blue-600";
        }}

        async function fetchServices() {{
            try {{
                const res = await fetch('/api/services');
                const data = await res.json();
                const list = document.getElementById('ai-services-list');

                if (data.services && data.services.length > 0) {{
                    list.innerHTML = data.services.map(s =>
                        `<li><button class="w-full text-left p-2 rounded hover:bg-blue-50 text-gray-700 font-medium" onclick="selectService('${{s}}')">📦 ${{s}}</button></li>`
                    ).join('');
                }} else {{
                    list.innerHTML = '<li class="text-gray-500 text-sm">No services found in path.</li>';
                }}
            }} catch (e) {{
                document.getElementById('ai-services-list').innerHTML = '<li class="text-red-500">Error loading services.</li>';
            }}
        }}

        function selectService(name) {{
            currentService = name;
            currentPlan = "";
            currentManifest = "";
            currentExistingCode = "";

            document.getElementById('ai-welcome').classList.add('hidden');
            document.getElementById('ai-service-panel').classList.remove('hidden');
            document.getElementById('panel-title').textContent = `Service: ${{name}}`;

            document.getElementById('btn-generate-plan').classList.remove('hidden');
            document.getElementById('plan-container').classList.add('hidden');
            document.getElementById('execution-result').classList.add('hidden');
            document.getElementById('error-container').classList.add('hidden');
        }}

        function showLoading(text) {{
            document.getElementById('loading-spinner').classList.remove('hidden');
            document.getElementById('loading-text').textContent = text;
            document.getElementById('error-container').classList.add('hidden');
        }}

        function hideLoading() {{
            document.getElementById('loading-spinner').classList.add('hidden');
        }}

        function showError(msg) {{
            const err = document.getElementById('error-container');
            err.textContent = msg;
            err.classList.remove('hidden');
        }}

        async function generatePlan() {{
            document.getElementById('btn-generate-plan').classList.add('hidden');
            showLoading("Reading manifest and generating AI plan...");

            try {{
                const res = await fetch('/api/plan', {{
                    method: 'POST',
                    headers: {{'Content-Type': 'application/json'}},
                    body: JSON.stringify({{service: currentService}})
                }});
                const data = await res.json();

                if (data.error) {{
                    showError(data.error);
                    document.getElementById('btn-generate-plan').classList.remove('hidden');
                }} else {{
                    currentPlan = data.plan;
                    currentManifest = data.manifest;
                    currentExistingCode = data.existing_code;

                    document.getElementById('plan-content').innerHTML = marked.parse(currentPlan);
                    document.getElementById('plan-container').classList.remove('hidden');
                }}
            }} catch (e) {{
                showError("Network error occurred.");
                document.getElementById('btn-generate-plan').classList.remove('hidden');
            }} finally {{
                hideLoading();
            }}
        }}

        async function executePlan() {{
            document.getElementById('plan-container').classList.add('hidden');
            showLoading("Executing plan... writing files and running validation. This takes a while.");

            try {{
                const res = await fetch('/api/execute', {{
                    method: 'POST',
                    headers: {{'Content-Type': 'application/json'}},
                    body: JSON.stringify({{
                        service: currentService,
                        plan: currentPlan,
                        manifest: currentManifest,
                        existing_code: currentExistingCode
                    }})
                }});
                const data = await res.json();

                if (data.error) {{
                    showError(data.error);
                    document.getElementById('plan-container').classList.remove('hidden'); // allow retry
                }} else {{
                    document.getElementById('execution-result').classList.remove('hidden');
                    if (data.validation) {{
                        document.getElementById('validation-report').innerHTML = "<strong>Validation Agent Report:</strong><br><br>" + marked.parse(data.validation);
                    }} else {{
                        document.getElementById('validation-report').innerHTML = "No validation rules found for this run.";
                    }}
                }}
            }} catch (e) {{
                showError("Network error occurred.");
                document.getElementById('plan-container').classList.remove('hidden');
            }} finally {{
                hideLoading();
            }}
        }}

        if (IS_AI_DISABLED) {{
            document.getElementById('ai-disabled-msg').classList.remove('hidden');
        }} else {{
            document.getElementById('ai-app').classList.remove('hidden');
            fetchServices();
        }}

        // Initial fetch
        fetchComposeStatus();

    </script>
</body>
</html>
"#,
    repo,
    compose_list_html,
    if compose_services.is_empty() { "<p class='text-gray-500 italic mt-2'>No services found in docker-compose.yml</p>" } else { "" },
    ai_tab_disabled
    )
}

