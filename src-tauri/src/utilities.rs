use dirs::{cache_dir, data_dir, home_dir};
use glob::glob;
use icns::{IconFamily, IconType};
use plist::Value;
use std::{
    env,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
    thread,
};
use url::Url;

const APP_NAME: &str = "swordfish";

pub fn get_data_path() -> Option<PathBuf> {
    data_dir().and_then(|mut dir| {
        dir.push(APP_NAME);
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory: {}", e);
            return None;
        }
        Some(dir)
    })
}

pub fn get_cache_path() -> Option<PathBuf> {
    cache_dir().and_then(|mut dir| {
        dir.push(APP_NAME);
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory: {}", e);
            return None;
        }
        Some(dir)
    })
}

pub fn get_app_icon_cache_path() -> Option<PathBuf> {
    get_cache_path().and_then(|mut dir| {
        dir.push("app_icons");
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory: {}", e);
            return None;
        }
        Some(dir)
    })
}

pub fn get_cached_app_icon_path(app_name: &str) -> Option<String> {
    match get_app_icon_cache_path() {
        None => None,
        Some(mut path) => {
            path.push(format!("{}.png", app_name));
            if path.exists() {
                Some(path.to_string_lossy().to_string())
            } else {
                None
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
        Some(dir)
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
            if let Err(e) = fs::create_dir_all(&path) {
                eprintln!("Failed to create directory: {}", e);
                return None;
            }
            Some(path)
        })
}

pub fn config_filepath() -> Option<PathBuf> {
    config_dir().and_then(|mut dir| {
        dir.push("config.json");
        Some(dir)
    })
}

pub fn cache_all_app_icons() {
    use rayon::prelude::*;
    thread::spawn(move || {
        glob("/Applications/**/*.app")
            .and_then(|paths| Ok(paths.filter_map(Result::ok).collect()))
            .and_then(|app_bundle_paths: Vec<PathBuf>| {
                glob("/System/Applications/**/*.app")
                    .and_then(|paths| Ok(paths.into_iter().filter_map(Result::ok).collect()))
                    .and_then(|system_app_bundle_paths: Vec<PathBuf>| {
                        Ok(system_app_bundle_paths
                            .iter()
                            .chain(app_bundle_paths.iter())
                            .map(|p| p.to_owned())
                            .collect::<Vec<PathBuf>>())
                    })
            })
            .into_par_iter()
            .flatten()
            .for_each(|file| {
                file.to_str().and_then(|path| {
                    file.file_stem()
                        .and_then(|stem| stem.to_str())
                        .and_then(|file_stem| cache_app_icon_path(path, file_stem))
                });
            });
    });
}

pub fn cache_app_icon_path(app_bundle_path: &str, app_name: &str) -> Option<PathBuf> {
    if let Some(icon_path) = get_cached_app_icon_path(&app_name) {
        let path = PathBuf::from(&icon_path);
        if path.exists() {
            if let Ok(metadata) = path.clone().metadata() {
                if metadata.len() == 0 {
                    std::fs::remove_file(path.clone()).ok().unwrap_or(());
                    return None;
                }
            }
        }
        return Some(PathBuf::from(icon_path));
    }

    let mut app_bundle_path = PathBuf::from(app_bundle_path);
    app_bundle_path.push("Contents");
    let mut plist_path = app_bundle_path.clone();
    plist_path.push("Info.plist");

    let info_plist = Value::from_file(plist_path).ok()?;

    let icon_file_name = info_plist
        .as_dictionary()?
        .get("CFBundleIconFile")
        .and_then(Value::as_string)?;
    // macOS does not require the extension for .icns files in the Info.plist.
    // Ensure it has the .icns extension.
    let icon_file_name = if icon_file_name.ends_with(".icns") {
        icon_file_name.to_string()
    } else {
        format!("{}.icns", icon_file_name)
    };

    // Construct the path to the icon file within the .app bundle
    let icon_path = app_bundle_path
        .join("Resources")
        .join(icon_file_name.clone());

    get_app_icon_cache_path()
        .and_then(|mut dir| {
            dir.push(format!("{}.png", app_name));
            Some(dir)
        })
        .map(|cache_path| {
            File::open(icon_path.clone())
                .ok()
                .and_then(|f| Some(BufReader::new(f)))
                .and_then(|file| {
                    let icon_family = IconFamily::read(file).ok()?;
                    let file = BufWriter::new(File::create(cache_path.clone()).unwrap());
                    icon_family
                        .get_icon_with_type(IconType::RGBA32_512x512_2x)
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_512x512))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_256x256_2x))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_256x256))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_128x128_2x))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_128x128))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_64x64))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_32x32_2x))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGBA32_32x32))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGB24_128x128))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::RGB24_48x48))
                        .or_else(|_| icon_family.get_icon_with_type(IconType::Mask8_128x128))
                        .ok()
                        .and_then(|i| {
                            if i.data().is_empty() {
                                println!("Icon data is empty {}", icon_path.to_str().unwrap());
                                return None;
                            } else {
                                i.write_png(file).ok();
                            }
                            Some(cache_path)
                        })
                })
        })
        .flatten()
}
