use std::fs;
use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::ai::providers::get_provider;
use crate::ai::generator::{generate_plan, execute_plan, validate_code};
use std::io::{self, Write};

pub fn run_ai_generation(path: &str, config: &Config) {
    let base_path = Path::new(path);
    if !base_path.exists() || !base_path.is_dir() {
        eprintln!("❌ Error: Path '{}' does not exist or is not a directory.", path);
        return;
    }

    let provider_res = get_provider(config);
    if let Err(e) = provider_res {
        eprintln!("❌ Failed to initialize AI provider: {}", e);
        return;
    }

    let provider = provider_res.unwrap();
    println!("🤖 Initializing FastGen AI Generator...");

    // Find services and validation rules
    let mut services: Vec<PathBuf> = Vec::new();
    let mut validation_rules: Option<String> = None;

    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let dirname = p.file_name().unwrap_or_default().to_string_lossy();

                if dirname == "validation" {
                    // Try to read a .md file inside validation
                    if let Ok(val_entries) = fs::read_dir(&p) {
                        for v_entry in val_entries.flatten() {
                            let vp = v_entry.path();
                            if vp.is_file() && vp.extension().map(|e| e == "md" || e == "yml" || e == "yaml").unwrap_or(false) {
                                if let Ok(content) = fs::read_to_string(&vp) {
                                    validation_rules = Some(content);
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    // It's a service directory
                    services.push(p);
                }
            }
        }
    }

    if services.is_empty() {
        println!("⚠️ No service directories found in {}", path);
        return;
    }

    for service_dir in services {
        let service_name = service_dir.file_name().unwrap().to_string_lossy().into_owned();
        println!("\n🔍 Inspecting service: {}", service_name);

        // Find manifest (.md or .yml)
        let mut manifest_content = String::new();
        if let Ok(entries) = fs::read_dir(&service_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() && p.extension().map(|e| e == "md" || e == "yml" || e == "yaml").unwrap_or(false) {
                    if let Ok(content) = fs::read_to_string(&p) {
                        manifest_content = content;
                        break;
                    }
                }
            }
        }

        if manifest_content.is_empty() {
            println!("⚠️ No manifest (.md or .yml) found in service '{}', skipping...", service_name);
            continue;
        }

        // 1. Plan
        println!("🧠 Generating architecture plan for {}...", service_name);
        match generate_plan(&provider, &manifest_content, &service_name) {
            Ok(plan) => {
                println!("\n====================================");
                println!("📋 PLAN FOR: {}", service_name);
                println!("====================================\n");
                println!("{}", plan);
                println!("\n====================================");

                print!("Do you approve this plan? [Y/n]: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().to_lowercase() == "n" {
                    println!("⏭️ Skipping service '{}'.", service_name);
                    continue;
                }

                // 2. Execute
                println!("🛠️  Generating files for {}...", service_name);
                if let Err(e) = execute_plan(&provider, &manifest_content, &service_name, &plan, &service_dir) {
                    eprintln!("❌ Failed to generate files for {}: {}", service_name, e);
                    continue;
                }
                println!("✅ Generation complete for {}.", service_name);

                // 3. Validation
                if let Some(rules) = &validation_rules {
                    println!("\n🛡️ Running Validation Agent for {}...", service_name);
                    match validate_code(&provider, &service_dir, rules) {
                        Ok(report) => {
                            println!("\n--- Validation Report ---\n");
                            println!("{}", report);
                            println!("\n-------------------------\n");

                            print!("Do you approve the validation results? [Y/n]: ");
                            io::stdout().flush().unwrap();
                            let mut val_input = String::new();
                            io::stdin().read_line(&mut val_input).unwrap();
                            if val_input.trim().to_lowercase() == "n" {
                                println!("⚠️ Service '{}' validation was rejected.", service_name);
                                // Here we could add a loop to re-generate or fix, but for now we just warn.
                            } else {
                                println!("✅ Service '{}' successfully validated and finished.", service_name);
                            }
                        },
                        Err(e) => eprintln!("❌ Validation failed: {}", e)
                    }
                }

            },
            Err(e) => eprintln!("❌ Failed to generate plan: {}", e)
        }
    }

    println!("\n🎉 All services processed successfully!");
}
