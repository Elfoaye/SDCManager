use serde::Serialize;
use rusqlite::{Connection, Result};

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
fn get_data() -> Result<Vec<Item>, String> {
    let conn = Connection::open("../../sync_data/database.db")
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


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
