// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::Manager;

fn main() {
    tauri::Builder::default().setup(|app| {
        let main_window = app.get_window("main").unwrap();
        main_window.get_window(label)
        main_window.with_webview(|webview| {
            #[cfg(target_os = "linux")]
            {
                // see https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/struct.WebView.html
                // and https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/trait.WebViewExt.html
                // use webkit2gtk::traits::WebViewExt;
                // webview.inner().set_zoom_level(4.);
            }

            #[cfg(windows)]
            unsafe {
                // see https://docs.rs/webview2-com/0.19.1/webview2_com/Microsoft/Web/WebView2/Win32/struct.ICoreWebView2Controller.html
                webview.controller().SetZoomFactor(4.).unwrap();
            }

            #[cfg(target_os = "macos")]
            unsafe {
                let () = msg_send![webview.inner(), setPageZoom: 4.];
                let () = msg_send![webview.controller(), removeAllUserScripts];
                let bg_color: cocoa::base::id =
                    msg_send![class!(NSColor), colorWithDeviceRed:0.5 green:0.2 blue:0.4 alpha:1.];
                let () = msg_send![webview.ns_window(), setBackgroundColor: bg_color];
            }
        });
        Ok(())
    });
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// fn main() {
//     let ctx = tauri::generate_context!();

//     let app = tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .setup(move |app| {
//             // let app_handle = app.app_handle();
//             // let startup_win = window::show_startup_window(&app_handle);
//             // register_global_shortcut(&startup_win, &app_handle, &config.user_settings);
//             let _ = get_searchbar(&app);
//             Ok(())
//         })
//         .build(ctx)
//         .expect("Unable to create searchbar window");

//     let window = WindowBuilder::new(&app, "main window".to_string(), WindowUrl::App("/".into()))
//         .menu(get_app_menu())
//         .title("Spyglass")
//         .decorations(false)
//         // .transparent(true)
//         .visible(false)
//         .disable_file_drop_handler()
//         .inner_size(640.0, 108.0)
//         .build()
//         .expect("Unable to create searchbar window");

//     app.run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// pub fn get_app_menu() -> Menu {
//     #[cfg(target_os = "linux")]
//     return Menu::new();

//     #[cfg(not(target_os = "linux"))]
//     Menu::new().add_submenu(Submenu::new(
//         "Spyglass".to_string(),
//         Menu::new()
//             .add_native_item(MenuItem::About("Spyglass".to_string(), Default::default()))
//             // Currently we need to include these so that the shortcuts for these
//             // actions work.
//             .add_native_item(MenuItem::Copy)
//             .add_native_item(MenuItem::Paste)
//             .add_native_item(MenuItem::SelectAll)
//             .add_native_item(MenuItem::Separator)
//             .add_native_item(MenuItem::Hide)
//             .add_native_item(MenuItem::Quit),
//     ))
// }

// pub fn show_search_bar(window: &Window) {
//     platform::show_search_bar(window);

//     // Wait a little bit for the window to show being focusing on it.
//     let window = window.clone();
//     tauri::async_runtime::spawn(async move {
//         tokio::time::sleep(tokio::time::Duration::from_millis(256)).await;
//         let _ = window.emit(ClientEvent::FocusWindow.as_ref(), true);
//     });
// }

// pub fn get_searchbar(app: &AppHandle) -> Window {
//     if let Some(window) = app.get_window(constants::SEARCH_WIN_NAME) {
//         window
//     } else {
//         let window =
//             WindowBuilder::new(app, constants::SEARCH_WIN_NAME, WindowUrl::App("/".into()))
//                 .menu(get_app_menu())
//                 .title("Spyglass")
//                 .decorations(false)
//                 .transparent(true)
//                 .visible(false)
//                 .disable_file_drop_handler()
//                 .inner_size(640.0, 108.0)
//                 .build()
//                 .expect("Unable to create searchbar window");

//         // macOS: Handle multiple spaces correctly
//         #[cfg(target_os = "macos")]
//         {
//             use cocoa::appkit::NSWindow;
//             unsafe {
//                 let ns_window =
//                     window.ns_window().expect("Unable to get ns_window") as cocoa::base::id;
//                 ns_window.setCollectionBehavior_(cocoa::appkit::NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);
//             }
//         }

//         window
//     }
// }
