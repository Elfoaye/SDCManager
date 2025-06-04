use crate::files_setup::{get_or_create_data_dir};

fn get_settings_json(handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let path = get_or_create_data_dir(handle)?.join("settings.json");

    let file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
    serde_json::from_reader(file).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_materiel_types(handle: tauri::AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let json = get_settings_json(handle).map_err(|e| e.to_string())?;
    let types = json
        .get("data")
        .and_then(|d| d.get("types"))
        .and_then(|t| t.as_array())
        .ok_or("Champ data.types introuvable ou invalide")?;

    Ok(types.clone())
}

#[tauri::command]
pub fn get_loc_formulas(handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let json = get_settings_json(handle).map_err(|e| e.to_string())?;
    let formulas = json
        .get("formula")
        .ok_or("Champ data.types introuvable ou invalide")?;

    Ok(formulas.clone())
}