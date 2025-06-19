use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result, params, OptionalExtension};
use chrono::Local;
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

#[derive(Serialize, Deserialize)]
pub struct Client {
    id: i32,
    nom: String,
    evenement: String,
    adresse: String,
    tel: String,
    mail: String
}

#[derive(Serialize, Deserialize)]
pub struct Devis {
    id: i32,
    client_id: i32,
    nom: String,
    date: String,
    durée: i32,
    nb_tech: i32,
    taux_tech: f32,
    nb_km: i32,
    taux_km: f32,
    adhesion: bool,
    promo: f32,
    etat: String
}

#[derive(Serialize, Deserialize)]
pub struct FullItem {
    item: Item,
    quantité: i32,
    durée: i32,
    etat: String
}

#[derive(Serialize, Deserialize)]
pub struct DevisExtra {
    id: i32,
    devis_id: i32,
    nom: String,
    prix: f32
}

#[derive(Serialize, Deserialize)]
pub struct FullDevis {
    client: Client,
    devis: Devis,
    items: Vec<FullItem>,
    extra: Vec<DevisExtra>
}

#[derive(Serialize, Deserialize)]
pub struct SummDevis {
    id: i32,
    nom: String,
    date: String,
    client_nom: String,
    evenement: String,
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
        .query_row("SELECT * FROM Materiel where materiel_id = ?",
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
        where materiel_id = ?9",
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
        "SELECT total FROM Materiel where materiel_id = ?",
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

    sql.push_str(" where materiel_id = ?");
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
        "DELETE FROM Materiel where materiel_id = ?",
        params![id])
    .map_err(|e| e.to_string())?;

    Ok("Objet supprimé".to_string())
}

fn generate_new_devis_id(conn: &Connection) -> Result<i32, rusqlite::Error> {

    let now = Local::now();
    let current_year = now.format("%Y").to_string();
    let current_month = now.format("%m").to_string();

    let last_devis_id: Option<i32> = conn.query_row(
        "SELECT devis_id FROM Devis ORDER BY devis_id DESC LIMIT 1",
        [],
        |row| row.get(0),
    ).optional()?;

    let new_numero = if let Some(last_id) = last_devis_id {
        let last_id_str = format!("{:08}", last_id);
        let last_year = &last_id_str[0..4];
        let last_month = &last_id_str[4..6];
        let last_numero = &last_id_str[6..8];

        if last_year == current_year && last_month == current_month {
            let last_num: u32 = last_numero.parse().unwrap_or(0);
            last_num + 1
        } else {
            1
        }
    } else {
        1
    };

    let new_id_str = format!("{}{}{:02}", current_year, current_month, new_numero);
    let new_id = new_id_str.parse::<i32>().unwrap();

    Ok(new_id)
}

#[tauri::command]
pub fn save_devis(full_devis: FullDevis, handle: tauri::AppHandle) -> Result<i64, String> {
    let mut conn = get_database_connection(handle)?;

    let devis_exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM Devis WHERE devis_id = ?)",
        [full_devis.devis.id],
        |row| row.get(0)
    ).unwrap_or(false);

    let devis_id = if full_devis.devis.id == 0 || !devis_exists {
        generate_new_devis_id(&conn).map_err(|e| e.to_string())?
    } else {
        full_devis.devis.id
    };

    let transaction = conn.transaction().map_err(|e| e.to_string())?;

    // Client
    transaction.execute(
        "INSERT INTO Client (nom, evenement, adresse, tel, mail) VALUES (?, ?, ?, ?, ?)",
        params![
            full_devis.client.nom,
            full_devis.client.evenement,
            full_devis.client.adresse,
            full_devis.client.tel,
            full_devis.client.mail
        ],
    ).map_err(|e| e.to_string())?;
    let client_id = transaction.last_insert_rowid();

    // Devis
    if devis_exists { // Update devis
        transaction.execute(
            "UPDATE Devis SET client_id = ?, nom = ?, date = ?, durée = ?, nb_tech = ?, taux_tech = ?, nb_km = ?, taux_km = ?, adhesion = ?, promo = ?, etat = ? WHERE devis_id = ?",
            params![
                    client_id,
                    full_devis.devis.nom,
                    full_devis.devis.date,
                    full_devis.devis.durée,
                    full_devis.devis.nb_tech,
                    full_devis.devis.taux_tech,
                    full_devis.devis.nb_km,
                    full_devis.devis.taux_km,
                    full_devis.devis.adhesion,
                    full_devis.devis.promo,
                    full_devis.devis.etat,
                    devis_id
                ],
        ).map_err(|e| e.to_string())?;
        
        // Clean up linked elements
        transaction.execute("DELETE FROM Devis_materiel WHERE devis_id = ?", [devis_id]).map_err(|e| e.to_string())?;
        transaction.execute("DELETE FROM Devis_extra WHERE devis_id = ?", [devis_id]).map_err(|e| e.to_string())?;
    } else { // Insert new devis
        transaction.execute(
            "INSERT INTO Devis (devis_id, client_id, nom, date, durée, adhesion, promo, etat) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                devis_id,
                client_id,
                full_devis.devis.nom,
                full_devis.devis.date,
                full_devis.devis.durée,
                full_devis.devis.adhesion,
                full_devis.devis.promo,
                full_devis.devis.etat
            ],
        ).map_err(|e| e.to_string())?;
    }

    // Materiel
    { // Scope limit for borrowing in transaction.prepare 
        let mut requete_item = transaction.prepare(
            "INSERT INTO Devis_materiel (devis_id, materiel_id, quantité, durée, etat) 
            VALUES (?, ?, ?, ?, ?)"
        ).map_err(|e| e.to_string())?;

        for item in &full_devis.items {
            requete_item.execute(params![
                devis_id,
                item.item.id,
                item.quantité,
                item.durée,
                item.etat
            ]).map_err(|e| e.to_string())?;
        }
    }

    // Extras
    { // Scope limit for borrowing in transaction.prepare 
        let mut requete_extra = transaction.prepare(
            "INSERT INTO Devis_extra (devis_id, nom, prix) VALUES (?, ?, ?)"
        ).map_err(|e| e.to_string())?;

        for extra in &full_devis.extra {
            requete_extra.execute(params![
                devis_id,
                extra.nom,
                extra.prix
            ]).map_err(|e| e.to_string())?;
        }
    }

    transaction.commit().map_err(|e| e.to_string())?;

    Ok(devis_id.into())
}

