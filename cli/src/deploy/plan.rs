use super::{
    components::BuildedComponent,
    create::{self, CreateStep},
    delete::{self, DeleteStep},
    update::{self, UpdateStep},
    DeployStep,
};
use crate::terminal::Terminal;
use client::handler::HandlerClient;
use console::style;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct DeployPlan {
    steps: usize,
    create_steps: Vec<CreateStep>,
    update_steps: Vec<UpdateStep>,
    delete_steps: Vec<DeleteStep>,
}

impl DeployPlan {
    pub fn new(
        local_handlers: Vec<BuildedComponent>,
        remote_handlers: Vec<BuildedComponent>,
    ) -> Self {
        let local_handlers: HashSet<BuildedComponent> = HashSet::from_iter(local_handlers);
        let remote_handlers: HashSet<BuildedComponent> = HashSet::from_iter(remote_handlers);

        let create_steps = create::create_steps(&local_handlers, &remote_handlers);
        let update_steps = update::update_steps(&local_handlers, &remote_handlers);
        let delete_steps = delete::delete_steps(&local_handlers, &remote_handlers);

        Self {
            steps: create_steps.len() + update_steps.len() + delete_steps.len(),
            create_steps,
            update_steps,
            delete_steps,
        }
    }

    pub fn has_steps(&self) -> bool {
        self.steps > 0
    }

    pub fn deploy(
        &self,
        terminal: &Terminal,
        project: &str,
        client: &HandlerClient,
    ) -> anyhow::Result<()> {
        let mut step = 1;
        for create_step in &self.create_steps {
            let prefix = format!("[{}/{}]", step, self.steps);
            let message = format!("Creating handler {}", &create_step.0.name);
            let spinner = terminal.spinner_with_prefix(prefix, &message);
            create_step.deploy(project, client)?;
            spinner.finish_with_message(message);
            step += 1;
        }

        for update_step in &self.update_steps {
            let prefix = format!("[{}/{}]", step, self.steps);
            let message = format!("Updating handler {}", &update_step.0.name);
            let spinner = terminal.spinner_with_prefix(prefix, &message);
            update_step.deploy(project, client)?;
            spinner.finish_with_message(message);
            step += 1;
        }

        for delete_step in &self.delete_steps {
            let prefix = format!("[{}/{}]", step, self.steps);
            let message = format!("Deleting handler {}", &delete_step.0.name);
            let spinner = terminal.spinner_with_prefix(prefix, &message);
            delete_step.deploy(project, client)?;
            spinner.finish_with_message(message);
            step += 1;
        }

        Ok(())
    }
}

impl Display for DeployPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.has_steps() {
            return f.write_str("No changes");
        }

        let text = style("Changes:\n").bold();
        f.write_str(&text.to_string())?;

        for create_step in &self.create_steps {
            f.write_fmt(format_args!("{}\n", &create_step))?;
        }

        for update_step in &self.update_steps {
            f.write_fmt(format_args!("{}\n", &update_step))?;
        }

        for delete_step in &self.delete_steps {
            f.write_fmt(format_args!("{}\n", &delete_step))?;
        }

        Ok(())
    }
}
