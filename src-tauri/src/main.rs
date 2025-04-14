#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod device_monitor;

use device_monitor::{Device, DeviceMonitor};
use serde_json::json;
use std::sync::Mutex;
use tauri::{
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

struct AppState {
    monitor: Mutex<DeviceMonitor>,
}

#[tauri::command]
async fn add_device(state: State<'_, AppState>, name: String, ip: String) -> Result<(), String> {
    let mut monitor = state.monitor.lock().map_err(|_| "Failed to lock monitor")?;
    monitor.add_device(name, ip);
    Ok(())
}

#[tauri::command]
async fn remove_device(state: State<'_, AppState>, index: usize) -> Result<(), String> {
    let mut monitor = state.monitor.lock().map_err(|_| "Failed to lock monitor")?;
    monitor.remove_device(index);
    Ok(())
}

#[tauri::command]
async fn get_devices(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    let monitor = state.monitor.lock().map_err(|_| "Failed to lock monitor")?;
    Ok(monitor.get_devices().to_vec())
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(AppState {
            monitor: Mutex::new(DeviceMonitor::new()),
        })
        .invoke_handler(tauri::generate_handler![
            add_device,
            remove_device,
            get_devices
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 