use crate::files_setup::get_or_create_data_dir;
use tauri::{path::BaseDirectory, Manager};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const FOLDER_ID: &str = "1lSVZb_C9WxF5yx6dewwcORbx0ACZyVXr";

#[derive(Deserialize)]
struct OAuthCredentials {
    installed: InstalledCreds
}

#[derive(Deserialize)]
struct InstalledCreds {
    client_id: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}

#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

#[derive(Deserialize)]
struct RawGoogleTokens {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    token_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct GoogleTokens {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    token_type: String,
    received_at: u64,
}

#[derive(serde::Deserialize)]
struct DriveFileList {
    files: Vec<DriveFile>,
}

#[derive(serde::Deserialize)]
pub struct DriveFile {
    pub id: String,
    pub name: String,
    #[serde(rename = "modifiedTime")]
    pub modified_time: String,
}


fn read_oauth_credentials(path: &Path) -> Result<InstalledCreds, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let creds: OAuthCredentials = serde_json::from_reader(file).map_err(|e| e.to_string())?;
    Ok(creds.installed)
}

fn get_credentials(handle: &tauri::AppHandle) -> Result<InstalledCreds, String> {
    let credentials_path = handle
        .path()
        .resolve("assets/credential.json", BaseDirectory::Resource)
        .map_err(|e| format!("Erreur chemin credential.json : {}", e))?;

    Ok(read_oauth_credentials(&credentials_path)?)
}

pub fn store_tokens(tokens: &GoogleTokens, handle: &tauri::AppHandle) -> Result<(), String> {
    let path = handle
        .path()
        .resolve("google_tokens.json", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    let content = serde_json::to_string_pretty(tokens).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())?;

    Ok(())
}

