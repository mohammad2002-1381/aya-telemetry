use std::{fs, process::Command};

pub fn get_ppid(pid: u32) -> u32 {
    let stat_path = format!("/proc/{}/stat", pid);
    if let Ok(stat_content) = fs::read_to_string(stat_path) {
        let parts: Vec<&str> = stat_content.split_whitespace().collect();
        if parts.len() > 3 {
            return parts[3].parse::<u32>().unwrap_or(0);
        }
    }
    0
}

pub fn get_process_path(pid: u32) -> String {
    let exe_path = format!("/proc/{}/exe", pid);
    match fs::read_link(exe_path) {
        Ok(path_buf) => path_buf.to_string_lossy().into_owned(),
        Err(_) => "[unknown path]".to_string(),
    }
}

pub fn kill_process(pid: u32) -> anyhow::Result<()> {
    Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .output()?;
    Ok(())
}
