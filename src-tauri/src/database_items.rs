use crate::admin_auth::is_admin;
use crate::files_setup::get_or_create_data_dir;
use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub nom: String,
    pub item_type: String,
    pub total: i32,
    pub valeur: f32,
    pub contrib: f32,
    pub nb_sorties: i32,
    pub benef: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SummFactureItem{
    id: i32,
    nom: String,
    date: String,
    quantité: i32,
    durée: i32
}

pub fn get_database_connection(handle: tauri::AppHandle) -> Result<Connection, String> {
    let path = get_or_create_data_dir(&handle)?.join("database.db");
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    Ok(conn)
}

#[tauri::command]
pub fn get_materiel_data(handle: tauri::AppHandle) -> Result<Vec<Item>, String> {
    let conn = get_database_connection(handle)?;

    let mut request = conn
        .prepare("SELECT * FROM Materiel")
        .map_err(|e| e.to_string())?;

    let requested_items = request
        .query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                nom: row.get(1)?,
                item_type: row.get(2)?,
                total: row.get(3)?,
                valeur: row.get(4)?,
                contrib: row.get(5)?,
                nb_sorties: row.get(6)?,
                benef: row.get(7)?,
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
        .query_row(
            "SELECT * FROM Materiel where materiel_id = ?",
            [id],
            |row| {
                Ok(Item {
                    id: row.get(0)?,
                    nom: row.get(1)?,
                    item_type: row.get(2)?,
                    total: row.get(3)?,
                    valeur: row.get(4)?,
                    contrib: row.get(5)?,
                    nb_sorties: row.get(6)?,
                    benef: row.get(7)?,
                })
            },
        )
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
            valeur = ?4,
            contrib = ?5,
            nb_sorties = ?6,
            benef = ?7
        where materiel_id = ?8",
        rusqlite::params![
            item.nom,
            item.item_type,
            item.total,
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
            valeur,
            contrib )
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            item.nom,
            item.item_type,
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

    conn.execute("DELETE FROM Materiel where materiel_id = ?", params![id])
        .map_err(|e| e.to_string())?;

    Ok("Objet supprimé".to_string())
}

#[tauri::command]
pub fn get_item_dispo(
    id: i32,
    devis_id: i32,
    date: String,
    duration: i32,
    handle: tauri::AppHandle,
) -> Result<i32, String> {
    NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Date invalide: {}", e))?;

    let conn = get_database_connection(handle)?;

    let total: i32 = conn
        .query_row(
            "SELECT total FROM Materiel WHERE materiel_id = ?",
            params![id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Erreur lors de la récupération du total: {}", e))?;

    let used: i32 = conn
        .query_row(
            "SELECT COALESCE(SUM(dm.quantité), 0)
        FROM Devis_materiel dm
        JOIN Devis d ON d.devis_id = dm.devis_id
        WHERE dm.d_item_id = ?1
          AND DATE(d.date) <= DATE(?2, '+' || ?3 || ' days')
          AND DATE(d.date, '+' || dm.durée || ' days') > DATE(?2)
          AND dm.etat LIKE '%valide%'
          AND d.devis_id != ?4",
            params![id, date, duration, devis_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Erreur lors de la récupération des réservations: {}", e))?;

    Ok((total - used).max(0))
}

#[tauri::command]
pub fn get_factures_from_item(item_id: i32,  handle: tauri::AppHandle) -> Result<Vec<SummFactureItem>, String> {
    let conn = get_database_connection(handle)?;

    let mut request = conn
        .prepare(
            "SELECT 
                f.facture_id, 
                f.nom, 
                f.date, 
                fm.quantité,
                fm.durée
            FROM Factures f
            JOIN Facture_materiel fm ON fm.facture_id = f.facture_id
            WHERE fm.materiel_id = ?1
            GROUP BY f.facture_id
            ORDER BY f.date DESC"
        )
        .map_err(|e| e.to_string())?;

    let devis_iter = request
        .query_map([item_id], |row| {
            Ok(SummFactureItem {
                id: row.get(0)?,
                nom: row.get(1)?,
                date: row.get(2)?,
                quantité: row.get(3)?,
                durée: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for devis in devis_iter {
        result.push(devis.map_err(|e| e.to_string())?);
    }

    Ok(result)
}