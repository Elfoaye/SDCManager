use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use std::fs;
use std::path::PathBuf;
use serde_json::json;
use crate::files_setup::{get_settings_json, set_settings_json};

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();
    Ok(hash)
}

fn verify_password(password: &str, hashed: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hashed).map_err(|e| e.to_string())?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[tauri::command]
pub fn check_admin_password(password: String, handle: tauri::AppHandle) -> Result<bool, String> {
    let json = get_settings_json(handle).map_err(|e| e.to_string())?;
    let hash_str = json.get("data")
        .and_then(|d| d.get("admin_password_hash"))
        .and_then(|h| h.as_str());
    
    let hashed = if let Some(hash_value) = hash_str { 
        hash_value.to_string()
    } 
    else {
        let default_hash = hash_password("admin123")?;
        json["data"]["admin_password_hash"] = json!(default_hash);
        set_settings_json(&json, handle.clone())?;
        default_hash
    };

    verify_password(password, &hashed)
}

#[tauri::command]
pub fn update_admin_password(old_password: String, new_password: String, handle: tauri::AppHandle) -> Result<(), String> {
    if !check_admin_password(&old_password, handle)? {
        return Err("Mot de passe actuel incorrect".into());
    }

    let mut json = get_settings_json(handle).map_err(|e| e.to_string())?;
    let data = json.get_mut("data").ok_or("Champ dataintrouvable ou invalide")?;
    data["admin_password_hash"] = json!(hash_password(&new_password)?);

    set_settings_json(&json, handle)?;

    Ok(())
}