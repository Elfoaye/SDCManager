use crate::files_setup::{get_settings_json, set_settings_json};

#[tauri::command]
pub fn get_materiel_types(handle: tauri::AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let json = get_settings_json(&handle).map_err(|e| e.to_string())?;
    let types = json
        .get("data")
        .and_then(|d| d.get("types"))
        .and_then(|t| t.as_array())
        .ok_or("Champ data.types introuvable ou invalide")?;

    Ok(types.clone())
}

#[tauri::command]
pub fn get_loc_formulas(handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let json = get_settings_json(&handle).map_err(|e| e.to_string())?;
    let formulas = json
        .get("formula")
        .ok_or("Champ data.types introuvable ou invalide")?;

    Ok(formulas.clone())
}

#[tauri::command]
pub fn set_materiel_types(new_types: Vec<serde_json::Value>, handle: tauri::AppHandle) -> Result<(), String> {
    let mut json = get_settings_json(&handle).map_err(|e| e.to_string())?;
    
    json["data"]["types"] = serde_json::Value::Array(new_types);

    set_settings_json(&json, &handle).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_loc_formulas(formulas: serde_json::Value, handle: tauri::AppHandle) -> Result<(), String> {
    let mut json = get_settings_json(&handle).map_err(|e| e.to_string())?;

    json["formula"] = formulas;

    set_settings_json(&json, &handle).map_err(|e| e.to_string())
}