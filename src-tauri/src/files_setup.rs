use std::fs;
use std::path::{Path, PathBuf};
use tauri::{path::BaseDirectory, Manager};
use base64::{engine::general_purpose, Engine as _};

fn copy_recursively(src: &Path, dst: &Path) -> Result<(), String> {
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_type = entry.file_type().map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            fs::create_dir_all(&dst_path).map_err(|e| e.to_string())?;
            copy_recursively(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn get_or_create_data_dir(handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = handle
        .path()
        .resolve("SDCManager", BaseDirectory::AppData)
        .map_err(|e| format!("Erreur de chemin AppData: {}", e))?;

    let default_data_dir = handle
        .path()
        .resolve("default_data", BaseDirectory::Resource)
        .map_err(|e| format!("Erreur de chemin default_data: {}", e))?;

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Erreur de création du dossier: {}", e))?;

        copy_recursively(&default_data_dir, &app_data_dir)?;
    }

    Ok(app_data_dir)
}

pub fn get_settings_json(handle: &tauri::AppHandle) -> Result<serde_json::Value, String> {
    let path = get_or_create_data_dir(&handle)?.join("settings.sdcdata");

    let encoded = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let decoded_bytes = general_purpose::STANDARD.decode(&encoded).map_err(|e| e.to_string())?;

    serde_json::from_slice(&decoded_bytes).map_err(|e| e.to_string())
}

pub fn set_settings_json(value: &serde_json::Value, handle: &tauri::AppHandle) -> Result<(), String> {
    let path = get_or_create_data_dir(&handle)?.join("settings.sdcdata");

    let json_bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    let encoded = general_purpose::STANDARD.encode(&json_bytes);

    std::fs::write(&path, encoded).map_err(|e| e.to_string())
}