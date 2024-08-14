use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize)]
struct ColorScheme {
    properties: HashMap<String, String>,
}

fn get_current_theme() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()?;
    let stdout_raw = String::from_utf8(output.stdout)?;
    Ok(stdout_raw.trim().trim_matches('\'').to_string())
}

fn read_gtkrc(theme_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut theme_path = get_gtkrc(&format!("{}{}", "/usr/share/themes/", theme_name));
    if !theme_path.is_file() {
        let home_dir = env::var("HOME")?;
        theme_path = get_gtkrc(&format!("{}/.themes/{}", home_dir, theme_name));
    }

    Ok(fs::read_to_string(theme_path)?)
}

fn get_gtkrc(path: &str) -> PathBuf {
    let path_theme = Path::new(path);
    path_theme.join("gtk-2.0").join("gtkrc")
}

fn parse_color_scheme(contents: &str) -> HashMap<String, String> {
    let re = Regex::new(r#"([a-zA-Z_]+):\s*([#0-9a-zA-Z]+)"#).unwrap();

    let mut scheme_contents = String::new();
    let mut in_scheme = false;

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("gtk-color-scheme") {
            in_scheme = true;
            if let Some(index) = trimmed.find('=') {
                scheme_contents.push_str(trimmed[index + 1..].trim());
            }
        } else if in_scheme {
            if trimmed.ends_with('"') {
                scheme_contents.push_str(&trimmed[..trimmed.len() - 1]);
                in_scheme = false;
            } else {
                scheme_contents.push_str(trimmed);
            }
        }
    }
    scheme_contents = scheme_contents.replace(r"\n", " ");
    re.captures_iter(&scheme_contents)
        .map(|caps| (caps[1].to_string(), caps[2].to_string()))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme_name = get_current_theme()?;
    let contents = read_gtkrc(&theme_name)?;
    let properties = parse_color_scheme(&contents);
    let color_scheme = ColorScheme { properties };
    let json = serde_json::to_string_pretty(&color_scheme)?;
    println!("{}", json);
    Ok(())
}
