use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
    let path_theme = Path::new("/usr/share/themes/");
    let theme_path = path_theme.join(theme_name).join("gtk-2.0").join("gtkrc");
    Ok(fs::read_to_string(theme_path)?)
}

fn parse_color_scheme(contents: &str) -> HashMap<String, String> {
    let contents = contents.replace(r"\n", "\n");
    let re = Regex::new(r#"([a-zA-Z_]+):#([0-9a-fA-F]{6})"#).unwrap();
    re.captures_iter(&contents)
        .map(|caps| (caps[1].to_string(), format!("#{}", &caps[2])))
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
