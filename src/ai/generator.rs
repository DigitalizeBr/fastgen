use std::fs;
use std::path::Path;
use std::io::Write;
use crate::ai::providers::{LlmProvider, GeneratedFile};

pub fn generate_plan(provider: &Box<dyn LlmProvider>, manifest_content: &str, service_name: &str, existing_code: &str, feedback: Option<&str>, previous_plan: Option<&str>) -> Result<String, String> {
    let existing_code_section = if !existing_code.is_empty() {
        format!(
            "The directory already contains some code. You should refactor, update, or extend this existing code based on the instructions, rather than starting entirely from scratch unless requested.\n\n\
            Existing Code:\n{}\n\n",
            existing_code
        )
    } else {
        String::new()
    };

    let feedback_section = match (feedback, previous_plan) {
        (Some(f), Some(p)) => format!(
            "The user reviewed your previous plan and provided the following feedback:\n{}\n\n\
            Here was your previous plan:\n{}\n\n\
            Please generate a NEW plan incorporating the user's feedback.\n\n",
            f, p
        ),
        _ => String::new(),
    };

    let prompt = format!(
        "You are FastGen AI, an expert software architect and Cloud Native engineer.\n\
        The user wants to generate or update a microservice or infrastructure component named '{}'.\n\
        Read the following manifest/instructions:\n\n\
        {}\n\n\
        {}{}Create a detailed step-by-step plan of the files and directories that will be created or modified for this service.\n\
        Do NOT generate the code yet. Just describe the architecture and the file structure in Markdown.",
        service_name, manifest_content, existing_code_section, feedback_section
    );

    provider.chat(&prompt)
}

pub fn execute_plan(
    provider: &Box<dyn LlmProvider>,
    manifest_content: &str,
    service_name: &str,
    plan_text: &str,
    existing_code: &str,
    target_dir: &Path
) -> Result<(), String> {
    let existing_code_section = if !existing_code.is_empty() {
        format!(
            "The following code already exists in the project. You should output the full, updated code for these files or new files as required by the plan.\n\n\
            Existing Code:\n{}\n\n",
            existing_code
        )
    } else {
        String::new()
    };

    let prompt = format!(
        "You are FastGen AI, an expert Cloud Native engineer.\n\
        You must generate or update the code for a service named '{}'.\n\
        Here is the original manifest:\n{}\n\n\
        Here is the approved plan:\n{}\n\n\
        {}You MUST use the function 'create_files' to generate the files. \
        Ensure all code is functional, clean, and follows best practices.",
        service_name, manifest_content, plan_text, existing_code_section
    );

    let files = provider.generate_files(&prompt)?;

    for file in files {
        let file_path = target_dir.join(Path::new(&file.path));

        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir: {}", e))?;
        }

        // Write file
        let mut f = fs::File::create(&file_path).map_err(|e| format!("Failed to create file: {}", e))?;
        f.write_all(file.content.as_bytes()).map_err(|e| format!("Failed to write to file: {}", e))?;

        println!("✅ Arquivo criado/atualizado: {:?}", file_path);
    }

    Ok(())
}

pub fn validate_code(
    provider: &Box<dyn LlmProvider>,
    target_dir: &Path,
    validation_rules: &str
) -> Result<String, String> {

    // Read all files from target_dir (recursively, skipping git/target if necessary, but here we just read everything simple)
    let mut all_code = String::new();

    // Simplistic recursive reader for context
    fn read_dir_recursive(dir: &Path, acc: &mut String) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    read_dir_recursive(&path, acc);
                } else if let Ok(content) = fs::read_to_string(&path) {
                    acc.push_str(&format!("\n\n--- File: {:?} ---\n\n{}", path, content));
                }
            }
        }
    }

    read_dir_recursive(target_dir, &mut all_code);

    let prompt = format!(
        "You are FastGen AI Security and QA Agent.\n\
        Review the following codebase against the validation rules provided.\n\
        Validation Rules:\n{}\n\n\
        Codebase:\n{}\n\n\
        Provide a detailed report on whether the code passes the validation rules. \
        List any issues found, or state clearly if everything is correct.",
        validation_rules, all_code
    );

    provider.chat(&prompt)
}
