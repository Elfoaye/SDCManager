use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result, params};
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};
use crate::files_setup::{get_or_create_data_dir};
use crate::admin_auth::{is_admin};

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: i32,
    nom: String,
    item_type: String,
    total: i32,
    dispo: i32,
    valeur: f32,
    contrib: f32,
    nb_sorties: i32,
    benef: f32
}

static DB_CONN: OnceCell<Mutex<Connection>> = OnceCell::new();

fn get_database_connection(handle: tauri::AppHandle) -> Result<MutexGuard<'static, Connection>, String> {
    DB_CONN.get_or_try_init(|| {
        let path = get_or_create_data_dir(&handle)?.join("database.db");

        Connection::open(path)
            .map(Mutex::new)
            .map_err(|e| e.to_string())
    })?.lock().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_materiel_data(handle: tauri::AppHandle) -> Result<Vec<Item>, String> {
    let conn = get_database_connection(handle)?;

    let mut request = conn.prepare("SELECT * FROM Materiel")
        .map_err(|e| e.to_string())?;

    let requested_items = request
        .query_map(
            [], 
            |row| {
                Ok(Item {
                    id: row.get(0)?,
                    nom: row.get(1)?,
                    item_type: row.get(2)?,
                    total: row.get(3)?,
                    dispo: row.get(4)?,
                    valeur: row.get(5)?,
                    contrib: row.get(6)?,
                    nb_sorties: row.get(7)?,
                    benef: row.get(8)?
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
pub fn get_item_data(id: i32, handle: tauri::AppHandle) -> Result<Item, String> {
    let conn = get_database_connection(handle)?;

    let item = conn
        .query_row("SELECT * FROM Materiel where id = ?",
            [id], 
            |row| {
                Ok(Item {
                    id: row.get(0)?,
                    nom: row.get(1)?,
                    item_type: row.get(2)?,
                    total: row.get(3)?,
                    dispo: row.get(4)?,
                    valeur: row.get(5)?,
                    contrib: row.get(6)?,
                    nb_sorties: row.get(7)?,
                    benef: row.get(8)?
                })
            })
        .map_err(|e| e.to_string())?;

    Ok(item)
}

#[tauri::command]
pub fn update_item(item: Item, handle: tauri::AppHandle) -> Result<String, String> {
    if !is_admin() {
        return Err("Les droits Admin sont nécessaires pour cette action".to_string());
    }
        
    let conn = get_database_connection(handle)?;

    conn.execute(
        "UPDATE Materiel SET
            nom = ?1,
            item_type = ?2,
            total = ?3,
            dispo = ?4,
            valeur = ?5,
            contrib = ?6,
            nb_sorties = ?7,
            benef = ?8
        WHERE id = ?9",
        rusqlite::params![
            item.nom,
            item.item_type,
            item.total,
            item.dispo,
            item.valeur,
            item.contrib,
            item.nb_sorties,
            item.benef,
            item.id
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok("Objet modifié".to_string())
}


#[tauri::command]
pub fn update_dispo(valeur: i32, old: i32, benef: f32, id: i32, handle: tauri::AppHandle) -> Result<String, String> {
    if valeur < 0 {
        return Err("Disponible ne peut pas être négatif".to_string());
    }

    let conn = get_database_connection(handle)?;

    let total = conn.query_row(
        "SELECT total FROM Materiel WHERE id = ?",
        params![id], 
        |row|  row.get(0)
    ).map_err(|e| e.to_string())?;

    if valeur > total {
        return Err("Disponible ne peut pas être superieur au total".to_string());
    }

    let diff = old - valeur;
    let mut sql = String::from("UPDATE Materiel SET dispo = ?");
    let mut params: Vec<&dyn rusqlite::ToSql> = vec![&valeur];
    
    if diff > 0 {
        sql.push_str(", nb_sorties = nb_sorties + ?");
        params.push(&diff);
    }

    if benef > 0.0 {
        sql.push_str(", benef = benef + ?");
        params.push(&benef);
    }

    sql.push_str(" WHERE id = ?");
    params.push(&id);
    
    conn.execute(&sql, params.as_slice())
        .map_err(|e| e.to_string())?;

    Ok("Valeur modifiée".to_string())
}

#[tauri::command]
pub fn add_item(item: Item, handle: tauri::AppHandle) -> Result<i64, String> {
    if !is_admin() {
        return Err("Les droits Admin sont nécessaires pour cette action".to_string());
    }

    let conn = get_database_connection(handle)?;

    conn.execute(
        "INSERT INTO Materiel (
            nom,
            item_type,
            total,
            dispo,
            valeur,
            contrib )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            item.nom,
            item.item_type,
            item.total,
            item.total,
            item.valeur,
            item.contrib
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    Ok(id)
}

#[tauri::command]
pub fn delete_item(id: i32, handle: tauri::AppHandle) -> Result<String, String> {
    if !is_admin() {
        return Err("Les droits Admin sont nécessaires pour cette action".to_string());
    }
    
    let conn = get_database_connection(handle)?;

    conn.execute(
        "DELETE FROM Materiel WHERE id = ?",
        params![id])
    .map_err(|e| e.to_string())?;

    Ok("Objet supprimé".to_string())
}