fn exchange_code_for_token(code: &str, creds: &InstalledCreds,) -> Result<GoogleTokens, String> {
    let redirect_uri = creds
        .redirect_uris
        .get(0)
        .ok_or("Redirect URI manquant")?;

    let form_data = [
        ("code", code),
        ("client_id", &creds.client_id),
        ("client_secret", &creds.client_secret),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let res = ureq::post("https://oauth2.googleapis.com/token")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .send_form(form_data)
    .map_err(|e| format!("Erreur échange token : {}", e))?;

    let raw: RawGoogleTokens = res
        .into_body()
        .read_json()
        .map_err(|e| format!("Erreur lecture/parsing JSON token : {}", e))?;

    let received_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Erreur temps système : {}", e))?
        .as_secs();

    let tokens = GoogleTokens {
        access_token: raw.access_token,
        refresh_token: raw.refresh_token,
        expires_in: raw.expires_in,
        token_type: raw.token_type,
        received_at,
    };

    Ok(tokens)
}

pub fn get_valid_access_token(handle: &tauri::AppHandle) -> Result<String, String> {
    let token_path: PathBuf = handle
        .path()
        .resolve("google_tokens.json", BaseDirectory::AppData)
        .map_err(|e| format!("Erreur chemin tokens : {}", e))?;

    let creds = get_credentials(handle)?;

    let tokens: GoogleTokens = {
        let content = fs::read_to_string(&token_path)
            .map_err(|e| format!("Erreur lecture fichier tokens : {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Erreur parsing tokens JSON : {}", e))?
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Erreur temps système : {}", e))?
        .as_secs();

    let expires_at = tokens.received_at + tokens.expires_in - 60;
    if now < expires_at { 
        return Ok(tokens.access_token); // Valid token
    }

    // Refreshing token
    let form_data = [
        ("client_id", creds.client_id.as_str()),
        ("client_secret", creds.client_secret.as_str()),
        ("refresh_token", tokens.refresh_token.as_str()),
        ("grant_type", "refresh_token"),
    ];

    let res = ureq::post("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send_form(form_data)
        .map_err(|e| format!("Erreur échange token : {}", e))?;

    let refreshed: RefreshResponse = res
        .into_body()
        .read_json()
        .map_err(|e| format!("Erreur parsing réponse refresh : {}", e))?;

    let new_tokens = GoogleTokens {
        access_token: refreshed.access_token.clone(),
        refresh_token: tokens.refresh_token.clone(),
        expires_in: refreshed.expires_in,
        token_type: refreshed.token_type,
        received_at: now,
    };

    store_tokens(&new_tokens, handle)?;

    Ok(refreshed.access_token)
}

fn list_drive_files(access_token: &str, folder_id: &str) -> Result<Vec<DriveFile>, String> {
    let response = ureq::get("https://www.googleapis.com/drive/v3/files")
        .query("fields", "files(id,name,modifiedTime)")
        .query("q", format!("'{}' in parents", folder_id))
        .query("pageSize", "1000")
        .header("Authorization", &format!("Bearer {}", access_token))
        .call()
        .map_err(|e| format!("Erreur lors de la récupération des fichiers : {}", e))?;

    let file_list: DriveFileList = response
        .into_body()
        .read_json()
        .map_err(|e| format!("Erreur parsing JSON Drive: {}", e))?;

    Ok(file_list.files)
}

fn download_file_from_drive(file: &DriveFile, access_token: &str, destination: &Path) -> Result<(), String> {
    let download_url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?alt=media",
        file.id
    );

    let response = ureq::get(&download_url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .call()
        .map_err(|e| format!("Erreur téléchargement fichier {} : {}", file.name, e))?;

    if response.status() != 200 {
        return Err(format!("Erreur HTTP {} pour {}", response.status(), file.name));
    }

    let mut body = response.into_body();
    let mut reader = body.as_reader();  

    let target_path = destination.join(&file.name);
    let mut output_file = File::create(&target_path)
        .map_err(|e| format!("Erreur création fichier : {}", e))?;

    copy(&mut reader, &mut output_file)
        .map_err(|e| format!("Erreur écriture fichier : {}", e))?;

    Ok(())
}

fn is_remote_newer(local_path: &Path, remote_modified_time: &str) -> bool {
    use chrono::{DateTime, Utc};
    use std::fs;

    let remote_time = DateTime::parse_from_rfc3339(remote_modified_time)
        .unwrap_or_else(|_| Utc::now().into());

    if let Ok(metadata) = fs::metadata(local_path) {
        if let Ok(local_time) = metadata.modified() {
            let local_time: DateTime<Utc> = local_time.into();
            return remote_time > local_time;
        }
    }
    true // Default : take distant file
}

fn is_synced(handle: &tauri::AppHandle) -> Result<bool, String> {
    let token_path = handle
        .path()
        .resolve("google_tokens.json", BaseDirectory::AppData)
        .map_err(|e| format!("Erreur de chemin AppData: {}", e))?;

    Ok(token_path.exists())
}

#[tauri::command]
pub fn is_synced_to_drive(handle: tauri::AppHandle) -> Result<bool, String> {
    Ok(is_synced(&handle)?)
}

#[tauri::command]
pub async fn download_sync_data_from_drive(force: bool, handle: tauri::AppHandle) -> Result<(), String> {
    if !(is_synced(&handle)?) {
        return Err("Synchronisation désactivée, veuillez vous identifier".to_string());
    }

    tauri::async_runtime::spawn_blocking(move || {
        let folder = get_or_create_data_dir(&handle)?;
        let token = get_valid_access_token(&handle)?;
        let files = list_drive_files(&token, &FOLDER_ID)?;

        for file in files {
            let local_path = folder.join(&file.name);

            if force || is_remote_newer(&local_path, &file.modified_time) {
                download_file_from_drive(&file, &token, &folder)?;
            }
        }

        Ok(())
        })
        .await
    .map_err(|e| format!("Erreur tâche bloquante : {}", e))?
}

fn upload_file_to_drive(file_path: &Path, access_token: &str, existing_files: &[DriveFile]) -> Result<(), String> {
    let file_name = file_path.file_name()
        .ok_or("Nom de fichier introuvable")?
        .to_string_lossy();

    let existing_file = existing_files.iter().find(|f| f.name == file_name);

    let file_bytes = fs::read(file_path).map_err(|e| e.to_string())?;

    let response = if let Some(existing) = existing_file {
        // Update existing file
        ureq::patch(&format!(
            "https://www.googleapis.com/upload/drive/v3/files/{}?uploadType=media",
            existing.id
        ))
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Content-Type", "application/octet-stream")
        .send(&file_bytes)
        .map_err(|e| format!("Erreur upload (PATCH): {}", e))?
    } else {
        // Create new file
        let metadata = json!({ "name": file_name, "parents": [FOLDER_ID] });

        let boundary = "boundary123";
        let body = format!(
            "--{0}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{1}\r\n--{0}\r\nContent-Type: application/octet-stream\r\n\r\n",
            boundary,
            metadata.to_string()
        );

        let mut request_body = body.into_bytes();
        request_body.extend(file_bytes);
        request_body.extend(format!("\r\n--{}--", boundary).as_bytes());

        ureq::post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
            .header("Authorization", &format!("Bearer {}", access_token))
            .header("Content-Type", &format!("multipart/related; boundary={}", boundary))
            .send(&request_body)
            .map_err(|e| format!("Erreur upload (POST): {}", e))?
    };

    if response.status() != 200 {
        return Err(format!("Échec upload: {}", response.status()));
    }

    Ok(())
}

#[tauri::command]
pub async fn upload_sync_data_to_drive(handle: tauri::AppHandle) -> Result<(), String> {
    if !(is_synced(&handle)?) {
        return Err("Synchronisation désactivée, veuillez vous identifier".to_string());
    }

    tauri::async_runtime::spawn_blocking(move || {
        let folder_path = get_or_create_data_dir(&handle)?;
        let access_token = get_valid_access_token(&handle)?;

        let existing_files = list_drive_files(&access_token, FOLDER_ID)?;

        for entry in std::fs::read_dir(&folder_path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() {
                upload_file_to_drive(&path, &access_token, &existing_files)?;
            }
        }

        Ok(())
    })
    .await
    .map_err(|e| format!("Erreur tâche bloquante : {}", e))?
}

#[tauri::command]
pub fn get_google_auth_url(handle: tauri::AppHandle) -> Result<String, String> {
    let creds = get_credentials(&handle)?;

    let redirect_uri = creds.redirect_uris.get(0).ok_or("No redirect URI found")?;

    let url = format!(
        "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri={}&scope=https://www.googleapis.com/auth/drive.file&response_type=code&access_type=offline&prompt=consent",
        creds.client_id, redirect_uri
    );

    Ok(url)
}

#[tauri::command]
pub fn save_tokens_from_url(auth_url: String, handle: tauri::AppHandle) -> Result<GoogleTokens, String> {
    let parsed_url = url::Url::parse(&auth_url).map_err(|e| format!("URL invalide: {}", e))?;
    let code = parsed_url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
        .ok_or("Paramètre 'code' introuvable dans l'URL")?;

    let creds = get_credentials(&handle)?;

    let tokens = exchange_code_for_token(&code, &creds)?;

    store_tokens(&tokens, &handle).map_err(|e| format!("Erreur sauvegarde des tokens : {}", e))?;
    
    Ok(tokens)
}