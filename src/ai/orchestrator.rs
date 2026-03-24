use std::fs;
use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::ai::providers::get_provider;
use crate::ai::generator::{generate_plan, execute_plan, validate_code};
use std::io::{self, Write};

pub fn run_ai_generation(path: &str, config: &Config) {
    let base_path = Path::new(path);
    if !base_path.exists() || !base_path.is_dir() {
        eprintln!("❌ Erro: O caminho '{}' não existe ou não é um diretório.", path);
        return;
    }

    let provider_res = get_provider(config);
    if let Err(e) = provider_res {
        eprintln!("❌ Falha ao inicializar o provedor de IA: {}", e);
        return;
    }

    let provider = provider_res.unwrap();
    println!("🤖 Inicializando o Gerador de IA FastGen...");

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
        println!("⚠️ Nenhum diretório de serviço encontrado em '{}'.", path);
        return;
    }

    for service_dir in services {
        let service_name = service_dir.file_name().unwrap().to_string_lossy().into_owned();
        println!("\n🔍 Inspecionando o serviço: {}", service_name);

        // Find manifest (.md or .yml)
        let mut manifest_content = String::new();
        let mut manifest_path = None;
        if let Ok(entries) = fs::read_dir(&service_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() && p.extension().map(|e| e == "md" || e == "yml" || e == "yaml").unwrap_or(false) {
                    let file_name = p.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                    if let Ok(content) = fs::read_to_string(&p) {
                        println!("📄 Utilizando manifesto: {}", file_name);
                        manifest_content = content;
                        manifest_path = Some(p);
                        break;
                    }
                }
            }
        }

        if manifest_content.is_empty() {
            println!("⚠️ Nenhum manifesto (.md ou .yml) encontrado no serviço '{}'. Ignorando...", service_name);
            continue;
        }

        // Scan for existing code
        let mut existing_code = String::new();
        fn read_existing_code(dir: &Path, acc: &mut String, manifest_path: Option<&Path>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = path.file_name().unwrap_or_default().to_string_lossy();

                    // Skip hidden files/directories (like .git, .env) and common build/dependency folders
                    if name.starts_with('.') || name == "target" || name == "node_modules" || name == "build" || name == "dist" || name == "venv" {
                        continue;
                    }

                    if path.is_dir() {
                        read_existing_code(&path, acc, manifest_path);
                    } else if path.is_file() {
                        // Skip the manifest file itself
                        if let Some(mp) = manifest_path {
                            if path == mp {
                                continue;
                            }
                        }

                        // Try reading text files, ignore binary/unreadable files
                        if let Ok(content) = fs::read_to_string(&path) {
                            acc.push_str(&format!("\n\n--- Arquivo Existente: {:?} ---\n\n{}", path, content));
                        }
                    }
                }
            }
        }

        read_existing_code(&service_dir, &mut existing_code, manifest_path.as_deref());

        if !existing_code.is_empty() {
            println!("🧩 Código existente encontrado em '{}'. O plano irá considerar refatoração/atualização.", service_name);
        } else {
            println!("🌱 Nenhum código pré-existente encontrado em '{}'. O plano irá gerar do zero.", service_name);
        }

        // 1. Plan
        println!("🧠 Gerando plano de arquitetura para {}...", service_name);

        let mut current_plan = match generate_plan(&provider, &manifest_content, &service_name, &existing_code, None, None) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ Failed to generate plan: {}", e);
                continue;
            }
        };

        let mut approved = false;

        while !approved {
            println!("\n====================================");
            println!("📋 PLANO PARA O SERVIÇO: {}", service_name);
            println!("====================================\n");
            println!("{}", current_plan);
            println!("\n====================================");

            print!("Você aprova este plano? [Y/n/i] (i = interagir/feedback): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input_trimmed = input.trim().to_lowercase();

            if input_trimmed == "n" {
                println!("⏭️  Pulando o serviço '{}'.", service_name);
                break;
            } else if input_trimmed == "i" {
                print!("💬 Digite seu feedback ou recomendação de mudança: ");
                io::stdout().flush().unwrap();
                let mut feedback = String::new();
                io::stdin().read_line(&mut feedback).unwrap();
                let feedback_trimmed = feedback.trim();

                if !feedback_trimmed.is_empty() {
                    println!("🔄 Gerando novo plano com base no seu feedback...");
                    match generate_plan(&provider, &manifest_content, &service_name, &existing_code, Some(feedback_trimmed), Some(&current_plan)) {
                        Ok(new_plan) => {
                            current_plan = new_plan;
                        },
                        Err(e) => {
                            eprintln!("❌ Falha ao regenerar o plano: {}", e);
                            println!("⚠️ Mantendo o plano anterior.");
                        }
                    }
                } else {
                    println!("⚠️ Nenhum feedback fornecido. O plano não foi alterado.");
                }
            } else {
                approved = true;
            }
        }

        if !approved {
            continue;
        }

        // 2. Execute
        println!("🛠️  Gerando arquivos para {}...", service_name);
        if let Err(e) = execute_plan(&provider, &manifest_content, &service_name, &current_plan, &existing_code, &service_dir) {
            eprintln!("❌ Falha ao gerar arquivos para {}: {}", service_name, e);
            continue;
        }
        println!("✅ Geração concluída para o serviço '{}'.", service_name);

        // 3. Validation
        if let Some(rules) = &validation_rules {
            println!("\n🛡️  Executando Agente de Validação para o serviço {}...", service_name);
            match validate_code(&provider, &service_dir, rules) {
                Ok(report) => {
                    println!("\n--- Relatório de Validação ---\n");
                    println!("{}", report);
                    println!("\n------------------------------\n");

                    print!("Você aprova os resultados da validação? [Y/n]: ");
                    io::stdout().flush().unwrap();
                    let mut val_input = String::new();
                    io::stdin().read_line(&mut val_input).unwrap();
                    if val_input.trim().to_lowercase() == "n" {
                        println!("⚠️ Validação do serviço '{}' foi rejeitada.", service_name);
                    } else {
                        println!("✅ Serviço '{}' validado e finalizado com sucesso.", service_name);
                    }
                },
                Err(e) => eprintln!("❌ Falha na validação: {}", e)
            }
        }
    }

    println!("\n🎉 Todos os serviços foram processados com sucesso!");
}
