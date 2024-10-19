mod config;
mod system_info;
mod ascii_art;
mod display;

use clap::{Arg, Command};
use sysinfo::System;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::config::Config;
use colored::{Color, Colorize};

fn main() {
    let (version, author, description) = get_metadata_from_cargo_toml().unwrap_or_else(|| (
        "Unknown".to_string(),
        "Unknown".to_string(),
        "A cat-themed system information tool".to_string(),
    ));

    let matches = Command::new("nekofetch")
        .version(Box::leak(
            format!(
                "{}\nRepository: https://github.com/ryawaa/nekofetch\nSpecial thanks to Joan G. Stark for ASCII art inspiration",
                version
            )
            .into_boxed_str(),
        ) as &str)
        .author(Box::leak(author.into_boxed_str()) as &str)
        .about(Box::leak(description.into_boxed_str()) as &str)
        .arg(
            Arg::new("no_ascii")
                .long("no-ascii")
                .help("Do not display the ASCII art")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("mini")
                .long("mini")
                .help("Display a minimal version")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("caps")
                .long("caps")
                .help("Capitalize labels")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("blahaj")
                .long("blahaj")
                .alias("haj")
                .help("Display the Blahaj ASCII art")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("colors")
                .long("colors")
                .help("Display terminal colors")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("blahaj") {
        println!(
            "{}",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣾⣧⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⡀⢀⣀⡀⣤⣤⣤⣤⣤⣤⣤⣶⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢻⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢀⡀⣠⣤⣶⣴⣿⣿⣿⣾⢷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠀⠀⠀⠀⠀⠀⠀⣠⡀⠀⠀⣹⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣤⣄⣀⣠⣾⣿⡇⠀⠀⢹⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣄⣀⣘⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀
⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⣄⣀⠀⠀
⠀⢻⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠄
⠀⠀⠳⣄⡀⠈⠉⠉⠉⠛⠛⠛⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠛⠁⠀⠉⠉⠉⠉⠉⠀⠀⠀
⠀⠀⠀⠀⠙⠲⣖⣤⣤⣆⣤⣀⣄⣄⣀⣀⣀⠀⠈⠉⠉⠛⠛⠻⠿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡿⠿⠿⠿⠿⣿⣿⣿⠿⠿⠛⠛⠛⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠉⠛⠯⣍⣉⠉⠋⠝⠉⠋⠅⠐⠀⠀⠀⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣶⣅⣰⠦⠗⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠒⣓⣒⣒⡒⠢⠤⠤⠦⠤⠦⠶⣬⡟⢿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠛⠿⠿⢿⣿⣿⣿⣿⣿⣿⡿⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"
        );
        return;
    }


    let show_ascii = !matches.get_flag("no_ascii");
    let is_mini = matches.get_flag("mini");
    let use_caps = matches.get_flag("caps");

    let config = config::load_config();

    let mut sys = System::new_all();
    sys.refresh_all();

    // Gather system information
    let info_map = system_info::gather_system_info(&sys);

    // Build the information lines based on the config
    let info = build_info_lines(&info_map, &config, use_caps, is_mini);

    // Get the ASCII art
    let ascii_art = if is_mini {
        vec![
            " /\\_/\\       ",
            "( o.o )      ",
            " > ^ <       ",
            " =====       ",
        ]
    } else if show_ascii {
        ascii_art::get_random_ascii_art()
    } else {
        vec![]
    };

    // Display the information with optional colors
    let show_colors = matches.get_flag("colors") && !is_mini;
    display::display_info(&ascii_art, &info, show_colors);
}

fn get_metadata_from_cargo_toml() -> Option<(String, String, String)> {
    let cargo_toml_path = Path::new("Cargo.toml");
    if let Ok(contents) = fs::read_to_string(cargo_toml_path) {
        for line in contents.lines() {
            if line.starts_with("version") {
                let version = line
                    .split('=')
                    .nth(1)
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                let author = contents
                    .lines()
                    .find(|l| l.starts_with("authors"))?
                    .split('=')
                    .nth(1)
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                let description = contents
                    .lines()
                    .find(|l| l.starts_with("description"))?
                    .split('=')
                    .nth(1)
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .unwrap_or_else(|| "A cat-themed system information tool".to_string());
                return Some((version, author, description));
            }
        }
    }
    None
}


fn build_info_lines(
    info_map: &HashMap<String, String>,
    config: &Config,
    use_caps: bool,
    is_mini: bool,
) -> Vec<String> {
    let mut info = Vec::new();

    if let Some(true) = config.show_username {
        let username = &info_map["username"];
        if let Some(true) = config.show_hostname {
            let hostname = &info_map["hostname"];
            info.push(format!(
                "{}",
                format!("{}@{}", username, hostname)
                    .color(Color::BrightCyan)
                    .bold()
            ));
        } else {
            info.push(username.color(Color::BrightCyan).bold().to_string());
        }
    }

    if is_mini {
        let mini_entries = vec!["os", "cpu", "memory"];
        for key in mini_entries {
            if let Some(true) = config_value(config, key) {
                let value = &info_map[key];
                let color = get_label_color(config, key);
                let label = if use_caps {
                    key.to_uppercase()
                } else {
                    key.to_string()
                };
                info.push(format!("{} {}", label.color(color).bold(), value));
            }
        }
        return info;
    }

    let entries = vec![
        ("os", "OS"),
        ("host", "Host"),
        ("kernel", "Kernel"),
        ("uptime", "Uptime"),
        ("packages", "Packages"),
        ("shell", "Shell"),
        ("resolution", "Resolution"),
        ("de", "DE"),
        ("wm", "WM"),
        ("wm_theme", "WM Theme"),
        ("terminal", "Terminal"),
        ("cpu", "CPU"),
        ("gpu", "GPU"),
        ("memory", "Memory"),
        ("storage", "Storage"),
    ];

    for (key, label) in entries {
        if let Some(true) = config_value(config, key) {
            let value = info_map
                .get(key)
                .unwrap_or(&"Unknown".to_string())
                .clone();
            let color = get_label_color(config, key);
            let label = if use_caps {
                label.to_uppercase()
            } else {
                label.to_lowercase()
            };
            info.push(format!("{} {}", label.color(color).bold(), value));
        }
    }

    info
}

fn config_value(config: &Config, key: &str) -> Option<bool> {
    match key {
        "os" => config.show_os,
        "host" => config.show_hostname,
        "kernel" => config.show_kernel,
        "uptime" => config.show_uptime,
        "packages" => config.show_packages,
        "shell" => config.show_shell,
        "resolution" => config.show_resolution,
        "de" => config.show_de,
        "wm" => config.show_wm,
        "wm_theme" => config.show_wm_theme,
        "terminal" => config.show_terminal,
        "cpu" => config.show_cpu,
        "gpu" => config.show_gpu,
        "memory" => config.show_memory,
        "storage" => config.show_storage,
        _ => Some(true),
    }
}

fn get_label_color<'a>(_config: &'a Config, _key: &'a str) -> Color {
    Color::BrightCyan // Skyblue color
}








