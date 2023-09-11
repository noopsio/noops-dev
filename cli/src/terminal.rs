use console::{style, StyledObject};
use dialoguer::{Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const TICK_STRING: &[&str] = &["⠲", "⠴", "⠦", "⠖", "✔️"];

pub struct Terminal {
    term: console::Term,
}

// Allowed because console::Term does not implements Default
#[allow(clippy::new_without_default)]
impl Terminal {
    pub fn new() -> Self {
        Self {
            term: console::Term::stderr(),
        }
    }

    pub fn write_heading(&self, heading: impl AsRef<str>) -> anyhow::Result<()> {
        let heading = format!("--- {} ---", heading.as_ref());
        let heading = style(heading.as_str()).bold();
        self.write_styled_text(heading)?;
        Ok(())
    }

    pub fn write_text(&self, text: impl AsRef<str>) -> anyhow::Result<()> {
        self.term.write_str(text.as_ref())?;
        Ok(())
    }

    pub fn write_styled_text(&self, style: StyledObject<&str>) -> anyhow::Result<()> {
        self.term.write_line(&style.to_string())?;
        Ok(())
    }

    pub fn _text_prompt(&self, text: &str) -> anyhow::Result<String> {
        let response = Input::new().with_prompt(text).interact_on(&self.term)?;
        Ok(response)
    }

    pub fn confirm_prompt(&self, text: impl AsRef<str>) -> anyhow::Result<bool> {
        let response = Confirm::new()
            .with_prompt(text.as_ref())
            .wait_for_newline(true)
            .default(false)
            .show_default(true)
            .report(false)
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

    pub fn spinner(&self, message: impl AsRef<str>) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(175));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(TICK_STRING),
        );
        pb.set_message(message.as_ref().to_string());
        pb
    }

    pub fn spinner_with_prefix(
        &self,
        prefix: impl AsRef<str>,
        message: impl AsRef<str>,
    ) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(175));
        pb.set_style(
            ProgressStyle::with_template("{prefix} {spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(TICK_STRING),
        );
        pb.set_prefix(style(prefix.as_ref()).bold().dim().to_string());
        pb.set_message(message.as_ref().to_string());
        pb
    }
}
