
use std::{fs, thread::sleep, time::Duration};
use std::path::Path;
use std::process::{Command, Stdio};
use ureq::{Body, get, post};
use serde_json::json;
use tauri::Manager;

const API_URL: &str = "http://localhost:8384";
const API_KEY: &str = "syncthing";

pub fn launch_syncthing(handle: &tauri::AppHandle) -> Result<(), String> {
    let os_name = std::env::consts::OS;
    let binary_name = if os_name == "windows" {
        "syncthing.exe"
    } else {
        "syncthing"
    };

    let relative_path = format!("bin/{}/{}", os_name, binary_name);

    let syncthing_path = handle
        .path()
        .resolve(relative_path, tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let config_dir = handle
        .path()
        .resolve("syncthing_config", tauri::path::BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    Command::new(&syncthing_path)
        .arg("-home")
        .arg(&config_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Erreur de lancement Syncthing : {}", e))?;

    Ok(())
}

pub fn wait_for_syncthing_api() -> Result<(), String> {
    for _ in 0..30 {
        match get(&format!("{}/rest/system/ping", API_URL)).call() {
            Ok(resp) if resp.status() == 200 => return Ok(()),
            _ => sleep(Duration::from_secs(1)),
        }
    }
    Err("Syncthing API non disponible après 30 secondes".into())
}

// pub fn configure_syncthing_folder(path: &Path, folder_id: &str) -> Result<(), String> {
//     let url = format!("{}/rest/config/folders", API_URL);

//     let body = json!({
//         "id": folder_id,
//         "label": "SDC Data",
//         "path": path.to_str().ok_or("Chemin invalide")?,
//         "type": "sendreceive",
//         "devices": ["self"],
//         "rescanIntervalS": 60,
//         "fsWatcherEnabled": true,
//         "fsWatcherDelayS": 10
//     });

//     let response = post(&url)
//             .header("X-API-Key", API_KEY)
//             .header("Content-Type", "application/json")
//             .send_string(&body.to_string());

//     match response {
//         Ok(_) => Ok(()),
//         Err(e) => Err(format!("Erreur configuration dossier Syncthing : {}", e)),
//     }
// }

pub fn restart_syncthing() -> Result<(), String> {
    Ok(())
}