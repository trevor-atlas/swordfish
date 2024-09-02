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
use swordfish_utilities::{get_app_icon_cache_path, get_cached_app_icon_path};
use url::Url;

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
