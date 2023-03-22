use crate::{client, modules::templates, print};

use super::{load_config, print_modules};

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

pub fn module_add() -> anyhow::Result<()> {
    let config = load_config();
    println!("Creating new module");
    templates::create(config)?;
    Ok(())
}
