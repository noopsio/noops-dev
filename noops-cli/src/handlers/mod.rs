use crate::{
    client,
    config::{self, Config},
    print,
};

pub async fn module_delete() -> anyhow::Result<()> {
    let mut config = load_config();
    print_modules(&config);
    let module_index =
        print::Color::prompt_number(&crate::print::Color::White, "--- \nEnter index \n---");

    let module = config.get_module(module_index);
    client::NoopsClient::from(&config)
        .delete_module(module)
        .await?;
    config.delete_module(module_index)?;
    Ok(())
}

fn print_modules(config: &Config) {
    let headers = vec!["Name","Root", "Template","Description"];
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
    println!("Successfully loaded Config {}", config.name);

    config
}
