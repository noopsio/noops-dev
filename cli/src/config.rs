use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Config {
    pub jwt_file: PathBuf,
    pub base_url: String,
    pub manifest: PathBuf,
    pub templates_dir: PathBuf,
    pub template_manifest: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "io".to_string(),
            author: "Noops".to_string(),
            app_name: "noops".to_string(),
        })
        .unwrap();

        fs::create_dir_all(strategy.cache_dir()).unwrap();

        Self {
            jwt_file: strategy.in_cache_dir("jwt"),
            base_url: "http://localhost:8080/api/".to_string(),
            manifest: Path::new("./noops.yaml").to_path_buf(),
            templates_dir: strategy.in_cache_dir("templates"),
            template_manifest: strategy.in_cache_dir("templates").join("manifest.yaml"),
        }
    }
}
