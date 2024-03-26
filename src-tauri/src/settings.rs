use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub launch_shortcut: String,
    pub search_directories: Vec<String>,
}

pub fn config_dir() -> Option<PathBuf> {
    match env::var_os("XDG_CONFIG_HOME")
        .and_then(dirs_sys::is_absolute_path)
        .or_else(|| home_dir().map(|h| h.join(".config")))
    {
        Some(mut p) => {
            p.push("swordfish");
            Some(p)
        }
        None => None,
    }
}

pub fn config_filepath() -> Option<PathBuf> {
    config_dir().and_then(|mut dir| {
        dir.push("config.toml");
        Some(dir)
    })
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
            "/System/Applications/Notes.app".to_string(),
            "/Applications".to_string(),
        ]
    }
    #[cfg(target_os = "windows")]
    {
        vec![
            format!("{}\\Desktop", home_path),
            format!("{}\\Downloads", home_path),
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
        let home_path = match home_dir() {
            Some(p) => p,
            None => {
                eprintln!("Could not find the home directory");
                return None;
            }
        };
        match home_path.to_str() {
            Some(path) => Some(
                self.search_directories
                    .iter()
                    .map(|dir| dir.replace('~', path))
                    .collect(),
            ),
            None => {
                eprintln!("Could convert the home directory path to a string!");
                return None;
            }
        }
    }

    pub fn read(&self) -> Self {
        match config_dir() {
            Some(dir) => {
                if let Err(e) = fs::create_dir_all(&dir) {
                    eprintln!("Failed to create config directory: {}", e);
                }
            }
            None => {
                eprintln!("Can't locate the user's home directory");
                return self.clone();
            }
        }
        let config_filepath = config_filepath();
        let config_path = match config_filepath {
            Some(path) => path,
            None => {
                eprintln!("Unable to locate config directory");
                return self.clone();
            }
        };

        let config_file = match std::fs::read_to_string(config_path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    eprintln!("Config file does not exist yet, writing it now.");
                    return self.write();
                } else {
                    eprintln!("Error reading config file {}", e);
                    return self.clone();
                }
            }
        };

        match self.from_toml(config_file) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error decoding config file {}", e);
                return self.clone();
            }
        }
    }

    pub fn write(&self) -> Self {
        match config_dir() {
            Some(dir) => {
                if let Err(e) = fs::create_dir_all(&dir) {
                    eprintln!("Failed to create config directory: {}", e);
                }
            }
            None => {
                eprintln!("Can't locate the user's home directory");
                return self.clone();
            }
        }

        let config_path = match config_filepath() {
            Some(path) => path,
            None => {
                eprintln!("Unable to locate config directory");
                return self.clone();
            }
        };

        let mut config_file = match std::fs::File::create(&config_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!(
                    "Error creating config file at '{:?}'\n  {}",
                    &config_path.to_str(),
                    e
                );
                return self.clone();
            }
        };

        let toml_config = match self.to_toml() {
            Ok(toml_str) => toml_str,
            Err(e) => {
                eprintln!("Error parsing config into a toml string {}", e);
                return self.clone();
            }
        };

        match config_file.write_all(toml_config.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error writing config file {}", e);
            }
        }
        return self.clone();
    }

    pub fn from_json(&self, f: String) -> serde_json::Result<String> {
        serde_json::from_str(&f)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn from_toml(&self, f: String) -> Result<Self, toml::de::Error> {
        let me: Self = toml::from_str(&f)?;
        Ok(me)
    }
}
