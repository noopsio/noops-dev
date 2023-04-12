use crate::{adapter::git::GitAdapter, config::Config, modules::Module, print};

use super::Language;

pub fn create(mut config: Config) -> anyhow::Result<()> {
    let templates = ModuleTemplate::load();
    show_templates(&templates);
    let mut template = prompt_template(templates);
    prompt_module_information(&mut template);
    let new_module = Module::from(template);

    GitAdapter::clone_repository(&new_module.template, &new_module.root)?;
    println!("Adding module {} to config", &new_module.name);

    config.add_module(new_module)?;
    println!("Added module to config!");
    Ok(())
}

fn prompt_module_information(template: &mut ModuleTemplate) {
    template.module_name = prompt_question("---\nEnter module name:\n---");
    template.module_root =
        prompt_question("---\nEnter module root: (Leave blank to use module name)\n---");
}

fn prompt_template(templates: Vec<ModuleTemplate>) -> ModuleTemplate {
    let template_index =
        print::Color::prompt_number(&print::Color::White, "--- \nEnter index \n---");
    let template = templates
        .into_iter()
        .nth(template_index)
        .expect("Invalid template index");
    template
}

fn show_templates(templates: &[ModuleTemplate]) {
    let headers = vec!["Name", "Description", "Template"];
    let template_data = templates
        .iter()
        .map(|template| template.to_vec_string()) // Assuming the `into` function returns Vec<&str>
        .collect::<Vec<Vec<String>>>();

    print::Color::print_colorful(&print::Color::Red, "Choose template by index");
    let template_table = print::InteractiveTable::new(headers, &template_data);
    template_table.print_tty(true).unwrap();
}

fn prompt_question(question: &str) -> Option<String> {
    let answer = print::Color::prompt_text(&print::Color::White, question);
    if answer.trim().is_empty() {
        None
    } else {
        Some(answer)
    }
}

pub struct ModuleTemplate {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub language: Language,
    pub module_name: Option<String>,
    pub module_root: Option<String>,
}

// TODO LOAD THESE FROM URL
impl ModuleTemplate {
    pub fn load() -> Vec<ModuleTemplate> {
        vec![
            ModuleTemplate {
                name: "Rust Hello World".to_string(),
                description: "A hello world function in Rust".to_string(),
                repository: "jfcomputing/templates-rust-hello-world".to_string(),
                language: super::Language::Rust,
                module_name: None,
                module_root: None,
            },
            ModuleTemplate {
                name: "Golang Hello World".to_string(),
                description: "A hello world function in Golang".to_string(),
                repository: "jfcomputing/templates-go-hello-world".to_string(),
                language: super::Language::Golang,
                module_name: None,
                module_root: None,
            },
        ]
    }

    pub fn to_vec_string(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.description.clone(),
            self.repository.clone(),
            self.language.to_string(),
        ]
    }
}
