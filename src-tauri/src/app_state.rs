use crate::{
    settings::AppConfig,
    windows::{acquire_main_window, hide_main_window},
};
use axum::{
    error_handling::HandleErrorLayer, extract::State, handler::Handler, http::StatusCode,
    response::IntoResponse, routing::post, BoxError, Json, Router,
};
use serde_variant::to_variant_name;
use std::{sync::Mutex, time::Duration};
use swordfish_types::{ReceivedEvent, SFEvent};
use tauri::AppHandle;
use tauri::Manager;
use tower::ServiceBuilder;

pub struct AppState {
    pub config: Mutex<AppConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(AppConfig::new()),
        }
    }
}
