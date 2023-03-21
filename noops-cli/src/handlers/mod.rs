pub mod modules;
pub mod project;

use crate::{
    config::{self, Config},
    print,
};

fn print_modules(config: &Config) {
    let headers = vec!["Name", "Root", "Template", "Description"];
    let modules = config
        .modules
        .iter()
        .map(|module| Vec::from(module)) // Assuming the `into` function returns Vec<&str>
        .collect::<Vec<Vec<String>>>();

    crate::print::Color::print_colorful(&print::Color::Red, "Choose Module to delete");
    let modules_table = print::InteractiveTable::new(headers, &modules);
    modules_table.print_tty(true).unwrap();
}

fn load_config() -> config::Config {
    println!("Loading Config");
    let config = config::Config::from_yaml("noops-config.yaml").unwrap_or_else(|_| {
        println!("Please init project first with 'noops init' command.");
        std::process::exit(1);
    });
    println!("Successfully loaded config {}", config.name);

    config
}