#[tauri::command]
pub fn load_devis(devis_id: i32, handle: tauri::AppHandle) -> Result<FullDevis, String> {
    let conn = get_database_connection(handle)?;

    let devis: Devis = conn.query_row(
        "SELECT devis_id, client_id, nom, date, durée, nb_tech, taux_tech, nb_km, taux_km, adhesion, promo, etat FROM Devis WHERE devis_id = ?",
        params![devis_id],
        |row| Ok(Devis {
            id: row.get(0)?,
            client_id: row.get(1)?,
            nom: row.get(2)?,
            date: row.get(3)?,
            durée: row.get(4)?,
            nb_tech: row.get(5)?,
            taux_tech: row.get(6)?,
            nb_km: row.get(7)?,
            taux_km: row.get(8)?,
            adhesion: row.get(9)?,
            promo: row.get(10)?,
            etat: row.get(11)?,
        }),
    ).map_err(|e| e.to_string())?;

    let client: Client = conn.query_row(
        "SELECT client_id, nom, evenement, adresse, tel, mail FROM Client WHERE client_id = ?",
        params![devis.client_id],
        |row| Ok(Client {
            id: row.get(0)?,
            nom: row.get(1)?,
            evenement: row.get(2)?,
            adresse: row.get(3)?,
            tel: row.get(4)?,
            mail: row.get(5)?,
        }),
    ).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare("
        SELECT 
            m.materiel_id, m.nom, m.item_type, m.total, m.dispo, m.valeur, m.contrib, m.nb_sorties, m.benef,
            d.quantité, d.durée, d.etat
        FROM Devis_materiel d
        JOIN Materiel m ON d.materiel_id = m.materiel_id
        WHERE d.devis_id = ?
    ").map_err(|e| e.to_string())?;

    let items_iter = stmt.query_map(params![devis_id], |row| {
        Ok(FullItem {
            item: Item {
                id: row.get(0)?,
                nom: row.get(1)?,
                item_type: row.get(2)?,
                total: row.get(3)?,
                dispo: row.get(4)?,
                valeur: row.get(5)?,
                contrib: row.get(6)?,
                nb_sorties: row.get(7)?,
                benef: row.get(8)?,
            },
            quantité: row.get(9)?,
            durée: row.get(10)?,
            etat: row.get(11)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for item_res in items_iter {
        items.push(item_res.map_err(|e| e.to_string())?);
    }

    let mut stmt_extra = conn.prepare(
        "SELECT extra_id, devis_id, nom, prix FROM Devis_extra WHERE devis_id = ?"
    ).map_err(|e| e.to_string())?;
    let extra_iter = stmt_extra.query_map(params![devis_id], |row| {
        Ok(DevisExtra {
            id: row.get(0)?,
            devis_id: row.get(1)?,
            nom: row.get(2)?,
            prix: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut extra = Vec::new();
    for extra_res in extra_iter {
        extra.push(extra_res.map_err(|e| e.to_string())?);
    }

    Ok(FullDevis {
        client,
        devis,
        items,
        extra,
    })
}

#[tauri::command]
pub fn get_devis_summaries(handle: tauri::AppHandle) -> Result<Vec<SummDevis>, String> {
    let conn = get_database_connection(handle)?;

    let mut request = conn.prepare(
        "SELECT 
            d.devis_id, 
            d.nom, 
            d.date, 
            c.nom, 
            c.evenement
         FROM Devis d
         JOIN Client c ON d.client_id = c.client_id"
    ).map_err(|e| e.to_string())?;

    let devis_iter = request
        .query_map([], |row| {
            Ok(SummDevis {
                id: row.get(0)?,
                nom: row.get(1)?,
                date: row.get(2)?,
                client_nom: row.get(3)?,
                evenement: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for devis in devis_iter {
        result.push(devis.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

#[tauri::command]
pub fn get_client_infos(handle: tauri::AppHandle) -> Result<Vec<Client>, String> {
    let conn = get_database_connection(handle)?;

    let mut request = conn.prepare(
        "SELECT * FROM Client"
    ).map_err(|e| e.to_string())?;

    let client_iter = request
        .query_map([], |row| {
            Ok(Client {
                id: row.get(0)?,
                nom: row.get(1)?,
                evenement: row.get(2)?,
                adresse: row.get(3)?,
                tel: row.get(4)?,
                mail: row.get(5)?
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for client in client_iter {
        result.push(client.map_err(|e| e.to_string())?);
    }

    Ok(result)
}