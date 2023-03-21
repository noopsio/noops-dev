use crate::{config::Config, helpers::GitAdapter, modules::Module, print};

pub fn create(mut config: Config) -> anyhow::Result<()> {
    let templates = ModuleTemplate::load();
    show_templates(&templates);
    let template = prompt_template(templates);
    let module_name = prompt_module_name();
    let mut new_module = Module::from(template);
    new_module.name = module_name;

    GitAdapter::clone_repository(&new_module.template, &new_module.name)?;
    println!("Adding Module {} to config", new_module.name);

    config.add_module(new_module)?;
    Ok(())
}

fn prompt_template(templates: Vec<ModuleTemplate>) -> ModuleTemplate {
    let template_index =
        print::Color::prompt_number(&crate::print::Color::White, "--- \nEnter index \n---");
    let template = templates.into_iter().nth(template_index).expect("Invalid template index");
    template
}

fn show_templates(templates: &[ModuleTemplate]) {
    let headers = vec!["Template", "Description", "Template"];
    let template_data = templates
        .iter()
        .map(|template| template.into()) // Assuming the `into` function returns Vec<&str>
        .collect::<Vec<Vec<String>>>();

    crate::print::Color::print_colorful(&print::Color::Red, "Choose Template by Number");
    let template_table = print::InteractiveTable::new(headers, &template_data);
    template_table.print_tty(true).unwrap();
}

fn prompt_module_name() -> String {
    let module_name = print::Color::prompt_text(
        &print::Color::White,
        "Name your Module (This will name the root directory)",
    );
    module_name.to_string()
}

pub struct ModuleTemplate {
    pub name: String,
    pub description: String,
    pub repository: String,
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
                module_name: None,
                module_root: None,
            },
            ModuleTemplate {
                name: "Golang Hello World".to_string(),
                description: "A hello world function in Golang".to_string(),
                repository: "jfcomputing/templates-go-hello-world".to_string(),
                module_name: None,
                module_root: None,
            },
        ]
    }
}


impl From<&ModuleTemplate> for Vec<String> {
    fn from(template: &ModuleTemplate) -> Vec<String> {
        match template {
            ModuleTemplate {
                name,
                description,
                repository,
                module_name: _,
                module_root: _,
            } => vec![name.clone(), description.clone(), repository.clone()],
        }
    }
}
