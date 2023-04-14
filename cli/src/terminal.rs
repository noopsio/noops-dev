use console;
use dialoguer::{Confirm, Input, Select};

pub struct Terminal {
    term: console::Term,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            term: console::Term::stdout(),
        }
    }

    pub fn writeln(&self, line: impl AsRef<str>) -> anyhow::Result<()> {
        self.term.write_line(line.as_ref())?;
        Ok(())
    }

    pub fn text_prompt(&self, text: &str) -> anyhow::Result<String> {
        let response = Input::new().with_prompt(text).interact_on(&self.term)?;
        Ok(response)
    }

    pub fn confirm_prompt(&self, text: &str) -> anyhow::Result<bool> {
        let response = Confirm::new()
            .with_prompt(text)
            .wait_for_newline(true)
            .default(false)
            .show_default(true)
            .interact_on(&self.term)?;
        Ok(response)
    }

    pub fn select_prompt(&self, text: &str, items: &[impl ToString]) -> anyhow::Result<usize> {
        let selection = Select::new()
            .with_prompt(text)
            .items(items)
            .default(0)
            .interact_on(&self.term)?;
        Ok(selection)
    }
}
