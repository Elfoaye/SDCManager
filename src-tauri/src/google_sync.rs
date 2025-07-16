use crate::files_setup::get_or_create_data_dir;
use tauri::{path::BaseDirectory, Manager};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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

fn upload_file_to_drive(file_path: &Path, access_token: &str) -> Result<(), String> {
    let file_name = file_path.file_name()
        .ok_or("Nom de fichier introuvable")?
        .to_string_lossy();

    let metadata = json!({"name": file_name});

    let boundary = "boundary123";
    let body = format!(
        "--{0}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{1}\r\n--{0}\r\nContent-Type: application/octet-stream\r\n\r\n",
        boundary,
        metadata.to_string()
    );

    let file_bytes = fs::read(file_path).map_err(|e| e.to_string())?;

    let mut request_body = body.into_bytes();
    request_body.extend(file_bytes);
    request_body.extend(format!("\r\n--{}--", boundary).as_bytes());


    let response = ureq::post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Content-Type", &format!("multipart/related; boundary={}", boundary))
        .send(&request_body)
        .map_err(|e| format!("Erreur upload: {}", e))?;

    if response.status() != 200 {
        return Err(format!("Échec upload: {}", response.status()));
    }

    Ok(())
}

#[tauri::command]
pub fn upload_sync_data_to_drive(handle: tauri::AppHandle) -> Result<(), String> {
    let folder_path = get_or_create_data_dir(&handle)?;
    let access_token = get_valid_access_token(&handle)?;

    for entry in std::fs::read_dir(&folder_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() {
            upload_file_to_drive(&path, &access_token)?;
        }
    }

    Ok(())
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