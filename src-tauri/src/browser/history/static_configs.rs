pub fn arc_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/Arc/User Data/Default/History".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some("{}\\AppData\\Local\\Arc\\User Data\\Default\\History".into())
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.config/Arc/User Data/Default/History".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn chrome_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/Google/Chrome/Default/History".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some("{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\History".into())
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.config/google-chrome/Default/History".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn firefox_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/Firefox/Profiles/**/places.sqlite".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some(
            "{}\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\*.default-release\\places.sqlite"
                .into(),
        )
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.mozilla/firefox/*.default-release/places.sqlite".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn safari_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Safari/History.db".into())
    }

    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

pub fn opera_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/com.operasoftware.Opera/History".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some("{}\\AppData\\Roaming\\Opera Software\\Opera Stable\\History".into())
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.config/opera/History".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn brave_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/BraveSoftware/Brave-Browser/Default/History".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some("{}\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default\\History".into())
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.config/BraveSoftware/Brave-Browser/Default/History".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn edge_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/Microsoft Edge/Default/History".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some("{}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\History".into())
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.config/microsoft-edge/Default/History".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn vivaldi_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/Vivaldi/Default/History".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some("{}\\AppData\\Local\\Vivaldi\\User Data\\Default\\History".into())
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.config/vivaldi/Default/History".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn chromium_path() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        Some("{}/Library/Application Support/Chromium/Default/History".into())
    }

    #[cfg(target_os = "windows")]
    {
        Some("{}\\AppData\\Local\\Chromium\\User Data\\Default\\History".into())
    }

    #[cfg(target_os = "linux")]
    {
        Some("{}/.config/chromium/Default/History".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}
