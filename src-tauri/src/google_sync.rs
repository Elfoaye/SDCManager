use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
struct OAuthCredentials {
    installed: InstalledCreds
}

#[derive(serde::Deserialize)]
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

fn build_auth_url(client_id: &str, redirect_uri: &str) -> String {
    format!(
        "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri={}&scope=https://www.googleapis.com/auth/drive.file&response_type=code&access_type=offline&prompt=consent",
        client_id, redirect_uri
    )
}