use std::{env, fs, path::PathBuf};

use dirs::{cache_dir, data_dir, home_dir};
use url::Url;

const APP_NAME: &str = "swordfish";

pub fn get_cache_path() -> Option<PathBuf> {
    cache_dir().and_then(|mut dir| {
        dir.push(APP_NAME);
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory: {}", e);
            return None;
        }
        return Some(dir);
    })
}

pub fn get_app_icon_cache_path() -> Option<PathBuf> {
    get_cache_path().and_then(|mut dir| {
        dir.push("app_icons");
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory: {}", e);
            return None;
        }
        return Some(dir);
    })
}

pub fn get_cached_app_icon_path(app_name: &str) -> Option<String> {
    match get_app_icon_cache_path() {
        None => return None,
        Some(mut path) => {
            path.push(format!("{}.png", app_name));
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            } else {
                return None;
            }
        }
    }
}

pub fn get_favicon_cache_path() -> Option<PathBuf> {
    get_cache_path().and_then(|mut dir| {
        dir.push("favicons");
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory: {}", e);
            return None;
        }
        return Some(dir);
    })
}

pub fn get_favicon_path(url: &str) -> Option<String> {
    Url::parse(url).ok().and_then(|url| {
        if let Some(domain) = url.domain() {
            match get_favicon_cache_path() {
                None => return None,
                Some(mut path) => {
                    path.push(format!("{}.png", domain));
                    if path.exists() {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    })
}

pub fn config_dir() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME")
        .and_then(dirs_sys::is_absolute_path)
        .or_else(|| {
            home_dir()
                .and_then(|mut home| {
                    home.push(".config");
                    Some(home)
                })
                .or_else(|| {
                    println!("Failed to get config directory");
                    None
                })
        })
        .and_then(|mut path| {
            path.push(APP_NAME);
            println!("config_dir is {:?}", &path);
            if let Err(e) = fs::create_dir_all(&path) {
                eprintln!("Failed to create directory: {}", e);
                return None;
            }
            Some(path)
        })
}

pub fn config_filepath() -> Option<PathBuf> {
    config_dir().and_then(|mut dir| {
        dir.push("config.toml");
        Some(dir)
    })
}
