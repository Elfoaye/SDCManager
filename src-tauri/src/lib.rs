use serde::Serialize;
use rusqlite::{Connection, Result};
use tauri::{Manager, path::BaseDirectory};

#[derive(Serialize)]
struct Item {
    id: i32,
    nom: String,
    item_type: String,
    total: i32,
    dispo: i32,
    value: f32,
    contrib: f32,
    nb_sorties: i32
}

#[tauri::command]
fn get_materiel_data(handle: tauri::AppHandle) -> Result<Vec<Item>, String> {
    let path = handle.path()
        .resolve("sync_data/database.db", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let conn = Connection::open(path)
        .map_err(|e| e.to_string())?;

    let mut request = conn.prepare("SELECT * FROM Materiel")
        .map_err(|e| e.to_string())?;

    let requested_items = request
        .query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                nom: row.get(1)?,
                item_type: row.get(2)?,
                total: row.get(3)?,
                dispo: row.get(4)?,
                value: row.get(5)?,
                contrib: row.get(6)?,
                nb_sorties: row.get(7)?
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for item_result in requested_items {
        items.push(item_result.map_err(|e| e.to_string())?);
    }

    Ok(items)
}

fn get_settings_json(handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let path = handle.path()
        .resolve("sync_data/settings.json", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
    serde_json::from_reader(file).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_materiel_types(handle: tauri::AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let json = get_settings_json(handle).map_err(|e| e.to_string())?;
    let types = json
        .get("data")
        .and_then(|d| d.get("types"))
        .and_then(|t| t.as_array())
        .ok_or("Champ data.types introuvable ou invalide")?;

    Ok(types.clone())
}

#[tauri::command]
fn get_loc_formulas(handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let json = get_settings_json(handle).map_err(|e| e.to_string())?;
    let formulas = json
        .get("formula")
        .ok_or("Champ data.types introuvable ou invalide")?;

    Ok(formulas.clone())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler!
            [get_materiel_data, 
            get_materiel_types,
            get_loc_formulas])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
