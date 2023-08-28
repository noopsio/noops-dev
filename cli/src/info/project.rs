use std::fmt::Display;

use super::component::ComponentInformation;

pub struct ProjectInformation {
    name: String,
    deployed: bool,
    components: Vec<ComponentInformation>,
}

impl ProjectInformation {
    pub fn new(name: String, deployed: bool, components: Vec<ComponentInformation>) -> Self {
        Self {
            name,
            deployed,
            components,
        }
    }
}

impl Display for ProjectInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Name:\t\t{}\nDeployed:\t{}",
            self.name, self.deployed
        ))?;
        f.write_fmt(format_args!("\n\nComponents:\t{}", self.components.len()))?;
        for component in &self.components {
            f.write_fmt(format_args!("\n\n{}", component))?;
        }

        Ok(())
    }
}
