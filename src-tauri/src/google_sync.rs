use std::path::Path;
use serde::Deserialize;
use tauri::{path::BaseDirectory, Manager};

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

fn read_oauth_credentials(path: &Path) -> Result<InstalledCreds, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let creds: OAuthCredentials = serde_json::from_reader(file).map_err(|e| e.to_string())?;
    Ok(creds.installed)
}

#[tauri::command]
pub fn get_google_auth_url(handle: tauri::AppHandle) -> Result<String, String> {
    let credentials_path = handle
        .path()
        .resolve("assets/credential.json", BaseDirectory::Resource)
        .map_err(|e| format!("Erreur chemin credential.json : {}", e))?;

    let creds = read_oauth_credentials(&credentials_path)?;

    let redirect_uri = creds.redirect_uris.get(0).ok_or("No redirect URI found")?;

    let url = format!(
        "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri={}&scope=https://www.googleapis.com/auth/drive.file&response_type=code&access_type=offline&prompt=consent",
        creds.client_id, redirect_uri
    );

    Ok(url)
}

