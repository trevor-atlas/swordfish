use crate::{
    app_state::AppState,
    windows::{acquire_main_window, hide_main_window, show_main_window},
};
use serde_variant::to_variant_name;
use swordfish_types::SFEvent;
use tauri::{App, GlobalShortcutManager, Manager};
use tracing::{error, info};

pub fn handle_shortcuts(app: &App) {
    let h = app.app_handle();
    let w = acquire_main_window(&h);
    app.state::<AppState>()
        .config
        .lock()
        .map(|config| config.read().launch_shortcut)
        .ok()
        .and_then(|shortcut| {
            let mut gsm = app.global_shortcut_manager();
            gsm.register(shortcut.as_str(), move || {
                if let Ok(bool) = w.is_visible() {
                    if bool {
                        h.emit_all(to_variant_name(&SFEvent::MainWindowHidden).unwrap(), ())
                            .ok();
                        hide_main_window(h.clone());
                    } else {
                        h.emit_all(to_variant_name(&SFEvent::MainWindowShown).unwrap(), ())
                            .ok();
                        show_main_window(h.clone());
                    }
                }
            })
            .ok()
        })
        .and_then(|_| {
            info!("Global shortcuts registered");
            None::<()>
        })
        .or_else(|| {
            error!("Failed to register global shortcut");
            None
        });
}
