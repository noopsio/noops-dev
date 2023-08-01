use reqwest::Url;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub struct Config {
    pub jwt_file: PathBuf,
    pub base_url: Url,
}

impl Default for Config {
    fn default() -> Self {
        let xdg_config_home = env::var("XDG_CONFIG_HOME").unwrap();
        let config_path = Path::new(&xdg_config_home).join("noops");
        fs::create_dir_all(config_path.clone()).unwrap();

        Self {
            jwt_file: config_path.join("jwt"),
            base_url: Url::parse("http://localhost:8080/api/").unwrap(),
        }
    }
}
