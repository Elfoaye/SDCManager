use serde::Serialize;
use tauri::{Manager, path::BaseDirectory};
use rusqlite::{Connection, Result};

#[derive(Serialize)]
pub struct Item {
    id: i32,
    nom: String,
    item_type: String,
    total: i32,
    dispo: i32,
    value: f32,
    contrib: f32,
    nb_sorties: i32
}

fn get_database_connection(handle: tauri::AppHandle) -> Result<Connection, String> {
    let path = handle.path()
        .resolve("sync_data/database.db", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let conn = Connection::open(path)
        .map_err(|e| e.to_string())?;

    Ok(conn)
}

#[tauri::command]
pub fn get_materiel_data(handle: tauri::AppHandle) -> Result<Vec<Item>, String> {
    let conn = get_database_connection(handle)?;

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

#[tauri::command]
pub fn update_dispo(value: i32, id: i32, handle: tauri::AppHandle) -> Result<String, String> {
    if value < 0 {
        return Err("Disponible ne peut pas être négatif".to_string());
    }

    let conn = get_database_connection(handle)?;

    let mut verif_request = conn.prepare("SELECT total FROM Materiel WHERE id = ?")
        .map_err(|e| e.to_string())?;
    let total = verif_request.query_row(&[&id], |row| {
        row.get(0)
    }).map_err(|e| e.to_string())?;

    if value > total {
        return Err("Disponible ne peut pas être superieur au total".to_string());
    }

    let mut update_request = conn.prepare("UPDATE Materiel SET dispo = ? WHERE id = ?")
        .map_err(|e| e.to_string())?;

    update_request.execute(&[&value, &id])
        .map_err(|e| e.to_string())?;

    Ok("Valeur modifiée".to_string())
}