use crate::{
    adapter::git,
    client::NoopsClient,
    config::Config,
    modules::{templates::TEMPLATES, Module},
    terminal::Terminal,
};

pub async fn module_delete(
    term: &Terminal,
    mut config: Config,
    client: NoopsClient,
) -> anyhow::Result<()> {
    let index = term.select_prompt("Select a module to delete", &config.modules)?;
    let module = config.get_module(index);
    client.delete_module(module).await?;
    config.delete_module(index)?;
    Ok(())
}

pub fn module_add(term: &Terminal, mut config: Config) -> anyhow::Result<()> {
    term.writeln("Creating new module")?;

    let index = term.select_prompt("Select a template", &TEMPLATES)?;
    let mut template = TEMPLATES[index].clone();
    template.name = term.text_prompt("Enter new module name")?;
    let module = Module::from_template(template);

    git::clone_repository(&module.template, &module.root)?;
    config.add_module(module)?;

    term.writeln("Added module to config!")?;
    Ok(())
}
