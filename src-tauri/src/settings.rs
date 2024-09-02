use std::{
    fs::File,
    io::{Read, Write},
};

use dirs::home_dir;
use serde::{Deserialize, Serialize};
use swordfish_utilities::config_filepath;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub launch_shortcut: String,
    pub search_directories: Vec<String>,
}

fn get_default_search_directories() -> Vec<String> {
    let home_path = home_dir().expect("couldn't find the home dir!");
    let home_path = home_path
        .to_str()
        .expect("Could convert the home directory path to a string!");

    #[cfg(target_os = "macos")]
    {
        vec![
            format!("{}/Desktop", home_path),
            format!("{}/Downloads", home_path),
            format!("{}/Documents", home_path),
            "/System/Applications".to_string(),
            "/Applications".to_string(),
        ]
    }
    #[cfg(target_os = "windows")]
    {
        vec![
            format!("{}\\Desktop", home_path),
            format!("{}\\Downloads", home_path),
            "C:\\Program Files",
            "C:\\Program Files (x86)",
        ]
    }

    #[cfg(target_os = "linux")]
    {
        vec![
            "/usr/share/applications",
            "/usr/local/share/applications",
            "/usr/bin",
            "/usr/sbin",
        ]
    }
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            launch_shortcut: "Control+Space".to_string(),
            search_directories: get_default_search_directories(),
        }
        .read()
    }

    pub fn get_search_directories(&self) -> Option<Vec<String>> {
        home_dir().and_then(|path| {
            path.to_str().map(|path| {
                self.search_directories
                    .iter()
                    .map(|dir| {
                        if dir.starts_with("~") {
                            dir.replace("~", path)
                        } else {
                            dir.to_string()
                        }
                    })
                    .collect()
            })
        })
    }

    pub fn read(&self) -> Self {
        config_filepath()
            .and_then(|filepath| match File::open(filepath) {
                Ok(file) => Some(file),
                Err(e) => {
                    eprintln!("Error opening config file: {}", e);
                    self.write();
                    return None;
                }
            })
            .and_then(|mut file| {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    Some(contents)
                } else {
                    None
                }
            })
            .and_then(|fcontent| {
                if let Ok(config) = self.from_json(fcontent) {
                    Some(config)
                } else {
                    None
                }
            })
            .unwrap_or(self.clone())
    }

    pub fn write(&self) -> Self {
        config_filepath()
            .and_then(|filepath| match File::create(filepath) {
                Ok(file) => Some(file),
                Err(e) => {
                    eprintln!("Error creating the config file: {}", e);
                    return None;
                }
            })
            .and_then(|mut file| match self.to_json() {
                Ok(file_content) => {
                    if file.write_all(file_content.as_bytes()).is_ok() {
                        Some(self.clone())
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing config into a string {}", e);
                    return None;
                }
            })
            .unwrap_or(self.clone())
    }

    pub fn from_json(&self, f: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&f)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
