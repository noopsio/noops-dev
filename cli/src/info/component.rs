use crate::manifest::Component;
use common::dtos::{GetFunctionDTO, Language};
use std::fmt::Display;

pub struct ComponentInformation {
    pub name: String,
    pub language: Language,
    pub deployed: bool,
    pub build: bool,
    pub link: String,
}

impl ComponentInformation {
    pub fn new(local_component: &Component, remote_component: Option<GetFunctionDTO>) -> Self {
        let deployed = remote_component.is_some();
        let link = if let Some(remote_component) = remote_component {
            remote_component.link
        } else {
            "N/A".to_string()
        };

        ComponentInformation {
            name: local_component.name.clone(),
            language: local_component.language,
            deployed,
            build: local_component.is_build(),
            link,
        }
    }
}

impl Display for ComponentInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Name:\t\t{}\nLanguage:\t{}\nBuild:\t\t{}\nDeployed:\t{}\nLink:\t\t{}\n",
            self.name, self.language, self.build, self.deployed, self.link
        ))
    }
}
