use tauri::{Manager, path::BaseDirectory};
use std::fs;
use std::path::{Path, PathBuf};

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

pub fn get_or_create_data_dir(handle: tauri::AppHandle) -> Result<PathBuf, String> {
    println!("Getting or creating data dir");

    let app_data_dir = handle.path()
        .resolve("SDCManager", BaseDirectory::AppData)
        .map_err(|e| format!("Erreur de chemin AppData: {}", e))?;

    println!("AppData dir resolved to : {}", app_data_dir.display().to_string());

    let default_data_dir = handle.path()
        .resolve("default_data", BaseDirectory::Resource)
        .map_err(|e| format!("Erreur de chemin default_data: {}", e))?;

    println!("DefaultData dir resolved to : {}", default_data_dir.display().to_string());

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Erreur de création du dossier: {}", e))?;

        copy_recursively(&default_data_dir, &app_data_dir)?;
    }

    Ok(app_data_dir)
}