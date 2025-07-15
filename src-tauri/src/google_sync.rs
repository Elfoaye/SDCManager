use tauri::{path::BaseDirectory, Manager};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

#[derive(Deserialize, Serialize)]
pub struct GoogleTokens {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    token_type: String,
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

    let tokens: GoogleTokens = res
        .into_body()
        .read_json()
        .map_err(|e| format!("Erreur lecture/parsing JSON token : {}", e))?;

    Ok(tokens)
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
pub fn exchange_code_from_url(auth_url: String, handle: tauri::AppHandle) -> Result<GoogleTokens, String> {
    let parsed_url = url::Url::parse(&auth_url).map_err(|e| format!("URL invalide: {}", e))?;
    let code = parsed_url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
        .ok_or("Paramètre 'code' introuvable dans l'URL")?;

    let creds = get_credentials(&handle)?;

    let tokens = exchange_code_for_token(&code, &creds)?;

    Ok(tokens)
}