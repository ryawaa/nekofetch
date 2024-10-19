use std::collections::HashMap;
use std::env;
use sysinfo::{Disks, System};
use whoami;

pub fn gather_system_info(sys: &System) -> HashMap<String, String> {
    let mut info_map = HashMap::new();

    // OS information
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
    let os_long_version = format!("{} {}", os_name, os_version);

    // Kernel version
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

    // Uptime
    let uptime = System::uptime();
    let uptime_days = uptime / 86400;
    let uptime_hours = (uptime % 86400) / 3600;
    let uptime_minutes = (uptime % 3600) / 60;
    let uptime_str = format!(
        "{} days, {} hours, {} mins",
        uptime_days, uptime_hours, uptime_minutes
    );

    // Shell
    let shell = env::var("SHELL")
        .or_else(|_| env::var("ComSpec"))
        .unwrap_or_else(|_| "Unknown".to_string());

    // Terminal
    let terminal = env::var("TERM_PROGRAM")
        .or_else(|_| env::var("TERM"))
        .unwrap_or_else(|_| "Unknown".to_string());

    // CPU
    let cpu_brand = sys
        .cpus()
        .get(0)
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let cpu_cores = sys.cpus().len();
    let cpu_info = format!("{} ({} cores)", cpu_brand, cpu_cores);

    // Memory
    let total_memory = sys.total_memory() / 1024;
    let used_memory = (sys.total_memory() - sys.available_memory()) / 1024;
    let memory_info = format!("{}MiB / {}MiB", used_memory, total_memory);

    // Hostname
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

    // Username
    let username = whoami::username();

    // GPU Info
    let gpu_info = get_gpu_info();

    // Add the gathered information to the map
    info_map.insert("username".to_string(), username);
    info_map.insert("hostname".to_string(), hostname);
    info_map.insert("os".to_string(), os_long_version);
    info_map.insert("kernel".to_string(), kernel_version);
    info_map.insert("uptime".to_string(), uptime_str);
    info_map.insert("shell".to_string(), shell);
    info_map.insert("terminal".to_string(), terminal);
    info_map.insert("cpu".to_string(), cpu_info);
    info_map.insert("memory".to_string(), memory_info);
    info_map.insert("gpu".to_string(), gpu_info);

    // Additional Info (if needed)
    let package_count = get_package_count();
    info_map.insert("packages".to_string(), package_count);

    let resolution = get_resolution();
    info_map.insert("resolution".to_string(), resolution);

    let de = get_desktop_environment();
    info_map.insert("de".to_string(), de);

    let wm = get_window_manager();
    info_map.insert("wm".to_string(), wm);

    let wm_theme = get_wm_theme();
    info_map.insert("wm_theme".to_string(), wm_theme);

    let storage_info: Vec<String> = get_storage_info();
    for (_index, disk_info) in storage_info.iter().enumerate() {
        info_map.insert("storage".to_string(), disk_info.clone());
    }

    info_map
}

fn get_storage_info() -> Vec<String> {
    let mut storage_info = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let total_space = disk.total_space() / 1024 / 1024; // Convert to MiB
        let available_space = disk.available_space() / 1024 / 1024; // Convert to MiB
        let used_space = total_space - available_space;
        let disk_info = format!(
            "{}: {}MiB / {}MiB",
            disk.name().to_string_lossy(),
            used_space,
            total_space
        );
        
        storage_info.push(disk_info);
        }
    storage_info
}

fn get_package_count() -> String {
    if cfg!(target_os = "linux") {
        // For dpkg-based systems
        if let Ok(output) = std::process::Command::new("bash")
            .arg("-c")
            .arg("dpkg --list | wc -l")
            .output()
        {
            let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return format!("{} (dpkg)", count);
        }
        // For pacman-based systems
        if let Ok(output) = std::process::Command::new("bash")
            .arg("-c")
            .arg("pacman -Q | wc -l")
            .output()
        {
            let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return format!("{} (pacman)", count);
        }
    } else if cfg!(target_os = "macos") {
        // For Homebrew
        if let Ok(output) = std::process::Command::new("brew").arg("list").output() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            return format!("{} (brew)", count);
        }
    }
    "Unknown".to_string()
}

fn get_resolution() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(output) = std::process::Command::new("xdpyinfo").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("dimensions:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return parts[1].to_string();
                    }
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(output) = std::process::Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.trim().starts_with("Resolution:") {
                    let res = line.trim().replace("Resolution:", "").trim().to_string();
                    return res;
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_desktop_environment() -> String {
    if cfg!(target_os = "linux") {
        env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        "Aqua".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn get_window_manager() -> String {
    if cfg!(target_os = "linux") {
        env::var("XDG_SESSION_DESKTOP").unwrap_or_else(|_| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        "Quartz Compositor".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn get_wm_theme() -> String {
    if cfg!(target_os = "macos") {
        "Blue (Dark)".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn get_gpu_info() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(output) = std::process::Command::new("lspci").arg("-mm").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("VGA compatible controller") || line.contains("3D controller") {
                    let parts: Vec<&str> = line.split('"').collect();
                    if parts.len() >= 4 {
                        return parts[3].to_string();
                    }
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(output) = std::process::Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.trim().starts_with("Chipset Model:") {
                    let gpu = line.trim().replace("Chipset Model:", "").trim().to_string();
                    return gpu;
                }
            }
        }
    } else if cfg!(target_os = "windows") {
        // For Windows, use the 'wmic' command
        if let Ok(output) = std::process::Command::new("wmic")
            .args(&["path", "win32_VideoController", "get", "name"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            if lines.len() >= 2 {
                return lines[1].trim().to_string();
            }
        }
    }
    "Unknown".to_string()
}
