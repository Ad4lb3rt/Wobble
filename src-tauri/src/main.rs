#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Manager;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

#[derive(Clone, serde::Deserialize)]
pub struct Region {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Clone)]
pub struct AppState {
    regions: Arc<Mutex<Vec<Region>>>,
}

#[tauri::command]
fn update_regions(state: tauri::State<AppState>, regions: Vec<Region>) {
    let mut lock = state.regions.lock().unwrap();
    *lock = regions;
}

fn main() {
    let state = AppState {
        regions: Arc::new(Mutex::new(Vec::new())),
    };

    tauri::Builder::default()
        .manage(state.clone())
        .invoke_handler(tauri::generate_handler![update_regions])
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            let regions_clone = state.regions.clone();

            thread::spawn(move || {
                let mut is_ignoring = false; // Assume solid at start
                
                loop {
                    thread::sleep(Duration::from_millis(16)); // ~60fps check

                    let mut point = POINT::default();
                    unsafe {
                        let _ = GetCursorPos(&mut point);
                    }

                    if let Ok(pos) = window.outer_position() {
                        let scale = window.scale_factor().unwrap_or(1.0);
                        
                        // Map global screen cursor to local window coordinates
                        let rel_x = (point.x as f64 - pos.x as f64) / scale;
                        let rel_y = (point.y as f64 - pos.y as f64) / scale;

                        let regions = regions_clone.lock().unwrap();
                        let mut over_element = false;
                        
                        for r in regions.iter() {
                            if rel_x >= r.x && rel_x <= r.x + r.width && 
                               rel_y >= r.y && rel_y <= r.y + r.height {
                                over_element = true;
                                break;
                            }
                        }

                        // Only trigger OS API if the state actually changes
                        if over_element && is_ignoring {
                            let _ = window.set_ignore_cursor_events(false);
                            is_ignoring = false;
                        } else if !over_element && !is_ignoring {
                            let _ = window.set_ignore_cursor_events(true);
                            is_ignoring = true;
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}