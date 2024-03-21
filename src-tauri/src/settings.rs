use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub launch_shortcut: Setting<String>,
    pub search_directories: Setting<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SettingType {
    Boolean,
    String,
    Enum,
    List,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting<T> {
    pub value: T,
    pub r#type: SettingType,
    pub description: String,
}

impl Setting<String> {
    pub fn new_string(value: &str, r#type: SettingType, description: &str) -> Self {
        Self {
            value: value.to_string(),
            r#type,
            description: description.to_string(),
        }
    }
}

impl<T> Setting<T> {
    pub fn new(value: T, r#type: SettingType, description: &str) -> Self {
        Self {
            value,
            r#type,
            description: description.to_string(),
        }
    }
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

impl AppConfig {
    pub fn new() -> Self {
        let home_path = home_dir().expect("couldn't find the home dir!");
        let home_path = home_path
            .to_str()
            .expect("Could convert the home directory path to a string!");
        #[cfg(target_os = "macos")]
        {
            let default_search_directories: Vec<String> = vec![
                format!("{}/Desktop", home_path),
                format!("{}/Downloads", home_path),
                format!("{}/Applications", home_path),
                format!("{}/Documents", home_path),
                format!("{}/Movies", home_path),
                format!("{}/Music", home_path),
            ];
            Self {
                launch_shortcut: Setting::new_string(
                    "Control+Space",
                    SettingType::String,
                    "What shortcut should the primary app window trigger with",
                ),
                search_directories: Setting::new(
                    default_search_directories,
                    SettingType::List,
                    "What directories should we search for files in?",
                ),
            }
            .read()
        }
        #[cfg(target_os = "windows")]
        {
            let default_search_directories: Vec<String> = vec![
                format!("{}\\Desktop", home_path),
                format!("{}\\Downloads", home_path),
                format!("{}\\Pictures", home_path),
                format!("{}\\Videos", home_path),
            ];

            Self {
                launch_shortcut: Setting::new_string(
                    "Control+Space",
                    SettingType::String,
                    "What shortcut should the primary app window trigger with",
                ),
                search_directories: Setting::new(
                    default_search_directories,
                    SettingType::List,
                    "What directories should we search for files in?",
                ),
            }
            .read()
        }
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
                    .value
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

        let config_file = match std::fs::File::open(config_path) {
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

        match serde_json::from_reader(config_file) {
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

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }
}
