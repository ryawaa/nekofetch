use serde::Deserialize;
use std::fs;
use dirs;

#[derive(Deserialize)]
pub struct Config {
    pub show_username: Option<bool>,
    pub show_hostname: Option<bool>,
    pub show_os: Option<bool>,
    pub show_kernel: Option<bool>,
    pub show_uptime: Option<bool>,
    pub show_packages: Option<bool>,
    pub show_shell: Option<bool>,
    pub show_resolution: Option<bool>,
    pub show_de: Option<bool>,
    pub show_wm: Option<bool>,
    pub show_wm_theme: Option<bool>,
    pub show_terminal: Option<bool>,
    pub show_cpu: Option<bool>,
    pub show_gpu: Option<bool>,
    pub show_memory: Option<bool>,
    pub show_storage: Option<bool>,
}

pub fn load_config() -> Config {
    let config_paths = vec![
        dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("nekofetch/nekofetch_config.yml"),
        std::path::PathBuf::from("nekofetch_config.yml"),
    ];

    for path in config_paths {
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = serde_yml::from_str::<Config>(&contents) {
                return config;
            }
        }
    }

    Config {
        show_storage: Some(true),
        show_username: Some(true),
        show_hostname: Some(true),
        show_os: Some(true),
        show_kernel: Some(true),
        show_uptime: Some(true),
        show_packages: Some(true),
        show_shell: Some(true),
        show_resolution: Some(true),
        show_de: Some(true),
        show_wm: Some(true),
        show_wm_theme: Some(true),
        show_terminal: Some(true),
        show_cpu: Some(true),
        show_gpu: Some(true),
        show_memory: Some(true),
    }
}
